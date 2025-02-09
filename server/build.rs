fn main() {
    println!("cargo::rerun-if-changed=../ui");
    println!("cargo::rerun-if-changed=../app");
    let result = std::process::Command::new("npm")
        .current_dir("../ui")
        .args(["run", "generate"])
        .output()
        .expect("Unable to generate Web UI");

    if !result.status.success() {
        let error = String::from_utf8_lossy(&result.stderr);
        println!("cargo::error=Unable to build Web UI!");
        println!("cargo::error=Output: {error}");
    }
}
