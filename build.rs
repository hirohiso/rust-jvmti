use std::env;
use std::path;

fn main() {
    // Note: These paths are environment-specific and need to be adjusted based on your JDK installation
    // The following are examples for different operating systems:
    
    // macOS example (update version as needed):
    const LIB: &str = "/Library/Java/JavaVirtualMachines/openjdk-13.0.1.jdk/Contents/Home/lib/server";
    const INCLUDE: &str = "/Library/Java/JavaVirtualMachines/openjdk-13.0.1.jdk/Contents/Home/include";
    const INCLUDE_LINUX: &str = "/Library/Java/JavaVirtualMachines/openjdk-13.0.1.jdk/Contents/Home/include/darwin";
    
    // Linux example (uncomment and adjust as needed):
    // let java_home = env::var("JAVA_HOME").unwrap_or_else(|_| "/usr/lib/jvm/default-java".to_string());
    // let lib_path = format!("{}/lib/server", java_home);
    // let include_path = format!("{}/include", java_home);
    // let include_linux_path = format!("{}/include/linux", java_home);

    //https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    println!("cargo:rustc-link-lib=jvm");
    println!("cargo:rustc-link-search=native={}", LIB);

    let bindings = bindgen::builder()
        .header_contents("bindings.h", "#include <jvmti.h>")//#include <jvmti.h>という内容のbindings.hを生成する（Diskに保持されない）
        .clang_arg(format!("-I{}", INCLUDE))
        .clang_arg(format!("-I{}", INCLUDE_LINUX))
        .derive_debug(true)
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate bindgen.");

    let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings.rs.");
}