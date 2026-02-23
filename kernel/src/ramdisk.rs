use crate::block_device::{BlockDevice, BlockError, BlockResult, BLOCK_SIZE};
use spin::Mutex;

/// A simple RAM disk that stores blocks in memory
///
/// This is useful for testing and as a temporary storage area.
/// The storage is backed by a static array allocated at compile time.
pub struct RamDisk {
    /// The actual storage for the blocks
    storage: &'static mut [u8],
    /// Number of blocks in this RAM disk
    block_count: u64,
}

impl RamDisk {
    /// Create a new RAM disk from a mutable slice
    ///
    /// # Arguments
    /// * `storage` - A mutable slice that will back the RAM disk
    ///
    /// # Panics
    /// Panics if the storage size is not a multiple of BLOCK_SIZE
    pub fn new(storage: &'static mut [u8]) -> Self {
        assert!(
            storage.len() % BLOCK_SIZE == 0,
            "RAM disk storage must be a multiple of block size"
        );

        let block_count = (storage.len() / BLOCK_SIZE) as u64;

        RamDisk {
            storage,
            block_count,
        }
    }

    /// Get a reference to a specific block's data
    fn get_block(&self, block_id: u64) -> BlockResult<&[u8]> {
        if block_id >= self.block_count {
            return Err(BlockError::OutOfBounds);
        }

        let start = (block_id as usize) * BLOCK_SIZE;
        let end = start + BLOCK_SIZE;

        Ok(&self.storage[start..end])
    }

    /// Get a mutable reference to a specific block's data
    fn get_block_mut(&mut self, block_id: u64) -> BlockResult<&mut [u8]> {
        if block_id >= self.block_count {
            return Err(BlockError::OutOfBounds);
        }

        let start = (block_id as usize) * BLOCK_SIZE;
        let end = start + BLOCK_SIZE;

        Ok(&mut self.storage[start..end])
    }
}

impl BlockDevice for RamDisk {
    fn read_block(&self, block_id: u64, buffer: &mut [u8; BLOCK_SIZE]) -> BlockResult<()> {
        let block_data = self.get_block(block_id)?;
        buffer.copy_from_slice(block_data);
        Ok(())
    }

    fn write_block(&mut self, block_id: u64, buffer: &[u8; BLOCK_SIZE]) -> BlockResult<()> {
        let block_data = self.get_block_mut(block_id)?;
        block_data.copy_from_slice(buffer);
        Ok(())
    }

    fn block_count(&self) -> u64 {
        self.block_count
    }
}

// Define a static storage area for the RAM disk
// This allocates 1MB of space (2048 blocks of 512 bytes each)
const RAMDISK_SIZE: usize = 1024 * 1024; // 1 MB
static mut RAMDISK_STORAGE: [u8; RAMDISK_SIZE] = [0; RAMDISK_SIZE];

/// Global RAM disk instance wrapped in a mutex for thread safety
pub static RAMDISK: Mutex<Option<RamDisk>> = Mutex::new(None);

/// Initialize the global RAM disk
///
/// This should be called once during kernel initialization
pub fn init() {
    let storage = unsafe { &mut RAMDISK_STORAGE };
    let ramdisk = RamDisk::new(storage);
    *RAMDISK.lock() = Some(ramdisk);
}
