fn main() {
    let cmd = std::process::Command::new("cargo").stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn().unwrap();
    println!("{}", cmd.stderr.is_none());
}
