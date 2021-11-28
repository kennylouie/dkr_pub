use std::process::Command;

pub fn build() {
    let command = format!("docker build -t {}:{}", )
    let out = Command::new("sh").arg("-c").arg("docker images").output().expect("process failed to execute");

    if !out.status.success() {
        panic!("failed");
    }

    println!("{:?}", String::from_utf8(out.stdout).unwrap());
}
