use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use flowtrace_core::state::Status;
use fs2::FileExt;
use git2::{ObjectType, Repository, RepositoryOpenFlags, Signature};
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

/// Process-wide lock around any write to the underlying git repo.
///
/// Why: under the single-repo layout, multiple trace folders share one outer
/// `.git`. Two concurrent `trace` processes will both: (a) write `index.lock`
/// at the same time (libgit2 returns `Locked`), and (b) race HEAD between
/// `revparse` and `commit` (libgit2 returns "current tip is not the first
/// parent"). libgit2's per-process index lock alone doesn't serialize across
/// processes, so we hold an OS-level file lock on `.git/trace-commit.lock`
/// for the whole read-modify-commit sequence.
///
/// Acquires an exclusive lock, then drops it when the returned guard goes
/// out of scope. Idempotent: re-entering from the same process serializes
/// via the OS, which is what we want.
struct CommitLock {
    _file: std::fs::File,
}

impl CommitLock {
    fn acquire(repo: &Repository) -> Result<Self> {
        let git_dir = repo.path(); // always points at `.git/` (or bare repo dir)
        let lock_path = git_dir.join("trace-commit.lock");
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&lock_path)
            .with_context(|| format!("opening commit lock {}", lock_path.display()))?;
        file.lock_exclusive()
            .with_context(|| format!("acquiring commit lock {}", lock_path.display()))?;
        Ok(Self { _file: file })
    }
}

impl Drop for CommitLock {
    fn drop(&mut self) {
        // BSD/Linux flock auto-releases on close, but be explicit.
        let _ = fs2::FileExt::unlock(&self._file);
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CommitInfo {
    pub sha: String,
    pub at: DateTime<Utc>,
    pub message: String,
}

const AUTHOR_NAME: &str = "trace";
const AUTHOR_EMAIL: &str = "trace@local";

/// Open the repo that owns `root`. Walks up from `root` so a trace folder
/// nested inside a parent repo resolves to that parent (single-repo layout).
/// When `root` itself is a repo, opens that.
fn open(root: &Path) -> Result<Repository> {
    Repository::open_ext(root, RepositoryOpenFlags::empty(), &[] as &[&Path])
        .with_context(|| format!("opening git repo from {}", root.display()))
}

/// Open the containing repo, or initialize one at `root` if no ancestor is a repo.
fn open_or_init(root: &Path) -> Result<Repository> {
    open(root).or_else(|_| {
        Repository::init(root).with_context(|| format!("git init at {}", root.display()))
    })
}

/// Path inside the repo's working tree that points at `root`. Used to scope
/// `add_all` pathspec and to translate caller-supplied "trace-relative"
/// paths into "repo-relative" paths for commits_under / show_at / etc.
/// Returns `None` if `root` IS the repo workdir (i.e. trace == repo, e.g. tests).
fn repo_subpath(repo: &Repository, root: &Path) -> Option<PathBuf> {
    let workdir = repo.workdir()?;
    let workdir = workdir.canonicalize().ok()?;
    let root = root.canonicalize().ok()?;
    if root == workdir {
        return None;
    }
    root.strip_prefix(&workdir).ok().map(|p| p.to_path_buf())
}

/// Prefix a trace-relative path with the subpath portion, if any. Used when
/// looking up paths via libgit2 (commits_under, show_at) — git always wants
/// repo-relative paths even though the caller speaks trace-relative.
fn translate(repo: &Repository, root: &Path, rel: &str) -> String {
    match repo_subpath(repo, root) {
        Some(sub) => sub.join(rel).to_string_lossy().replace('\\', "/"),
        None => rel.to_string(),
    }
}

fn signature() -> Result<Signature<'static>> {
    Signature::now(AUTHOR_NAME, AUTHOR_EMAIL).context("building git signature")
}

/// `git init` in `root`. Idempotent — re-init on an existing repo is a no-op.
pub fn init(root: &Path) -> Result<()> {
    let _ = open_or_init(root)?;
    Ok(())
}

/// Stage exactly the listed trace-relative paths and commit. Replaces the
/// `add -A` sweep with an explicit declaration of what this commit contains.
/// Caller is responsible for ensuring every path exists on disk.
pub fn commit_files(root: &Path, msg: &str, files: &[&str]) -> Result<()> {
    let repo = open_or_init(root)?;
    // Serialize cross-process writes to this repo. Held until function exits.
    let _lock = CommitLock::acquire(&repo)?;

    let translate_one = |rel: &str| -> String {
        match repo_subpath(&repo, root) {
            Some(sub) => sub.join(rel).to_string_lossy().replace('\\', "/"),
            None => rel.to_string(),
        }
    };

    let translated_files: Vec<String> = files.iter().map(|f| translate_one(f)).collect();

    // Build the commit tree from HEAD instead of the repository's live index.
    // A trace can intentionally share an ancestor repository, and that index
    // may contain unrelated user work which must stay staged and uncommitted.
    let head_commit = repo.head().ok().and_then(|head| head.peel_to_commit().ok());
    // `Index::new` cannot stat worktree paths because it has no repository
    // backing. Keep a copy of the live entries, clear this repository index in
    // memory, and do not write it until the commit tree has been constructed.
    let mut index = repo.index().context("reading git index")?;
    let saved_entries: Vec<_> = index.iter().collect();
    index.clear().context("clearing isolated git index")?;
    if let Some(parent) = head_commit.as_ref() {
        let tree = parent.tree().context("reading HEAD tree")?;
        index
            .read_tree(&tree)
            .context("loading HEAD into isolated git index")?;
    }
    for translated in &translated_files {
        index
            .add_path(Path::new(translated))
            .with_context(|| format!("staging {}", translated))?;
    }

    let tree_oid = index.write_tree().context("writing tree from index")?;

    // Keep the caller's other staged entries intact while refreshing only the
    // paths owned by this commit, matching the old post-commit index state.
    index.clear().context("restoring live git index")?;
    for entry in saved_entries {
        index.add(&entry).context("restoring staged index entry")?;
    }
    for translated in &translated_files {
        index
            .add_path(Path::new(translated))
            .with_context(|| format!("updating staged {}", translated))?;
    }
    index.write().context("writing git index")?;

    // No-op guard: tree unchanged vs HEAD.
    if let Some(parent_commit) = head_commit.as_ref() {
        if parent_commit.tree_id() == tree_oid {
            return Ok(());
        }
    }

    let tree = repo.find_tree(tree_oid)?;
    let sig = signature()?;
    let parent_refs: Vec<&git2::Commit> = head_commit.iter().collect();
    repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &parent_refs)
        .context("git commit")?;
    Ok(())
}

