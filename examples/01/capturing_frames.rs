// rusty_tcp/examples/01/capturing_frames.rs
use rusty_tcp::xdp::XdpCapture;
use anyhow::Result;
use std::io::Write;
use std::str;

fn main() -> Result<()> {
    // Parse command line arguments for interface name
    let args: Vec<String> = std::env::args().collect();
    let interface = args.get(1).map(|s| s.as_str()).unwrap_or("wlan0");
    
    // Initialize XDP capture on the specified interface
    let capture = XdpCapture::new(interface)?;
    println!("XDP capture initialized on interface {}, ringbuf_fd: {}", 
             interface, capture.ringbuf_fd());
    println!("Waiting for packets... Press Ctrl+C to exit");
    
    // Main loop to read and process frames
    let mut frame_count = 0;
    loop {
        // Poll for frames with a 100ms timeout
        match capture.poll_frame(100) {
            Ok(Some(frame)) => {
                frame_count += 1;
                println!("\n--- Frame #{} ({} bytes) ---", frame_count, frame.len());
                
                // Print the frame in a hexdump-like format
                print_hexdump(&frame);
                
                // Try to decode printable ASCII
                print_ascii(&frame);
                
                // Flush stdout to ensure output is visible immediately
                std::io::stdout().flush()?;
            }
            Ok(None) => {
                // No frame received, just continue polling
            }
            Err(e) => {
                eprintln!("Error polling for frames: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

/// Prints a hexdump of the binary data
fn print_hexdump(data: &[u8]) {
    const BYTES_PER_LINE: usize = 16;
    
    for chunk in data.chunks(BYTES_PER_LINE) {
        // Print offset
        print!("{:08x}  ", (chunk.as_ptr() as usize) - (data.as_ptr() as usize));
        
        // Print hex values
        for (i, byte) in chunk.iter().enumerate() {
            print!("{:02x} ", byte);
            if i == 7 {
                print!(" "); // Extra space in the middle
            }
        }
        
        // Padding if the last line is incomplete
        if chunk.len() < BYTES_PER_LINE {
            let padding = BYTES_PER_LINE - chunk.len();
            for _ in 0..padding {
                print!("   ");
            }
            if chunk.len() <= 8 {
                print!(" "); // Extra space for middle alignment
            }
        }
        
        // Print ASCII representation
        print!("  |");
        for byte in chunk {
            if *byte >= 32 && *byte <= 126 {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }
        println!("|");
    }
}

/// Attempts to print ASCII representation if the data appears to be text
fn print_ascii(data: &[u8]) {
    let printable_chars = data.iter().filter(|&&b| b >= 32 && b <= 126).count();
    let ratio = printable_chars as f32 / data.len() as f32;
    
    // If more than 70% of the bytes are printable ASCII, try to show as text
    if ratio > 0.7 {
        if let Ok(text) = str::from_utf8(data) {
            println!("\nPossible ASCII text:\n{}", text);
        }
    }
}

