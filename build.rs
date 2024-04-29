use std::process::Command;
use std::io::{self, Write};

// Example custom build script.
fn main() {
    //Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/*.cs");
    println!("cargo:rerun-if-changed=project.csproj");

    let output = Command::new("dotnet")
        .args(&["build", "project.csproj"])
        .output()
        .unwrap();

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
