fn main() {
    println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
    println!("cargo:rustc-link-arg=-Wl,--copy-dt-needed-entries");
    println!("cargo:rustc-link-arg=-ltorch");
    println!("cargo:rustc-link-arg=-ltorch_cuda");
}
