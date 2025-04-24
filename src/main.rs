use std::io::BufRead;
use std::process::{Command, Stdio};

fn main() {
    let mut cmd = Command::new("cargo");

    cmd.args([
        "nextest",
        "run",
        "--message-format",
        "libtest-json-plus",
        "--workspace",
        "--no-fail-fast",
        "--hide-progress-bar",
        "--final-status-level",
        "none",
        "--status-level",
        "none",
        "--success-output",
        "never",
        "--failure-output",
        "never",
        "--no-input-handler",
    ])
        .env("NEXTEST_EXPERIMENTAL_LIBTEST_JSON", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    let mut child = cmd.spawn().unwrap();
    let stdout = child.stdout.take().unwrap();

    for line in std::io::BufReader::new(stdout).lines() {
        println!("{}", line.unwrap());
    }

    child.wait().unwrap();
}
