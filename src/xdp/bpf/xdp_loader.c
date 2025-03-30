// src/xdp/bpf/xdp_loader.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <fcntl.h>
#include <unistd.h>
#include <bpf/libbpf.h>
#include <bpf/bpf.h>
#include <linux/if_link.h>
#include <net/if.h>

extern unsigned char *get_xdp_bpf_o_data(void);
extern unsigned long get_xdp_bpf_o_len(void);

struct xdp_loader {
    struct bpf_object *obj;
    struct bpf_program *prog;
    struct bpf_map *ringbuf_map;
    int prog_fd;
    int ringbuf_fd;
    char *ifname;
};

struct xdp_loader *xdp_loader_init(const char *ifname) {
    struct xdp_loader *loader = calloc(1, sizeof(struct xdp_loader));
    if (!loader) {
        fprintf(stderr, "Failed to allocate loader: %s\n", strerror(errno));
        return NULL;
    }

    // Store ifname for cleanup
    loader->ifname = strdup(ifname);
    if (!loader->ifname) {
        fprintf(stderr, "Failed to duplicate ifname: %s\n", strerror(errno));
        free(loader);
        return NULL;
    }

    unsigned char *xdp_bpf_o_data = get_xdp_bpf_o_data();
    unsigned long xdp_bpf_o_len = get_xdp_bpf_o_len();

    loader->obj = bpf_object__open_mem(xdp_bpf_o_data, xdp_bpf_o_len, NULL);
    if (libbpf_get_error(loader->obj)) {
        fprintf(stderr, "Failed to open eBPF object from memory: %s\n", strerror(errno));
        free(loader->ifname);
        free(loader);
        return NULL;
    }

    // Find and enable the target program
    loader->prog = bpf_object__find_program_by_name(loader->obj, "xdp_frame_capture");
    if (!loader->prog) {
        fprintf(stderr, "Failed to find eBPF program 'xdp_frame_capture'\n");
        bpf_object__close(loader->obj);
        free(loader->ifname);
        free(loader);
        return NULL;
    }

    // Disable the debug program to prevent loading issues
    struct bpf_program *debug_prog = bpf_object__find_program_by_name(loader->obj, "xdp_frame_capture_debug");
    if (debug_prog) {
        bpf_program__set_autoload(debug_prog, false);
    }

    if (bpf_object__load(loader->obj)) {
        fprintf(stderr, "Failed to load eBPF object: %s\n", strerror(errno));
        bpf_object__close(loader->obj);
        free(loader->ifname);
        free(loader);
        return NULL;
    }

    loader->ringbuf_map = bpf_object__find_map_by_name(loader->obj, "frame_ringbuf");
    if (!loader->ringbuf_map) {
        fprintf(stderr, "Failed to find ring buffer map 'frame_ringbuf'\n");
        bpf_object__close(loader->obj);
        free(loader->ifname);
        free(loader);
        return NULL;
    }

    loader->prog_fd = bpf_program__fd(loader->prog);
    loader->ringbuf_fd = bpf_map__fd(loader->ringbuf_map);

    int ifindex = if_nametoindex(ifname);
    if (!ifindex) {
        fprintf(stderr, "Failed to get ifindex for %s: %s\n", ifname, strerror(errno));
        bpf_object__close(loader->obj);
        free(loader->ifname);
        free(loader);
        return NULL;
    }

    if (bpf_xdp_attach(ifindex, loader->prog_fd, XDP_FLAGS_SKB_MODE, NULL) < 0) {
        fprintf(stderr, "Failed to attach XDP program to %s: %s\n", ifname, strerror(errno));
        bpf_object__close(loader->obj);
        free(loader->ifname);
        free(loader);
        return NULL;
    }

    printf("XDP program attached to %s\n", ifname);
    return loader;
}

void xdp_loader_cleanup(struct xdp_loader *loader) {
    if (loader) {
        if (loader->ifname) {
            bpf_xdp_detach(if_nametoindex(loader->ifname), XDP_FLAGS_SKB_MODE, NULL);
            free(loader->ifname);
        }
        bpf_object__close(loader->obj);
        free(loader);
    }
}

int xdp_loader_get_ringbuf_fd(struct xdp_loader *loader) {
    return loader ? loader->ringbuf_fd : -1;
}
