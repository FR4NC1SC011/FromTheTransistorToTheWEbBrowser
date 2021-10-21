use crate::memory;
use std::alloc::*;
use std::ptr::NonNull;

pub type ChunkPtr = NonNull<u8>;
pub type ChunkSize = usize;

pub struct Chunk {
    count: i16,
    capacity: ChunkSize,
    code: ChunkPtr,
}

pub enum OpCode {
    OpReturn,
}

pub enum ChunkError {
    BadRequest,
    OOM,
}

impl Chunk {
    pub fn init_chunk(&mut self, size: ChunkSize) -> Result<Chunk, ChunkError> {
        if !(size & (size - 1) == 0) {
            return Err(ChunkError::BadRequest);
        }

        Ok(Chunk {
            count: 0,
            capacity: size,
            code: Chunk::alloc_chunk(size)?,
        })

        // chunk.count = 0;
        // chunk.capacity = 0;
        // chunk.code = Vec::with_capacity(0);
    }

    pub fn alloc_chunk(size: ChunkSize) -> Result<ChunkPtr, ChunkError> {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);

            let ptr = alloc(layout);
            if ptr.is_null() {
                Err(ChunkError::OOM)
            } else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }
    }

    pub fn write_chunk(&mut self, mut chunk: Chunk, byte: u8) {
        if chunk.capacity < chunk.count + 1 {
            let old_capacity = chunk.capacity;
            chunk.capacity = memory::grow_capacity(old_capacity);
            // chunk.code = GROW_ARRAY(chunk.code, old_capacity, chunk.capacity);
            chunk.code = Vec::with_capacity(chunk.capacity as usize);
        }
        chunk.code[(chunk.count) as usize] = byte;
        chunk.count += 1;
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.code.as_ptr()
    }
}
