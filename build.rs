fn main() {
    // SDL2のライブラリとヘッダのディレクトリを指定
    println!("cargo:rustc-link-search=native=C:\\SDL2-devel-2.30.7-VC\\SDL2-2.30.7\\lib\\x64");
    println!("cargo:rustc-link-lib=dylib=SDL2");
}