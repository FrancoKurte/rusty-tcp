# Rusty TCP: eBPF-Powered Network Capture Library in Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rusty TCP is a Rust library developed for network packet capture, leveraging the efficiency of eBPF (Extended Berkeley Packet Filter) and XDP (eXpress Data Path). Currently under active development in its early stages, this library aims to provide a streamlined and robust method for capturing network traffic directly at the kernel level, thereby minimizing overhead and maximizing throughput.

## Overview

This project centers around creating a user-friendly Rust interface to eBPF programs within the Linux kernel, specifically for network packet capture. By employing XDP, Rusty TCP enables packet processing at the earliest possible point in the network stack, directly within the network driver itself. Captured packets are then efficiently transferred to user space via a ring buffer, ready for immediate analysis or processing within your Rust applications.

Key features of Rusty TCP include:

* **High-Performance Capture:** Achieves near-line-rate packet capture with minimal kernel overhead thanks to XDP.
*   **Kernel Bypass Architecture:** Operates at the XDP layer, bypassing significant portions of the traditional kernel network stack for enhanced speed.
*   **Efficient Ring Buffer Delivery:** Utilizes a ring buffer mechanism for asynchronous and high-throughput transfer of network frames from kernel to user space.
*   **Rust and C Synergy:**  Combines the memory safety and expressiveness of Rust with the low-level system capabilities of C and eBPF for optimal performance and reliability.
*   **Modern eBPF Management:**  Integrated with the libbpf library for efficient, up-to-date eBPF program loading and lifecycle management.



## Installation

To utilize Rusty TCP, ensure the following prerequisites are installed on your system:

*   **Rust Toolchain:** A stable Rust installation is required. Installation instructions are available at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
*   **Clang Compiler:** Clang is necessary for compiling the eBPF program. Ensure it is installed and accessible in your system's PATH.
*   **libbpf Library:** The libbpf library and its development headers are essential for compiling and running the eBPF loader component. On Arch Linux systems, install with: `sudo pacman -S libbpf zlib`. For other distributions, use your system's package manager (eg. `apt`) to install the libbpf development packages.
*   **XDP-Enabled Linux Kernel:** Your Linux kernel must have XDP functionality enabled. Most modern Linux distributions include this support by default.

Once these prerequisites are met, you can incorporate Rusty TCP into your Rust project by adding it as a dependency in your `Cargo.toml` file:


```toml
[dependencies]
rusty_tcp = { git = "https://github.com/FrancoKurte/rusty-tcp" }
```

Alternatively, you can clone the repository and build the library locally:

```sh
git clone https://github.com/FrancoKurte/rusty-tcp
cd rusty-tcp
cargo build
```

## Usage

To quickly experience Rusty TCP's capabilities, explore the example provided in [examples/01/capturing_frames.rs](https://github.com/FrancoKurte/rusty-tcp/examples/01/). This example showcases basic packet capture and displays captured data in a hexdump format. For detailed instructions on running this example and understanding its output, please refer to the examples/01/README.md file within the example directory. More usage examples and comprehensive API documentation are planned for upcoming releases. Currently, the most detailed reference for library usage is the source code itself, particularly the src/xdp/capture.rs module.

## Future Prospects

Rusty TCP is envisioned to evolve into more than just a packet capture utility. A significant long-term goal for this project is the development of a complete, from-scratch TCP/IP stack implemented entirely in Rust. Motivated by the desire to explore Rust's potential in building high-performance, secure, and highly customizable networking solutions. Also, this project aims to serve as a valuable platform for education in network protocol design and implementation.

The current packet capture functionality is a foundational step towards this broader vision. By mastering eBPF and XDP for efficient data acquisition, the project is establishing the necessary groundwork to progress towards more complex network stack components. Future development will explore areas such as:

* **Integrated IP Layer**: Building an IP layer to manage network addressing, routing, and packet forwarding.

* **Rust-Native TCP Protocol Implementation**: Developing a robust and feature-complete TCP protocol implementation in Rust.


* **User-Space Networking library**: Creating a user-space networking library that leverages the custom TCP/IP stack, enabling the development of network applications directly in user space.


## Documentation

For now, please refer to the inline comments within the code for API details and usage information. As the project progresses, comprehensive documentation will be provided, including API references, usage guides, and design documents outlining the architecture of the Rust-based TCP/IP stack as it evolves.

## Contributing

Contributions to Rusty TCP are highly welcome! If you have ideas for improvements, bug fixes, or new features, especially in the areas of networking protocols, eBPF, or Rust systems programming, please contributeby reporting issues (bugs or feature requests) please open a detailed issue on the GitHub repository or submitting pull request with a clear description of your changes :)

## License

Rusty TCP is distributed under the MIT License. See the LICENSE file in the repository for the full license text.

## Contact

For any questions, suggestions, collaboration inquiries, or discussions related to Rusty TCP, please feel free to reach out:

Email: franco.kurte@gmail.com

LinkedIn: https://www.linkedin.com/in/franco-kurte-a4975b220/

Thank you!
