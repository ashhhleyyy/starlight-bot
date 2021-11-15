use std::process::Command;

fn main() {
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    let output = Command::new("git").args(&["status", "--porcelain"]).output().unwrap();
    let is_dirty = !output.stdout.is_empty();
    if is_dirty {
        println!("cargo:rustc-env=GIT_HASH={}+dirty", &git_hash[..7]);
    } else {
        println!("cargo:rustc-env=GIT_HASH={}", &git_hash[..7]);
    }
    let output = Command::new("git").args(&["log", "-5", "--pretty=format:[`%h`] %an: %s"]).output().unwrap();
    let latest_commits = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_LOG={}", latest_commits.replace("\n", "\\n"));
}
