// src/xdp/bpf/xdp.bpf.c
#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <bpf/bpf_helpers.h>

#define MAX_FRAME_SIZE 9000 // Maximum Ethernet frame size (jumbo frames)

// Define the ring buffer map to send frames to user space
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024); // 256 KB ring buffer
} frame_ringbuf SEC(".maps");

// Define a constant size
#define RINGBUF_FRAME_SIZE 2048  // Choose a reasonable fixed size for your needs

SEC("xdp")
int xdp_frame_capture(struct xdp_md *ctx) {
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;
    __u32 frame_size = data_end - data;

    if (frame_size > MAX_FRAME_SIZE) {
        bpf_printk("XDP: Dropping oversized frame (%u bytes)", frame_size);
        return XDP_DROP;
    }

    // Use a constant size for ringbuf reservation
    // https://elixir.bootlin.com/linux/v6.14-rc6/source/kernel/bpf/ringbuf.c#L477
    void *ringbuf_data = bpf_ringbuf_reserve(&frame_ringbuf, RINGBUF_FRAME_SIZE, 0);
    if (!ringbuf_data) {
        bpf_printk("XDP: Failed to reserve ring buffer space");
        return XDP_PASS;
    }

        // Fix it with this:
    __u32 read_size = frame_size;
    if (read_size > RINGBUF_FRAME_SIZE)
        read_size = RINGBUF_FRAME_SIZE;
        
    // Ensure verifier can prove read_size is bounded and non-negative
    read_size &= 0x7FF;  // Equivalent to min(read_size, 2047) - matches your RINGBUF_FRAME_SIZE

    if (bpf_probe_read_kernel(ringbuf_data, read_size, data) < 0) {
        bpf_printk("XDP: Failed to read frame data into ring buffer");
        bpf_ringbuf_discard(ringbuf_data, 0);
        return XDP_PASS;
    }

    // You'll need to store the actual frame size for userspace to know
    // Store it at the beginning or end of the buffer, or use a header struct
    // For example, store it as the first 4 bytes
    *(__u32 *)ringbuf_data = frame_size;
    
    bpf_printk("XDP: Captured frame of size %u bytes", frame_size);
    bpf_ringbuf_submit(ringbuf_data, 0);
    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
