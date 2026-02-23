# ShadowOS File System Plan

This document outlines the plan for designing and implementing a file system for ShadowOS.

## 1. Overview and Goals

The initial goal is to implement a simple but robust file system that can handle basic file operations. The design should be modular to allow for future expansion and support for other file system types.

-   **Simplicity**: The first version should be simple to implement and debug.
-   **Extensibility**: The design should allow for adding more features and file systems in the future.
-   **Unix-like semantics**: The file system will expose a POSIX-like API to user-space applications (e.g., `open`, `read`, `write`, `close`, `ls`).

## 2. File System Choice: A Simple Inode-based File System ("ShadowFS")

We will implement a custom, minimalistic, inode-based file system inspired by the design of Unix file systems like `ext2`. This provides a good learning experience and a solid foundation for more advanced features. It will be named "ShadowFS".

Why not FAT32?
-   While the bootloader has FAT32 support, implementing our own file system gives us more control and avoids the complexities and patent issues of FAT.
-   An inode-based system is a more common design in modern operating systems.

## 3. On-Disk Layout

The disk will be partitioned with ShadowFS. The layout will be as follows:

1.  **Superblock**: A single block at a known location (likely the first block of the partition). It contains metadata about the entire file system, such as:
    *   Total number of blocks.
    *   Total number of inodes.
    *   Block size.
    *   A magic number to identify ShadowFS.
    *   Pointers to the start of the inode bitmap, data block bitmap, inode table, and data blocks.

2.  **Inode Bitmap**: A block of bits where each bit represents the allocation status of an inode (free or in use).

3.  **Data Block Bitmap**: A block of bits where each bit represents the allocation status of a data block.

4.  **Inode Table**: A contiguous area of blocks containing all the inodes. Each inode will store:
    *   File type (regular file, directory, etc.).
    *   File size.
    *   Timestamps (creation, modification, access).
    *   Permissions.
    *   User/Group ID.
    *   Pointers to data blocks (direct and indirect).

5.  **Data Blocks**: The remaining space on the partition, used to store file and directory content.

## 4. Key Data Structures (In-Memory)

The kernel will maintain in-memory representations of the on-disk structures.

-   **`struct superblock`**: In-memory copy of the superblock.
-   **`struct inode`**: In-memory representation of an on-disk inode.
-   **`struct file`**: Represents an open file, containing a pointer to its inode, the current file offset, and access flags (read/write).
-   **`struct directory_entry`**: A structure to represent an entry within a directory. It will contain an inode number and a file name.

## 5. Virtual File System (VFS)

To support multiple file systems in the future, we will build a VFS layer from the start.

-   The VFS will define a common interface for all file systems.
-   It will provide a generic API for user-space applications.
-   Key VFS objects: `vfs_node` (similar to inode), `file_operations` (function pointers for read, write, etc.).
-   Initially, we will only have one implementation of this interface: ShadowFS.
-   This will allow us to mount different file systems under a unified directory tree (e.g., `/`, `/mnt/usb`).

## 6. Implementation Milestones

### Milestone 1: Basic Disk Driver and Block Device Abstraction ✅ COMPLETED

-   [x] Implement a driver for a simple disk device (e.g., RAM disk or a simple IDE driver).
-   [x] Create a block device abstraction that provides `read_block` and `write_block` functions.

**Implementation Details:**
- Created `BlockDevice` trait in `kernel/src/block_device.rs`
- Implemented RAM disk in `kernel/src/ramdisk.rs` with 1MB storage (2048 blocks)
- Added comprehensive tests in `kernel/src/main.rs`
- See [RAMDISK_README.md](RAMDISK_README.md) for full documentation

### Milestone 2: ShadowFS On-Disk Structures

-   [ ] Write a tool to create a ShadowFS file system image from a host machine. This tool will:
    -   Create a superblock.
    -   Initialize inode and data block bitmaps.
    -   Create a root directory inode (`/`).
-   [ ] Write the kernel code to read and parse the superblock and other on-disk structures.

### Milestone 3: Inode and Data Block Management

-   [ ] Implement functions to allocate and free inodes using the inode bitmap.
-   [ ] Implement functions to allocate and free data blocks using the data block bitmap.

### Milestone 4: VFS Layer and First File Operations

-   [ ] Implement the basic VFS structures (`vfs_node`, `file_operations`).
-   [ ] Implement `mount()` to mount the root file system.
-   [ ] Implement `read()` for a single-block file.
-   [ ] Implement directory lookups.

### Milestone 5: Expanding File Operations

-   [ ] Implement `write()` for files. This will involve handling multi-block files and indirect pointers.
-   [ ] Implement file creation (`creat()`) and deletion (`unlink()`).
-   [ ] Implement directory creation (`mkdir()`) and deletion (`rmdir()`).

### Milestone 6: User-space Integration

-   [ ] Create system calls for the VFS functions (`open`, `read`, `write`, `close`, etc.).
-   [ ] Test the file system with simple user-space programs.

## 7. Future Work

-   Journaling for crash-resistance.
-   Support for more file system types (e.g., FAT32, ext2).
-   Caching of blocks in memory (buffer cache).
-   Support for special files (devices, pipes).
