// src/xdp/capture.rs
use std::ffi::{CString, c_void};
use std::os::unix::io::RawFd;
use std::ptr;
use std::slice;
use libc::{c_char, c_int};
use libc::c_uint;

/// Handle to the XDP loader (defined in C)
#[repr(C)]
struct XdpLoader {
    // Actual fields are in C
    _private: [u8; 0],
}

extern "C" {
    fn xdp_loader_init(ifname: *const c_char) -> *mut XdpLoader;
    fn xdp_loader_cleanup(loader: *mut XdpLoader);
    fn xdp_loader_get_ringbuf_fd(loader: *mut XdpLoader) -> c_int;
}

// Constants matching those in the BPF program
const RINGBUF_FRAME_SIZE: usize = 2048;

// Struct for libbpf ring buffer callback
#[allow(non_camel_case_types)]
type ring_buffer_sample_fn = extern "C" fn(ctx: *mut c_void, data: *mut c_void, size: c_uint) -> c_int;

// Binding to libbpf ring buffer functions
extern "C" {
    fn ring_buffer__new(fd: c_int, sample_cb: ring_buffer_sample_fn, ctx: *mut c_void, opts: *const c_void) -> *mut c_void;
    fn ring_buffer__free(rb: *mut c_void);
    fn ring_buffer__poll(rb: *mut c_void, timeout_ms: c_int) -> c_int;
}

// Global buffer to store the current frame (used in the callback)
thread_local! {
    static CURRENT_FRAME: std::cell::RefCell<Option<Vec<u8>>> = std::cell::RefCell::new(None);
}

// Callback function for ring buffer
extern "C" fn process_sample(_ctx: *mut c_void, data: *mut c_void, _size: c_uint) -> c_int {
    // The first 4 bytes contain the actual frame size
    unsafe {
        let size_ptr = data as *const u32;
        let frame_size = *size_ptr as usize;
        
        // Ensure we don't read beyond the buffer
        let read_size = std::cmp::min(frame_size, RINGBUF_FRAME_SIZE - 4);
        
        // Create a vector from the data (skip the first 4 bytes which contain the size)
        let frame_data = slice::from_raw_parts((data as *const u8).add(4), read_size);
        let frame = frame_data.to_vec();
        
        // Store the frame in thread-local storage
        CURRENT_FRAME.with(|f| {
            *f.borrow_mut() = Some(frame);
        });
    }
    
    0
}

/// Manages the XDP frame capture functionality.
pub struct XdpCapture {
    loader: *mut XdpLoader,
    ringbuf_fd: RawFd,
    ring_buffer: *mut c_void,
}

impl XdpCapture {
    /// Initializes XDP frame capture on the specified network interface.
    pub fn new(ifname: &str) -> Result<Self, anyhow::Error> {
        let ifname_c = CString::new(ifname)?;
        let loader = unsafe { xdp_loader_init(ifname_c.as_ptr()) };
        if loader.is_null() {
            return Err(anyhow::anyhow!("Failed to initialize XDP loader"));
        }
        
        let ringbuf_fd = unsafe { xdp_loader_get_ringbuf_fd(loader) };
        if ringbuf_fd < 0 {
            unsafe { xdp_loader_cleanup(loader) };
            return Err(anyhow::anyhow!("Failed to get ring buffer FD"));
        }
        
        // Initialize the ring buffer
        let ring_buffer = unsafe { 
            ring_buffer__new(ringbuf_fd, process_sample, ptr::null_mut(), ptr::null()) 
        };
        
        if ring_buffer.is_null() {
            unsafe { xdp_loader_cleanup(loader) };
            return Err(anyhow::anyhow!("Failed to create ring buffer"));
        }
        
        Ok(XdpCapture {
            loader,
            ringbuf_fd,
            ring_buffer,
        })
    }

    /// Returns the ring buffer file descriptor for polling or mapping.
    pub fn ringbuf_fd(&self) -> RawFd {
        self.ringbuf_fd
    }

    /// Polls the ring buffer for new frames with a timeout in milliseconds.
    /// Returns the captured frame if available.
    pub fn poll_frame(&self, timeout_ms: i32) -> Result<Option<Vec<u8>>, anyhow::Error> {
        // Clear any previous frame
        CURRENT_FRAME.with(|f| {
            *f.borrow_mut() = None;
        });
        
        // Poll the ring buffer
        let ret = unsafe { ring_buffer__poll(self.ring_buffer, timeout_ms) };
        if ret < 0 {
            return Err(anyhow::anyhow!("Error polling ring buffer: {}", ret));
        }
        
        // Get the frame from thread-local storage if available
        let frame = CURRENT_FRAME.with(|f| {
            f.borrow_mut().take()
        });
        
        Ok(frame)
    }
}

impl Drop for XdpCapture {
    fn drop(&mut self) {
        unsafe { 
            if !self.ring_buffer.is_null() {
                ring_buffer__free(self.ring_buffer);
            }
            xdp_loader_cleanup(self.loader) 
        };
    }
}
