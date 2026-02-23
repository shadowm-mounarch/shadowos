use core::fmt;

/// Standard block size (512 bytes, common for disk sectors)
pub const BLOCK_SIZE: usize = 512;

/// Result type for block device operations
pub type BlockResult<T> = Result<T, BlockError>;

/// Errors that can occur during block device operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockError {
    /// The requested block is out of bounds
    OutOfBounds,
    /// The device is not ready
    NotReady,
    /// A general I/O error occurred
    IoError,
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockError::OutOfBounds => write!(f, "Block out of bounds"),
            BlockError::NotReady => write!(f, "Device not ready"),
            BlockError::IoError => write!(f, "I/O error"),
        }
    }
}

/// Trait for block devices that can read and write fixed-size blocks
pub trait BlockDevice {
    /// Read a single block from the device
    ///
    /// # Arguments
    /// * `block_id` - The block number to read
    /// * `buffer` - Buffer to store the block data (must be BLOCK_SIZE bytes)
    fn read_block(&self, block_id: u64, buffer: &mut [u8; BLOCK_SIZE]) -> BlockResult<()>;

    /// Write a single block to the device
    ///
    /// # Arguments
    /// * `block_id` - The block number to write
    /// * `buffer` - Buffer containing the block data (must be BLOCK_SIZE bytes)
    fn write_block(&mut self, block_id: u64, buffer: &[u8; BLOCK_SIZE]) -> BlockResult<()>;

    /// Get the total number of blocks in this device
    fn block_count(&self) -> u64;

    /// Get the block size (always BLOCK_SIZE for now)
    fn block_size(&self) -> usize {
        BLOCK_SIZE
    }
}
