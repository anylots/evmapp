use ethereum_types::U256;

pub trait Database {
    fn get(&self, key: U256) -> U256;

    fn set(&mut self, key: U256, value: U256);
}
