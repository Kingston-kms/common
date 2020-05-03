use std::process::Command;

fn main() {
    let git_branch = Command::new("git").args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output().and_then(|param| String::from_utf8(param.stdout))
        .unwrap_or(String::from("Unknown"));

    let git_commit = Command::new("git").args(&["rev-parse", "HEAD"])
        .output().and_then(|param| String::from_utf8(param.stdout))
        .unwrap_or(String::from("Unknown"));
    }

    let commit_date = Command::new("git").args(&["log", "-1", "--date=iso", "--pretty=format:%cd"])
        .output().and_then(|param| String::from_utf8(param.stdout))
        .unwrap_or(String::from("Unknown"));

    let build_time = Command::new("date").args(&["+%Y-%m-%d %T %z"])
        .output().and_then(|param| String::from_utf8(param.stdout))
        .unwrap_or(String::from("Unknown"));

    println!("cargo:rustc-env=BUILD_GIT_BRANCH={}", git_branch);
    println!("cargo:rustc-env=BUILD_GIT_COMMIT={}", git_commit);
    println!("cargo:rustc-env=BUILD_GIT_DATE={}", commit_date);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
}
