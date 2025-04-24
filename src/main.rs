use std::io;
use std::io::BufRead;
use std::process::{Command, Stdio};
use std::sync::Arc;

fn main() {
    let mut cmd = Command::new("cargo");
    
    let f = std::fs::File::create("nextest.stderr.log").unwrap();

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
    .stderr(Stdio::from(f));

    let mut child = cmd.spawn().unwrap();
    let stdout = child.stdout.take().unwrap();

    println!("::group::Test results");

    for line in std::io::BufReader::new(stdout).lines() {
        println!("{}", line.unwrap());
    }

    println!("::endgroup::");

    child.wait().unwrap();
}
