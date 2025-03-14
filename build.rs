// build.rs
use std::process::Command;
use std::path::Path;

fn main() {
    // Compile the eBPF program (xdp.bpf.c) into an object file (xdp.bpf.o)
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let xdp_bpf_c = "src/xdp/bpf/xdp.bpf.c";
    let xdp_bpf_o = Path::new(&out_dir).join("xdp.bpf.o");

    // In build.rs, modify the clang command:
    let clang_status = Command::new("clang")
        .args(&[
            "-O2",
            "-target", "bpf",
            "-c",
            "-g",  // Add debug info for BTF
            xdp_bpf_c,
            "-o",
            xdp_bpf_o.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute clang");    

    if !clang_status.success() {
        panic!("Failed to compile eBPF program");
    }

    // Compile the C loader (xdp_loader.c) and link it
    cc::Build::new()
        .file("src/xdp/bpf/xdp_loader.c")
        .flag("-I/usr/include") // Ensure libbpf headers are available
        .compile("xdp_loader");

    // Link against libbpf
    println!("cargo:rustc-link-lib=bpf");
    // Ensure the linker can find libbpf in the standard library path
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");

    // Rerun build if source files change
    println!("cargo:rerun-if-changed=src/xdp/bpf/xdp.bpf.c");
    println!("cargo:rerun-if-changed=src/xdp/bpf/xdp_loader.c");

    // Tell Cargo to include the generated xdp.bpf.o in the build
    println!("cargo:rustc-env=XDP_BPF_O={}", xdp_bpf_o.display());
}
