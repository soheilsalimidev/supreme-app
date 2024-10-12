use std::fs;

fn main() {
    #[cfg(not(debug_assertions))]
    {
        let _ = fs::remove_dir_all("./resources/lib64/");
        let _ = fs::create_dir("./resources/lib64/");
    }

    #[cfg(not(target_os = "macos"))]
    #[cfg(not(debug_assertions))]
    {
        let _ = fs::copy(
            "./resources/lib64-linux/libbcc.so",
            "./resources/lib64/libbcc.so",
        );
        let _ = fs::copy(
            "./resources/lib64-linux/libc++.so",
            "./resources/lib64/libc++.so",
        );
        let _ = fs::copy(
            "./resources/lib64-linux/libc++.so.1",
            "./resources/lib64/libc++.so.1",
        );
        let _ = fs::copy(
            "./resources/lib64-linux/libbcinfo.so",
            "./resources/lib64/libbcinfo.so",
        );
        let _ = fs::copy(
            "./resources/lib64-linux/libaapt2_jni.so",
            "./resources/lib64/libaapt2_jni.so",
        );
    }

    #[cfg(target_os = "macos")]
    #[cfg(not(debug_assertions))]
    {
        let _ = fs::copy(
            "./resources/lib64-mac/libbcc.dylib",
            "./resources/lib64/libbcc.dylib",
        );
        let _ = fs::copy(
            "./resources/lib64-mac/libc++.dylib",
            "./resources/lib64/libc++.dylib",
        );
        let _ = fs::copy(
            "./resources/lib64-mac/libc++.1.dylib",
            "./resources/lib64/libc++.1.dylib",
        );
        let _ = fs::copy(
            "./resources/lib64-mac/libbcinfo.dylib",
            "./resources/lib64/libbcinfo.dylib",
        );
    }

    tauri_build::build()
}
