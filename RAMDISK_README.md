# RAM Disk Implementation

This document describes the RAM disk implementation for ShadowOS (Milestone 1 of the File System Plan).

## Overview

The RAM disk provides a simple block device that stores data in memory. It's useful for:
- Testing file system code without hardware dependencies
- Temporary storage during kernel operations
- Learning block device abstraction concepts

## Architecture

### Components

1. **Block Device Abstraction** ([block_device.rs](kernel/src/block_device.rs))
   - `BlockDevice` trait: Generic interface for all block devices
   - `BLOCK_SIZE`: Standard 512-byte blocks
   - `BlockError`: Error types for device operations
   - Methods: `read_block()`, `write_block()`, `block_count()`, `block_size()`

2. **RAM Disk Implementation** ([ramdisk.rs](kernel/src/ramdisk.rs))
   - `RamDisk` struct: Implements `BlockDevice` trait
   - Static storage: 1MB (2048 blocks) allocated at compile time
   - Thread-safe: Uses `Mutex` for safe concurrent access
   - Global instance: `RAMDISK` accessible throughout the kernel

### Memory Layout

```
RAMDISK_STORAGE: [u8; 1MB]
├─ Block 0:    [0..512]
├─ Block 1:    [512..1024]
├─ Block 2:    [1024..1536]
│  ...
└─ Block 2047: [1047552..1048064]
```

## Usage

### Initialization

Call `ramdisk::init()` during kernel startup:

```rust
// In main.rs _start() function
ramdisk::init();
```

### Reading a Block

```rust
use crate::ramdisk::RAMDISK;
use crate::block_device::{BlockDevice, BLOCK_SIZE};

let mut buffer = [0u8; BLOCK_SIZE];
let mut ramdisk = RAMDISK.lock();

if let Some(ref mut disk) = *ramdisk {
    match disk.read_block(block_id, &mut buffer) {
        Ok(_) => {
            // Use buffer data
        },
        Err(e) => {
            // Handle error
        }
    }
}
```

### Writing a Block

```rust
let mut buffer = [0u8; BLOCK_SIZE];
// Fill buffer with data...

let mut ramdisk = RAMDISK.lock();
if let Some(ref mut disk) = *ramdisk {
    match disk.write_block(block_id, &buffer) {
        Ok(_) => {
            // Write successful
        },
        Err(e) => {
            // Handle error
        }
    }
}
```

## Testing

The kernel includes built-in tests in [main.rs:28](kernel/src/main.rs#L28) that:
1. Display RAM disk capacity
2. Write a test string to block 0
3. Read back the data from block 0
4. Verify data integrity
5. Test out-of-bounds error handling

Run these tests by building and running the kernel:

```bash
./build.sh
./run.sh
```

Expected output:
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

## Error Handling

The `BlockError` enum provides three error types:

- `OutOfBounds`: Block ID exceeds device capacity
- `NotReady`: Device not initialized
- `IoError`: General I/O failure

All operations return `BlockResult<T>` which is `Result<T, BlockError>`.

## Future Enhancements

- [ ] Add async block I/O support
- [ ] Implement block caching
- [ ] Add wear leveling (if backing store changes to flash)
- [ ] Support multiple RAM disk instances
- [ ] Add performance statistics
- [ ] Implement scatter-gather I/O

## Next Steps (Milestone 2)

With the RAM disk complete, the next milestone is to implement ShadowFS on-disk structures:
1. Create a tool to format the RAM disk with ShadowFS
2. Implement superblock structure
3. Add inode and data block bitmaps
4. Create root directory

See [FILE_SYSTEM_PLAN.md](FILE_SYSTEM_PLAN.md) for the complete roadmap.