pub async fn commit_files_async(root: PathBuf, msg: String, files: Vec<String>) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        let refs: Vec<&str> = files.iter().map(String::as_str).collect();
        commit_files(&root, &msg, &refs)
    })
    .await
    .context("git task panicked")?
}

pub async fn init_async(root: PathBuf) -> Result<()> {
    tokio::task::spawn_blocking(move || init(&root))
        .await
        .context("git task panicked")?
}

/// SHA of HEAD. Errors if the repo has no commits yet.
pub fn head_sha(root: &Path) -> Result<String> {
    let repo = open(root)?;
    let head = repo.head().context("repo has no HEAD")?;
    let commit = head.peel_to_commit()?;
    Ok(commit.id().to_string())
}

/// Commits whose tree changed `rel_path` (file or directory), newest first.
/// `rel_path` is trace-relative; resolves to repo-relative internally so
/// single-repo and per-trace layouts both work.
pub fn commits_under(root: &Path, rel_path: &str) -> Result<Vec<CommitInfo>> {
    let repo = open(root)?;
    let translated = translate(&repo, root, rel_path);
    let mut walk = repo.revwalk()?;
    walk.push_head().context("revwalk push HEAD")?;
    walk.set_sorting(git2::Sort::TIME)?;

    let target_path = Path::new(&translated);
    let mut out = Vec::new();
    for oid in walk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let commit_tree = commit.tree()?;
        let touched = if commit.parent_count() == 0 {
            tree_contains_path(&commit_tree, target_path)?
        } else {
            let mut found = false;
            for i in 0..commit.parent_count() {
                let parent = commit.parent(i)?;
                let parent_tree = parent.tree()?;
                let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), None)?;
                for delta in diff.deltas() {
                    if path_matches(delta.new_file().path(), target_path)
                        || path_matches(delta.old_file().path(), target_path)
                    {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            found
        };
        if touched {
            let time = commit.time();
            let at = Utc.timestamp_opt(time.seconds(), 0).single().unwrap_or_else(Utc::now);
            out.push(CommitInfo {
                sha: oid.to_string(),
                at,
                message: commit.message().unwrap_or("").trim_end().to_string(),
            });
        }
    }
    Ok(out)
}

