use std::env;
use std::path;

fn main() {
    const LIB: &str = "/Library/Java/JavaVirtualMachines/openjdk-13.0.1.jdk/Contents/Home/lib/server";
    const INCLUDE: &str = "/Library/Java/JavaVirtualMachines/openjdk-13.0.1.jdk/Contents/Home/include";
    const INCLUDE_LINUX: &str = "/Library/Java/JavaVirtualMachines/openjdk-13.0.1.jdk/Contents/Home/include/darwin";

    //https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    println!("cargo:rustc-link-lib=jvm");
    println!("cargo:rustc-link-search=native={}", LIB);

    let bindings = bindgen::builder()
        .header_contents("bindings.h", "#include <jvmti.h>")//#include <jvmti.h>という内容のbindings.hを生成する（Diskに保持されない）
        .clang_arg(format!("-I{}", INCLUDE))
        .clang_arg(format!("-I{}", INCLUDE_LINUX))
        .derive_debug(true)
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("failed to generate bindgen.");

    let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings.rs.");
}