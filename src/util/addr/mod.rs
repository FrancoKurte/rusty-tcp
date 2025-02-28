// src/util/addr/mod.rs
use std::time::Duration;

/// Function to wait until the network interface has an assigned IP address.
/// This ensures that the interface is ready before we start processing packets.
pub fn wait_for_addr(iface_name: &str) {
    loop {
        let out = std::process::Command::new("ip")
            .arg("addr")
            .arg("show")
            .arg(iface_name)
            .output()
            .expect("Failed to execute command 'ip'");

        // Check if the output contains an assigned IP address
        if String::from_utf8_lossy(&out.stdout).contains("inet ") {
            break;
        } else {
            // Wait before retrying and gives extra time to the kernel to proceed
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}



