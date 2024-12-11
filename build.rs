use std::process::Command;

fn main() {
  // Run `npm run build`
  let status = Command::new("npm")
    .arg("run")
    .arg("build")
    .status()
    .expect("Failed to execute npm run build");

  if !status.success() {
    panic!("npm run build failed with status: {}", status);
  }

  // Tell Cargo to re-run the build script if package.json changes
  println!("cargo:rerun-if-changed=package.json");
  println!("cargo:rerun-if-changed=vite.config.js");
  println!("cargo:rerun-if-changed=views/main.js");
}
