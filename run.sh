#!/bin/bash
# Run ShadowOS in QEMU

# Default to the ISO if it exists, otherwise run kernel directly
if [ -f "shadowos.iso" ]; then
    echo "Running ShadowOS from ISO..."
    qemu-system-x86_64 \
        -cdrom shadowos.iso \
        -m 256M \
        -serial stdio \
        -d int,cpu_reset \
        -D qemu.log
else
    echo "ISO not found. Please run ./build.sh first."
    exit 1
fi