fn path_matches(p: Option<&Path>, target: &Path) -> bool {
    let Some(p) = p else { return false };
    p == target || p.starts_with(target)
}

fn tree_contains_path(tree: &git2::Tree, target: &Path) -> Result<bool> {
    let mut found = false;
    tree.walk(git2::TreeWalkMode::PreOrder, |dir, entry| {
        let mut p = PathBuf::from(dir);
        if let Some(name) = entry.name() {
            p.push(name);
        }
        if p == target || p.starts_with(target) {
            found = true;
            return git2::TreeWalkResult::Abort;
        }
        git2::TreeWalkResult::Ok
    })?;
    Ok(found)
}

/// Locate a subtree by trace-relative path inside the tree at `sha` and run
/// `f` against it. Returns `Ok(missing)` when the path isn't there (or isn't
/// a tree) — callers pick a sensible default for that case.
fn with_subtree_at<R>(
    root: &Path,
    sha: &str,
    rel: &str,
    missing: R,
    f: impl FnOnce(&Repository, &git2::Tree<'_>) -> Result<R>,
) -> Result<R> {
    let repo = open(root)?;
    let translated = translate(&repo, root, rel);
    let oid = git2::Oid::from_str(sha).context("invalid commit sha")?;
    let commit = repo
        .find_commit(oid)
        .with_context(|| format!("commit {} not found", sha))?;
    let tree = commit.tree()?;
    let entry = match tree.get_path(Path::new(&translated)) {
        Ok(e) => e,
        Err(_) => return Ok(missing),
    };
    let object = entry.to_object(&repo)?;
    let Some(subtree) = object.as_tree() else {
        return Ok(missing);
    };
    f(&repo, subtree)
}

/// For every leaf file under `dir_rel` in the tree at `sha`, return
/// `(filename, blob bytes)`. Reads are batched against a single repo open;
/// intended for "render this run's reply stream as of commit X". Returns
/// `Ok(vec![])` if the directory doesn't exist at that commit.
pub fn list_dir_blobs_at(
    root: &Path,
    sha: &str,
    dir_rel: &str,
) -> Result<Vec<(String, Vec<u8>)>> {
    with_subtree_at(root, sha, dir_rel, vec![], |repo, dir_tree| {
        let mut out = Vec::with_capacity(dir_tree.len());
        for e in dir_tree.iter() {
            let Some(name) = e.name() else { continue };
            let blob = repo.find_blob(e.id())?;
            out.push((name.to_string(), blob.content().to_vec()));
        }
        Ok(out)
    })
}

/// Walk the tree at `runs/<run_id>/` at `sha` and return per-step committed
/// files as `{step_id: [<file>, ...]}`. Files at `<step>/sub/x.png` flatten
/// to `"sub/x.png"`. Skips `state.json` and the append-only `replies/` dir.
/// Empty map when the run dir isn't in the tree at this commit.
pub fn list_run_step_files_at(
    root: &Path,
    sha: &str,
    run_id: &str,
) -> Result<std::collections::HashMap<String, Vec<String>>> {
    with_subtree_at(
        root,
        sha,
        &format!("runs/{}", run_id),
        Default::default(),
        |_, run_tree| {
            // Single pre-order walk over the whole run tree, grouping blobs by
            // their first path component (= step id). One pass instead of
            // N-step-subtree walks.
            let mut out: std::collections::HashMap<String, Vec<String>> = Default::default();
            run_tree.walk(git2::TreeWalkMode::PreOrder, |dir, entry| {
                if entry.kind() != Some(git2::ObjectType::Blob) {
                    return git2::TreeWalkResult::Ok;
                }
                // `dir` is empty at the run root, otherwise ends with '/'.
                // Top-level blobs under the run dir (e.g. state.json) are skipped.
                if dir.is_empty() {
                    return git2::TreeWalkResult::Ok;
                }
                let first_slash = dir.find('/').unwrap_or(dir.len());
                let step_name = &dir[..first_slash];
                if step_name == "replies" {
                    return git2::TreeWalkResult::Ok;
                }
                let rest_dir = if first_slash + 1 < dir.len() { &dir[first_slash + 1..] } else { "" };
                let name = entry.name().unwrap_or("");
                let rel = format!("{}{}", rest_dir, name);
                out.entry(step_name.to_string()).or_default().push(rel);
                git2::TreeWalkResult::Ok
            })?;
            Ok(out)
        },
    )
}

/// Build a `rel_path → introducing_sha` index in one history walk.
/// Replies are append-only, so the "introducing" commit is the unique add.
/// Replaces an O(N paths × full diff walk) per-path search with one walk.
pub fn introducing_commits<'a, I: IntoIterator<Item = &'a str>>(
    root: &Path,
    rel_paths: I,
) -> Result<std::collections::HashMap<String, String>> {
    use std::collections::{HashMap, HashSet};
    let repo = open(root)?;
    // Translate caller's trace-relative paths to repo-relative, and keep a
    // reverse mapping so the returned keys read back in trace-relative form.
    let pairs: Vec<(String, String)> = rel_paths
        .into_iter()
        .map(|r| (translate(&repo, root, r), r.to_string()))
        .collect();
    let wanted: HashSet<String> = pairs.iter().map(|(k, _)| k.clone()).collect();
    let back: HashMap<String, String> = pairs.into_iter().collect();
    let mut hits: HashMap<String, String> = HashMap::with_capacity(wanted.len());
    if wanted.is_empty() {
        return Ok(hits);
    }
    let mut walk = repo.revwalk()?;
    walk.push_head()?;
    walk.set_sorting(git2::Sort::TIME)?;

    for oid in walk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let commit_tree = commit.tree()?;
        let parents: Vec<git2::Tree> = (0..commit.parent_count())
            .filter_map(|i| commit.parent(i).ok().and_then(|p| p.tree().ok()))
            .collect();

        let record = |path: Option<&Path>, hits: &mut HashMap<String, String>| {
            let Some(p) = path else { return };
            let Some(s) = p.to_str() else { return };
            if wanted.contains(s) && !hits.contains_key(s) {
                hits.insert(s.to_string(), oid.to_string());
            }
        };

        if parents.is_empty() {
            commit_tree.walk(git2::TreeWalkMode::PreOrder, |dir, entry| {
                let mut p = PathBuf::from(dir);
                if let Some(name) = entry.name() {
                    p.push(name);
                }
                record(Some(p.as_path()), &mut hits);
                git2::TreeWalkResult::Ok
            })?;
        } else {
            for parent_tree in &parents {
                if let Ok(diff) = repo.diff_tree_to_tree(Some(parent_tree), Some(&commit_tree), None) {
                    for delta in diff.deltas() {
                        record(delta.new_file().path(), &mut hits);
                        record(delta.old_file().path(), &mut hits);
                    }
                }
            }
        }
        if hits.len() == wanted.len() {
            break;
        }
    }
    // Translate repo-relative keys back to trace-relative for the caller.
    let mut out: HashMap<String, String> = HashMap::with_capacity(hits.len());
    for (k, sha) in hits {
        if let Some(rel) = back.get(&k) {
            out.insert(rel.clone(), sha);
        }
    }
    Ok(out)
}

