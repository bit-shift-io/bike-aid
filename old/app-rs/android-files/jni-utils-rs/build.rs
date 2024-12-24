#[cfg(feature = "build-java-support")]
fn build_java() {
    use std::{env, path::PathBuf, process::Command};

    let mut java_src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    java_src_dir.push("java");

    let mut java_src_gradlew = java_src_dir.clone();
    java_src_gradlew.push(
        #[cfg(target_os = "windows")]
        "gradlew.bat",
        #[cfg(not(target_os = "windows"))]
        "gradlew",
    );

    let mut java_build_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    java_build_dir.pop();
    java_build_dir.pop();
    java_build_dir.pop();
    java_build_dir.push("java");

    let result = Command::new(java_src_gradlew)
        .args(&[
            format!("-PbuildDir={}", java_build_dir.to_str().unwrap()),
            "-p".to_string(),
            java_src_dir.to_str().unwrap().to_string(),
            "build".to_string(),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    if !result.success() {
        panic!("Gradle failed");
    }
}

fn main() {
    #[cfg(feature = "build-java-support")]
    build_java();
}
