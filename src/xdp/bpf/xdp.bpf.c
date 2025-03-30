// src/xdp/bpf/xdp.bpf.c
#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <bpf/bpf_helpers.h>

#define MAX_FRAME_SIZE 9000
#define RINGBUF_FRAME_SIZE (MAX_FRAME_SIZE + 4)

#ifndef RINGBUF_MAX_ENTRIES
#define RINGBUF_MAX_ENTRIES (256 * 1024)
#endif

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, RINGBUF_MAX_ENTRIES);
} frame_ringbuf SEC(".maps");

struct ringbuf_entry {
    __u32 frame_size;
    __u8 data[MAX_FRAME_SIZE];
};

static __always_inline int capture_frame(struct xdp_md *ctx, int debug) {
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;
    __u32 frame_size = data_end - data;

    if (frame_size > MAX_FRAME_SIZE) {
        if (debug) bpf_printk("XDP: Dropping oversized frame (%u bytes)", frame_size);
        return XDP_DROP;
    }

    struct ringbuf_entry *entry = bpf_ringbuf_reserve(&frame_ringbuf, RINGBUF_FRAME_SIZE, 0);
    if (!entry) {
        if (debug) bpf_printk("XDP: Failed to reserve ring buffer space");
        return XDP_PASS;
    }

    // Explicitly bound read_size to MAX_FRAME_SIZE (9000) to satisfy verifier
    entry->frame_size = frame_size;
    __u32 read_size = frame_size;
    if (read_size > MAX_FRAME_SIZE) read_size = MAX_FRAME_SIZE;  // Redundant but for clarity
    read_size &= (MAX_FRAME_SIZE - 1);  // 0–8999, ensures non-negative and bounded

    if (bpf_probe_read_kernel(entry->data, read_size, data) < 0) {
        if (debug) bpf_printk("XDP: Failed to read frame data into ring buffer");
        bpf_ringbuf_discard(entry, 0);
        return XDP_PASS;
    }

    if (debug) bpf_printk("XDP: Captured frame of size %u bytes", frame_size);
    bpf_ringbuf_submit(entry, 0);
    return XDP_PASS;
}

SEC("xdp")
int xdp_frame_capture(struct xdp_md *ctx) {
    return capture_frame(ctx, 0);
}

SEC("xdp_debug")
int xdp_frame_capture_debug(struct xdp_md *ctx) {
    return capture_frame(ctx, 1);
}

char _license[] SEC("license") = "GPL";
