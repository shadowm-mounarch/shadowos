#!/bin/bash
# Build script for ShadowOS

set -e

echo "Building ShadowOS kernel..."

# Build the kernel
cd kernel
cargo build --release

# Copy kernel to boot directory
echo "Preparing ISO structure..."
cd ..
mkdir -p iso/boot

# Copy the kernel binary
cp target/x86_64-unknown-none/release/kernel iso/boot/kernel

# Copy Limine files
if [ ! -f "iso/boot/limine-bios.sys" ]; then
    echo "Warning: Limine boot files not found. Make sure to install Limine first."
fi

# Create ISO if xorriso is available
if command -v xorriso &> /dev/null; then
    echo "Creating ISO image..."
    xorriso -as mkisofs -b boot/limine-bios.sys \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot boot/limine/limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        iso -o shadowos.iso
    echo "ISO created: shadowos.iso"
else
    echo "xorriso not found. Skipping ISO creation."
fi

echo "Build complete!"
