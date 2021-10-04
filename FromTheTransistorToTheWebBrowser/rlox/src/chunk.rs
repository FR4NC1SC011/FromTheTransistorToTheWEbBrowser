use crate::GROW_CAPACITY;

pub enum OpCode {
    OpReturn,
}

pub struct Chunk {
    count: i16,
    capacity: Option<i16>,
    code: Vec<u8>,
}

impl Chunk {
    pub fn init_chunk(chunk: Chunk) {
        chunk.count = 0;
        chunk.capacity = None;
        chunk.code = Vec::with_capacity(0);
    }

    pub fn write_chunk(&mut self, chunk: Chunk, byte: u8) {
        if chunk.capacity < chunk.count + 1 {
            let old_capacity: i16 = chunk.capacity;
            chunk.capacity = GROW_CAPACITY!(old_capacity);
            chunk.code = GROW_ARRAY(u8, chunk.code, old_capacity, chunk.capacity);
        }
        chunk.code[(chunk.count) as usize] = byte;
        chunk.count += 1;
    }
}
