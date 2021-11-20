use git2::Repository;

fn main() {
    let repo = Repository::open(".").unwrap();
    let head = repo.head().unwrap();
    let head_commit = head.peel_to_commit().unwrap();
    let mut is_dirty = false;
    for status in repo.statuses(None).unwrap().iter() {
        let status = status.status();
        if !status.contains(git2::Status::IGNORED) {
            is_dirty = true;
            break;
        }
    }

    if is_dirty {
        println!("cargo:rustc-env=GIT_HASH={}+dirty", &head_commit.id().to_string()[..7]);
    } else {
        println!("cargo:rustc-env=GIT_HASH={}", &head_commit.id().to_string()[..7]);
    }

    let mut revwalk = repo.revwalk().unwrap();

    revwalk.set_sorting(git2::Sort::NONE).unwrap();
    revwalk.push_head().unwrap();

    let revwalk = revwalk.map(|id| {
        let id = id.unwrap();
        println!("{}", id);
        let commit = repo.find_commit(id).unwrap();
        return format!("[`{}`] {}: {}", &id.to_string()[..7], commit.author().name().unwrap(), commit.summary().unwrap());
    }).take(5);

    // let output = Command::new("git").args(&["log", "-5", "--pretty=format:[`%h`] %an: %s"]).output().unwrap();
    // let latest_commits = String::from_utf8(output.stdout).unwrap();
    let latest_commits = revwalk.collect::<Vec<_>>().join("\n");

    println!("cargo:rustc-env=GIT_LOG={}", latest_commits.replace("\n", "\\n"));
}
