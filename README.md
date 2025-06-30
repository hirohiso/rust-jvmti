# Rust JVMTI

A Rust library that provides JVM Tool Interface (JVMTI) bindings for creating Java agents.

## Requirements

- Rust 1.70 or later (updated to use Rust edition 2021)
- JDK with JVMTI headers available
- Clang (required by bindgen)

## Building

Before building, you need to update the JDK paths in `build.rs` to match your environment:

### macOS
Update the paths in `build.rs` to point to your JDK installation:
```rust
const LIB: &str = "/Library/Java/JavaVirtualMachines/your-jdk/Contents/Home/lib/server";
const INCLUDE: &str = "/Library/Java/JavaVirtualMachines/your-jdk/Contents/Home/include";
const INCLUDE_LINUX: &str = "/Library/Java/JavaVirtualMachines/your-jdk/Contents/Home/include/darwin";
```

### Linux
Uncomment and adjust the Linux configuration in `build.rs`:
```rust
let java_home = env::var("JAVA_HOME").unwrap_or_else(|_| "/usr/lib/jvm/default-java".to_string());
let lib_path = format!("{}/lib/server", java_home);
let include_path = format!("{}/include", java_home);
let include_linux_path = format!("{}/include/linux", java_home);
```

Then build:
```bash
cargo build
```

## Recent Updates

- Updated to Rust edition 2021
- Updated bindgen dependency to 0.72.0 (latest)
- Fixed deprecated API usage
- Added minimum Rust version specification (1.70)
- Improved documentation for cross-platform JDK configuration

The library is now compatible with modern Rust versions including Rust 1.87+.