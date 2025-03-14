// src/xdp/bpf/mod.rs
use std::os::raw::{c_uchar, c_ulong};

// Embed the xdp.bpf.o file as a byte array
const XDP_BPF_O_BYTES: &[u8] = include_bytes!(env!("XDP_BPF_O"));

// Expose the byte array and its length to C via FFI
#[no_mangle]
pub extern "C" fn get_xdp_bpf_o_data() -> *const c_uchar {
    XDP_BPF_O_BYTES.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_xdp_bpf_o_len() -> c_ulong {
    XDP_BPF_O_BYTES.len() as c_ulong
}