/// Return the bytes of `rel_path` as it existed at commit `sha`. Errors if
/// the commit or path does not exist in that tree.
pub fn show_at(root: &Path, sha: &str, rel_path: &str) -> Result<Vec<u8>> {
    show_at_with_blob_sha(root, sha, rel_path).map(|(_, bytes)| bytes)
}

/// Same as [`show_at`] but also returns the blob's OID as a 40-char hex sha.
/// Callers can use this as an ETag: blob content is content-addressed in git,
/// so this sha uniquely identifies the bytes (even across renames / branches).
pub fn show_at_with_blob_sha(
    root: &Path,
    sha: &str,
    rel_path: &str,
) -> Result<(String, Vec<u8>)> {
    let repo = open(root)?;
    let translated = translate(&repo, root, rel_path);
    let oid = git2::Oid::from_str(sha).context("invalid commit sha")?;
    let commit = repo
        .find_commit(oid)
        .with_context(|| format!("commit {} not found", sha))?;
    let tree = commit.tree()?;
    let entry = tree
        .get_path(Path::new(&translated))
        .with_context(|| format!("path {} not in commit {}", rel_path, sha))?;
    let blob_oid = entry.id().to_string();
    let object = entry.to_object(&repo)?;
    let blob = object
        .peel(ObjectType::Blob)
        .context("path is not a blob")?
        .into_blob()
        .map_err(|_| anyhow::anyhow!("path is not a blob"))?;
    Ok((blob_oid, blob.content().to_vec()))
}

