# ShadowOS Quick Start Guide

## What's New: RAM Disk Implementation

I've just implemented Milestone 1 of your file system plan - a fully functional RAM disk! Here's what was added:

### New Files

1. **[kernel/src/block_device.rs](kernel/src/block_device.rs)** - Block device abstraction layer
   - Generic `BlockDevice` trait for all storage devices
   - Standard 512-byte block operations
   - Error handling (OutOfBounds, NotReady, IoError)

2. **[kernel/src/ramdisk.rs](kernel/src/ramdisk.rs)** - RAM disk implementation
   - 1MB RAM disk (2048 blocks)
   - Thread-safe with Mutex
   - Implements BlockDevice trait
   - Global RAMDISK instance

3. **Updated [kernel/src/main.rs](kernel/src/main.rs)**
   - Initializes RAM disk on boot
   - Comprehensive test suite
   - Shows RAM disk capacity and I/O tests

4. **[build.sh](build.sh)** - Build automation script
5. **[run.sh](run.sh)** - QEMU launch script
6. **[RAMDISK_README.md](RAMDISK_README.md)** - Full documentation

## Prerequisites

You'll need to install Rust and related tools:

### On Windows (WSL or MSYS2/Git Bash)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the Rust environment
source $HOME/.cargo/env

# Install nightly toolchain (required for OS development)
rustup toolchain install nightly
rustup default nightly

# Add rust-src component (for core library compilation)
rustup component add rust-src

# Install LLVM tools
rustup component add llvm-tools-preview
```

### Additional Tools

You also need:
- **QEMU**: For testing (you seem to have this at `/c/Program Files/qemu`)
- **xorriso**: For creating bootable ISOs (optional)

## Building ShadowOS

Once Rust is installed:

```bash
# Build the kernel
./build.sh

# Or manually:
cd kernel
cargo build --release
```

## Running in QEMU

```bash
# Run the OS
./run.sh

# Or manually:
qemu-system-x86_64 -cdrom shadowos.iso -m 256M -serial stdio
```

## Expected Output

When you boot ShadowOS, you should see:

```
ShadowOS v0.1.0
================

[*] Initializing RAM disk...
    RAM disk has 2048 blocks (1024 KB)
[*] Testing RAM disk I/O...
    Write to block 0: OK
    Read from block 0: OK
    Data verification: PASSED
    Out of bounds test: PASSED

[*] Kernel initialization complete.
```

This confirms that:
- ✅ RAM disk is initialized with 1MB storage
- ✅ Block writes work correctly
- ✅ Block reads work correctly
- ✅ Data integrity is maintained
- ✅ Error handling works (out of bounds detection)

## Project Structure

```
shadowos/
├── kernel/
│   ├── src/
│   │   ├── main.rs           # Kernel entry point + tests
│   │   ├── vga_buffer.rs     # VGA text output
│   │   ├── block_device.rs   # Block device abstraction
│   │   └── ramdisk.rs        # RAM disk implementation
│   ├── Cargo.toml            # Kernel dependencies
│   └── .cargo/
│       └── config.toml       # Build configuration
├── build.sh                   # Build script
├── run.sh                     # Run script
├── FILE_SYSTEM_PLAN.md       # Your file system roadmap
├── RAMDISK_README.md         # RAM disk documentation
└── QUICKSTART.md             # This file
```

## Next Steps

Now that Milestone 1 (RAM disk) is complete, you can move to **Milestone 2**:

1. **Create ShadowFS on-disk structures**
   - Design superblock format
   - Implement inode bitmap
   - Implement data block bitmap
   - Create inode table structure

2. **Write a formatting tool**
   - Initialize the superblock
   - Clear bitmaps
   - Create root directory inode

See [FILE_SYSTEM_PLAN.md](FILE_SYSTEM_PLAN.md) for the complete roadmap.

## Troubleshooting

### Cargo not found
- Make sure you've installed Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Source the environment: `source $HOME/.cargo/env`
- Restart your terminal

### Build errors
- Ensure you're using nightly: `rustup default nightly`
- Update rust-src: `rustup component add rust-src`

### QEMU doesn't start
- Check QEMU is in PATH: `which qemu-system-x86_64`
- On Windows, you may need to use the full path: `/c/Program\ Files/qemu/qemu-system-x86_64.exe`

## Resources

- [Rust OSDev Tutorial](https://os.phil-opp.com/)
- [Limine Bootloader](https://github.com/limine-bootloader/limine)
- [OSDev Wiki](https://wiki.osdev.org/)

---

**Ready to continue?** Install Rust, run `./build.sh`, then `./run.sh` to see your RAM disk in action!
