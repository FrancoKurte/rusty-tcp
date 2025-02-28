use std::io;
use std::time::SystemTime;

pub mod util;
 
fn main() -> io::Result<()> {
    let iface_name: &str = "tun0";
    let nic = tun_tap::Iface::new(iface_name, tun_tap::Mode::Tun)?;
    util::addr::wait_for_addr(iface_name);

    let mut buffer = [0u8; 1504];
    const TIMER_LIMIT: u64 = 2;
    let mut timer = SystemTime::now();
    const PROTO_IPV4: [u8; 4] = [0, 0, 8, 0];

    loop {
        match nic.recv(&mut buffer[..]) {
            Ok(nbytes) => {
                timer = SystemTime::now();

                // Check the protocol bytes (first 4 bytes) for IPv4
                if buffer[..4] != PROTO_IPV4 {
                    // Skip non-IPv4 packets (e.g., IPv6)
                    continue;
                }

                // Parse the IPv4 header starting after the protocol bytes
                match etherparse::Ipv4HeaderSlice::from_slice(&buffer[4..nbytes]) {
                    Ok(packet) => {
                        eprintln!(
                            "{} -> {}; {}; {}b; id={}; {}",
                            packet.source_addr(),
                            packet.destination_addr(),
                            packet.protocol().0,
                            packet.payload_len().unwrap(),
                            packet.identification(),
                            packet.ttl(),
                        );
                    }
                    Err(e) => {
                        println!("Error in packet parsing: {:?}", e);
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                match timer.elapsed() {
                    Ok(elapsed) => {
                        if elapsed.as_secs() >= TIMER_LIMIT {
                            println!(
                                "No data received for {} seconds. Shutting down.",
                                elapsed.as_secs()
                            );
                            return Ok(());
                        }
                    }
                    Err(e) => {
                        println!("Error in timer: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error in receiving packets: {:?}", e);
            }
        }
    }
}
