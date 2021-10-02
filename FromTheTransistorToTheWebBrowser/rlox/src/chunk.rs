use std::ptr;

pub enum OpCode {
    OpReturn,
}

pub struct Chunk {
    count: i16,
    capacity: i16,
    code: *mut u8,
}

impl Chunk {
    pub fn init_chunk(chunk: *mut Chunk) {
        unsafe {
            (*chunk).count = 0;
            (*chunk).capacity = 0;
            (*chunk).code = ptr::null_mut();
        }
    }

    pub fn write_chunk(chunk: *mut Chunk, byte: u8) {
        unsafe {
            if (*chunk).capacity < (*chunk).count + 1 {
                let old_capacity = (*chunk).capacity;
                (*chunk).capacity = GROW_CAPACITY(old_capacity);
                (*chunk).code = GROW_ARRAY(u8, (*chunk).code, old_capacity, (*chunk).capacity);
            }
            (*chunk).code[((*chunk).count) as u8] = byte;
            (*chunk).code = (*chunk).code + 1;
        }
    }
}
