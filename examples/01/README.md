# Example 01: Capturing Network Frames with Rusty TCP

This example demonstrates basic network frame capture using the Rusty TCP library. It initializes XDP capture on a specified network interface and continuously polls for incoming frames. Captured frames are then displayed in a hexdump format on the console, along with an attempt to decode and display any printable ASCII text found within the frame.

## What it does

The `capturing_frames.rs` example performs the following actions:

1.  **Initializes XDP Capture:** It uses the `XdpCapture` struct from the `rusty_tcp` library to attach an eBPF program to a network interface. This program is designed to capture all network frames received by the interface.
2.  **Polls for Frames:** In a loop, it polls the ring buffer associated with the XDP program for newly captured frames. The polling is done with a timeout to avoid excessive CPU usage when no frames are available.
3.  **Displays Frame Information:** When a frame is received, the example prints:
    *   A frame counter.
    *   The size of the captured frame in bytes.
    *   A hexdump representation of the frame data, showing the raw byte values in hexadecimal format alongside a potential ASCII representation.
    *   If a significant portion of the frame data appears to be printable ASCII characters (above a 70% threshold), it will also attempt to decode and display the data as ASCII text.

## How to Run

1.  **Navigate to the Example Directory:**
    ```bash
    cd examples/01
    ```

2.  **Run the Example:**
    You need to specify the network interface you want to capture traffic from as a command-line argument. For example, to capture traffic on the `wlan0` interface, use:
    ```bash
    cargo run -- capturing_frames wlan0
    ```
    Replace `wlan0` with the actual name of your network interface (e.g., `eth0`, `enp0s3`, etc.). You can usually find your interface names using the `ip link` or `ifconfig` commands.

    **Note:** Running this example requires root privileges or capabilities that allow attaching XDP programs to network interfaces. You might need to run it using `sudo`:
    ```bash
    sudo cargo run -- capturing_frames wlan0
    ```

3.  **Observe the Output:**
    Once the example is running and XDP capture is initialized successfully, you will see messages indicating the interface and ring buffer FD. The program will then wait for network packets. As packets are received on the specified interface, you will see output similar to the following for each captured frame:

    ```
    XDP capture initialized on interface wlan0, ringbuf_fd: 3
    Waiting for packets... Press Ctrl+C to exit

    --- Frame #1 (60 bytes) ---
    00000000  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
    00000010  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
    00000020  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
    00000030  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|

    --- Frame #2 (1514 bytes) ---
    00000000  ff ff ff ff ff ff 00 00  00 00 00 00 08 00 45 00  |..............E.|
    00000010  05 dc 00 00 40 00 ff 11  ff ff 00 00 00 00 00 00  |....@...........|
    ... (rest of hexdump) ...

    Possible ASCII text:
    ... (if applicable, decoded ASCII text from the frame) ...
    ```

4.  **Stop the Example:**
    Press `Ctrl+C` to terminate the program. This will gracefully detach the XDP program from the interface and clean up resources.


## Prerequisites

*   Ensure you have followed the installation instructions in the main project `README.md` to set up the Rust toolchain, Clang, libbpf, and have a Linux kernel with XDP support.
*   You need to have built the `rusty_tcp` library using `cargo build` in the project's root directory before running this example. This ensures that the core library components are compiled and available.
*   **Example-Specific Dependencies:** This example, `capturing_frames.rs`, utilizes additional Rust crates for functionality specific to the example, such as error handling and hexadecimal output formatting. These dependencies are:
    * [anyhow](https://crates.io/crates/anyhow) : For convenient error handling.
    *   [hex](https://crates.io/crates/hex) : For generating hexdump output.