// ──────────────────────────────────────────────────────────────────────────
// Canonical commit-message formatters. CLI and HTTP routes both call these
// so the git log shape is identical regardless of write surface.
// ──────────────────────────────────────────────────────────────────────────

fn label_with_msg(prefix: &str, kind: &str, message: Option<&str>) -> String {
    match message {
        Some(m) if !m.is_empty() => format!("{}: {} — {}", prefix, kind, m),
        _ => format!("{}: {}", prefix, kind),
    }
}

pub fn step_commit_msg(step_id: &str, status: &Status) -> String {
    label_with_msg(step_id, status.kind(), status_message(status))
}

pub fn deliverable_commit_msg(status: &Status) -> String {
    label_with_msg("deliverable", status.kind(), status_message(status))
}

pub fn reply_commit_msg(step_id: Option<&str>, seq: u32) -> String {
    match step_id {
        Some(s) => format!("reply: {} ({:04})", s, seq),
        None => format!("reply: ({:04})", seq),
    }
}

pub fn run_event_commit_msg(run_id: &str, event: &str) -> String {
    format!("run/{}: {}", run_id, event)
}

fn status_message(status: &Status) -> Option<&str> {
    match status {
        Status::Idle => None,
        Status::Running { message } | Status::Done { message } => message.as_deref(),
        Status::Blocked { message } | Status::Error { message } => Some(message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write(p: &Path, b: &[u8]) {
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, b).unwrap();
    }

    #[test]
    fn init_is_idempotent() {
        let d = tempdir().unwrap();
        init(d.path()).unwrap();
        init(d.path()).unwrap();
        assert!(d.path().join(".git").exists());
    }

    #[test]
    fn commit_files_noop_when_clean() {
        let d = tempdir().unwrap();
        init(d.path()).unwrap();
        write(&d.path().join("a.txt"), b"hi");
        commit_files(d.path(), "first", &["a.txt"]).unwrap();
        let sha1 = head_sha(d.path()).unwrap();
        commit_files(d.path(), "second", &["a.txt"]).unwrap();
        let sha2 = head_sha(d.path()).unwrap();
        assert_eq!(sha1, sha2, "no-op commit should not advance HEAD");
    }

    #[test]
    fn init_nested_trace_does_not_commit_parent_index() {
        let parent = tempdir().unwrap();
        init(parent.path()).unwrap();
        write(&parent.path().join("baseline.txt"), b"baseline");
        commit_files(parent.path(), "baseline", &["baseline.txt"]).unwrap();
        let parent_head = head_sha(parent.path()).unwrap();

        write(&parent.path().join("unrelated.txt"), b"keep staged");
        let parent_repo = Repository::open(parent.path()).unwrap();
        let mut parent_index = parent_repo.index().unwrap();
        parent_index.add_path(Path::new("unrelated.txt")).unwrap();
        parent_index.write().unwrap();

        let child = parent.path().join("probe-trace");
        fs::create_dir_all(&child).unwrap();
        write(&child.join("trace.json"), br#"{"id":"probe-trace"}"#);
        init(&child).unwrap();
        commit_files(&child, "initial trace", &["trace.json"]).unwrap();

        let new_parent_head = head_sha(parent.path()).unwrap();
        assert_ne!(new_parent_head, parent_head);
        assert!(!child.join(".git").exists());
        let parent_repo = Repository::open(parent.path()).unwrap();
        let parent_commit = parent_repo.head().unwrap().peel_to_commit().unwrap();
        let parent_tree = parent_commit.tree().unwrap();
        assert!(parent_tree
            .get_path(Path::new("probe-trace/trace.json"))
            .is_ok());
        assert!(parent_tree.get_path(Path::new("unrelated.txt")).is_err());
        let parent_index = parent_repo.index().unwrap();
        assert!(parent_index
            .get_path(Path::new("unrelated.txt"), 0)
            .is_some());
    }

    #[test]
    fn show_at_returns_historical_bytes() {
        let d = tempdir().unwrap();
        init(d.path()).unwrap();
        let p = d.path().join("note.txt");
        write(&p, b"v1");
        commit_files(d.path(), "v1", &["note.txt"]).unwrap();
        let v1_sha = head_sha(d.path()).unwrap();
        write(&p, b"v2");
        commit_files(d.path(), "v2", &["note.txt"]).unwrap();
        let v2_sha = head_sha(d.path()).unwrap();
        assert_ne!(v1_sha, v2_sha);
        assert_eq!(show_at(d.path(), &v1_sha, "note.txt").unwrap(), b"v1");
        assert_eq!(show_at(d.path(), &v2_sha, "note.txt").unwrap(), b"v2");
    }

    #[test]
    fn commits_under_filters_by_path() {
        let d = tempdir().unwrap();
        init(d.path()).unwrap();
        write(&d.path().join("a/x.txt"), b"a1");
        commit_files(d.path(), "a1", &["a/x.txt"]).unwrap();
        write(&d.path().join("b/y.txt"), b"b1");
        commit_files(d.path(), "b1", &["b/y.txt"]).unwrap();
        write(&d.path().join("a/x.txt"), b"a2");
        commit_files(d.path(), "a2", &["a/x.txt"]).unwrap();

        let under_a = commits_under(d.path(), "a").unwrap();
        let msgs: Vec<_> = under_a.iter().map(|c| c.message.clone()).collect();
        assert!(msgs.contains(&"a1".to_string()));
        assert!(msgs.contains(&"a2".to_string()));
        assert!(!msgs.contains(&"b1".to_string()));
    }

    #[test]
    fn introducing_commits_returns_add_commit() {
        let d = tempdir().unwrap();
        init(d.path()).unwrap();
        write(&d.path().join("a.txt"), b"a");
        commit_files(d.path(), "add a", &["a.txt"]).unwrap();
        let sha_a = head_sha(d.path()).unwrap();
        write(&d.path().join("b.txt"), b"b");
        commit_files(d.path(), "add b", &["b.txt"]).unwrap();
        let sha_b = head_sha(d.path()).unwrap();
        let m = introducing_commits(d.path(), ["a.txt", "b.txt"]).unwrap();
        assert_eq!(m.get("a.txt"), Some(&sha_a));
        assert_eq!(m.get("b.txt"), Some(&sha_b));
    }

    #[test]
    fn list_dir_blobs_at_returns_dir_contents() {
        let d = tempdir().unwrap();
        init(d.path()).unwrap();
        write(&d.path().join("x/1.txt"), b"one");
        write(&d.path().join("x/2.txt"), b"two");
        commit_files(d.path(), "x/1 and x/2", &["x/1.txt", "x/2.txt"]).unwrap();
        let sha = head_sha(d.path()).unwrap();
        let mut blobs = list_dir_blobs_at(d.path(), &sha, "x").unwrap();
        blobs.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(blobs.len(), 2);
        assert_eq!(blobs[0].0, "1.txt");
        assert_eq!(blobs[0].1, b"one");
        assert_eq!(blobs[1].0, "2.txt");
        assert_eq!(blobs[1].1, b"two");
    }
}
