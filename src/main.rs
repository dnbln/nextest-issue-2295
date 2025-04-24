use std::io;
use prodash::tree::root::Options;
use std::io::BufRead;
use std::process::{Command, Stdio};
use std::sync::Arc;
use prodash::render::line;

fn main() {
    let mut tree = Arc::new(
        Options {
            message_buffer_capacity: 30,
            ..Default::default()
        }
        .create(),
    );
    let root = tree.add_child("Root");
    let tree = Arc::downgrade(&tree);

    let mut opts = line::Options {
        frames_per_second: 20.0,
        ..Default::default()
    }
        .auto_configure(line::StreamKind::Stderr);

    let handle = line::render(io::stderr(), tree, opts);

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

    handle.shutdown_and_wait();
}
