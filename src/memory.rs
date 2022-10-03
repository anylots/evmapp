use ethereum_types::U256;

use crate::types::Error;

const MAX_SIZE: usize = 65536;
const WORD_SIZE: usize = 32;

pub struct Memory(Vec<u8>);

impl Memory {
    pub fn new() -> Self {
        Self(Vec::with_capacity(MAX_SIZE))
    }

    /// Resizes the memory buffer to allow accessing the given location.
    pub fn resize_for(&mut self, key: usize) -> Result<(), Error> {
        let bound = (((key - 1) / 32) + 1) * 32;
        if bound > self.0.len() {
            self.0.resize(bound, 0);
        }
        Ok(())
    }

    pub fn mstore(&mut self, key: usize, value: U256) -> Result<(), Error> {
        if key >= MAX_SIZE - WORD_SIZE {
            return Err(Error::MemoryOverflow);
        }
        self.resize_for(key + WORD_SIZE)?;
        Ok(value.to_big_endian(&mut self.0[key..key + WORD_SIZE]))
    }

    pub fn mview(&mut self, start: usize, len: usize) -> Result<&[u8], Error> {
        let end = start.checked_add(len).ok_or(Error::MemoryOutOfBound)?;
        if end <= self.0.len() {
            Ok(&self.0[start..end])
        } else {
            Err(Error::MemoryOutOfBound)
        }
    }
}
