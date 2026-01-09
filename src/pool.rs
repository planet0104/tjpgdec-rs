//! Memory pool implementation
//! 
//! Linear memory allocator for workspace allocation.
//!
//! ## Example
//!
//! ```c
//! static void* alloc_pool(JDEC* jd, size_t ndata) {
//!     ndata = (ndata + 3) & ~3;  // 4-byte alignment
//!     if (jd->sz_pool >= ndata) {
//!         jd->sz_pool -= ndata;
//!         rp = (char*)jd->pool;
//!         jd->pool = (void*)(rp + ndata);
//!     }
//!     return rp;
//! }
//! ```

use core::mem;

/// Memory pool for workspace allocation
/// 
/// Simple linear allocator with the following characteristics:
/// - Allocates sequentially from buffer start
/// - 8-byte alignment
/// - No individual deallocation (whole pool released together)
pub struct MemoryPool<'a> {
    /// Remaining available memory buffer
    buffer: &'a mut [u8],
    /// Current allocation position
    offset: usize,
}

impl<'a> MemoryPool<'a> {
    /// Create a new memory pool
    /// 
    /// # Example
    /// 
    /// ```
    /// use tjpgdec_rs::MemoryPool;
    /// 
    /// let mut workspace = vec![0u8; 10240];
    /// let mut pool = MemoryPool::new(&mut workspace);
    /// ```
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            buffer,
            offset: 0,
        }
    }

    /// Allocate memory from the pool
    /// 
    /// Uses 8-byte alignment and returns `None` if insufficient memory.
    pub fn alloc(&mut self, size: usize) -> Option<&'a mut [u8]> {
        self.alloc_aligned(size, 8)
    }

    /// Allocate memory with specified alignment
    pub fn alloc_aligned(&mut self, size: usize, align: usize) -> Option<&'a mut [u8]> {
        // 确保当前偏移量对齐
        let align_mask = align - 1;
        let aligned_offset = (self.offset + align_mask) & !align_mask;
        
        // 对齐大小
        let aligned_size = (size + align_mask) & !align_mask;
        
        let remaining = self.buffer.len() - aligned_offset;
        if remaining < aligned_size {
            return None;
        }

        let start = aligned_offset;
        self.offset = aligned_offset + aligned_size;

        // 使用unsafe来返回带有'a生命周期的切片
        // 这是安全的，因为我们保证不会重叠分配
        unsafe {
            let ptr = self.buffer.as_mut_ptr().add(start);
            Some(core::slice::from_raw_parts_mut(ptr, size))
        }
    }

    /// Allocate and initialize memory to zero
    pub fn alloc_zeroed(&mut self, size: usize) -> Option<&'a mut [u8]> {
        let slice = self.alloc(size)?;
        slice.fill(0);
        Some(slice)
    }

    /// Allocate typed array
    /// 
    /// # Safety
    /// 
    /// Type T's alignment requirement must not exceed 8 bytes.
    pub fn alloc_slice<T: Copy + Default>(&mut self, count: usize) -> Option<&'a mut [T]> {
        let size = count * mem::size_of::<T>();
        let slice = self.alloc(size)?;
        
        // 将字节切片转换为类型化切片
        let ptr = slice.as_mut_ptr() as *mut T;
        unsafe {
            let typed_slice = core::slice::from_raw_parts_mut(ptr, count);
            // 初始化为默认值
            for item in typed_slice.iter_mut() {
                *item = T::default();
            }
            Some(typed_slice)
        }
    }

    /// Allocate u8 array
    pub fn alloc_u8(&mut self, count: usize) -> Option<&'a mut [u8]> {
        self.alloc_zeroed(count)
    }

    /// Allocate u16 array
    pub fn alloc_u16(&mut self, count: usize) -> Option<&'a mut [u16]> {
        self.alloc_slice(count)
    }

    /// Allocate i32 array
    pub fn alloc_i32(&mut self, count: usize) -> Option<&'a mut [i32]> {
        self.alloc_slice(count)
    }

    /// Allocate i16 array
    pub fn alloc_i16(&mut self, count: usize) -> Option<&'a mut [i16]> {
        self.alloc_slice(count)
    }

    /// Get remaining available bytes
    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.offset
    }

    /// Get used bytes
    pub fn used(&self) -> usize {
        self.offset
    }

    /// Get total capacity
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Reset pool (release all allocations)
    pub fn reset(&mut self) {
        self.offset = 0;
    }
}


/// Recommended workspace size
/// 
/// Sufficient for most JPEG images, including with fast-decode-2 feature.
pub const RECOMMENDED_POOL_SIZE: usize = 10240;

/// Minimum workspace size
/// 
/// For small images or extremely memory-constrained environments.
pub const MINIMUM_POOL_SIZE: usize = 4096;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_basic() {
        let mut buffer = [0u8; 1024];
        let mut pool = MemoryPool::new(&mut buffer);

        let slice1 = pool.alloc(100).unwrap();
        assert_eq!(slice1.len(), 100);
        assert_eq!(pool.used(), 104);  // 100 aligned to 8 = 104

        let slice2 = pool.alloc(50).unwrap();
        assert_eq!(slice2.len(), 50);
        assert_eq!(pool.used(), 160);  // 104 + 56 (50 aligned to 8)
    }

    #[test]
    fn test_alloc_alignment() {
        let mut buffer = [0u8; 1024];
        let mut pool = MemoryPool::new(&mut buffer);

        pool.alloc(1).unwrap();
        assert_eq!(pool.used(), 8);  // 1 aligned to 8

        pool.alloc(5).unwrap();
        assert_eq!(pool.used(), 16);  // 8 + 8 (5 aligned to 8)
    }

    #[test]
    fn test_alloc_typed() {
        let mut buffer = [0u8; 1024];
        let mut pool = MemoryPool::new(&mut buffer);

        let u16_slice = pool.alloc_u16(10).unwrap();
        assert_eq!(u16_slice.len(), 10);
        assert_eq!(pool.used(), 24);  // 20 aligned to 8 = 24

        let i32_slice = pool.alloc_i32(5).unwrap();
        assert_eq!(i32_slice.len(), 5);
        assert_eq!(pool.used(), 48);  // 24 + 24 (20 aligned to 8)
    }

    #[test]
    fn test_alloc_fail() {
        let mut buffer = [0u8; 128];
        let mut pool = MemoryPool::new(&mut buffer);

        assert!(pool.alloc(50).is_some());  // uses 56 bytes
        assert!(pool.alloc(50).is_some());  // uses another 56 bytes = 112 total
        assert!(pool.alloc(20).is_none());  // 128 - 112 = 16, not enough for 20 (needs 24 aligned)
    }
}
