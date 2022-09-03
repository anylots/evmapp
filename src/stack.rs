use crate::types::Error;
use ethereum_types::{BigEndianHash, H256, U256};

pub struct Stack(Vec<U256>);

impl Stack{

    pub fn new()->Self{
        Self(Vec::with_capacity(1024usize))
    }

    pub fn push_u256(&mut self, value:U256)->Result<(),Error>{
        if self.0.len()<1024{
            Ok(self.0.push(value))
        }else{
            Err(Error::StackOverflow)
        }
    }

    pub fn pop(&mut self)-> U256{
        return self.0.pop().unwrap()
    }
}
