use crate::evm::memory::memory::SimpleMemory;
use crate::evm::stackop::stack::Stack;
use crate::evm::storage::storage::Storage;
use std::vec::Vec;


pub mod state {
    use super::*;

    #[derive(Debug)]
    pub struct State<T> {
        pub pc: usize,
        pub stack: Stack<T>,
        pub memory: SimpleMemory,
        pub storage: Storage,
        pub sender: String,
        pub program: String,
        pub gas: u64,
        pub value: u64,
        pub calldata: Vec<u8>,
        pub stop_flag: bool,
        pub revert_flag: bool,
        pub returndata: Vec<u8>,
        pub logs: Vec<String>,
    }

    impl <T: std::fmt::Display> State<T> {
        pub fn new(
            sender: String,
            program: String,
            gas: u64,
            value: u64,
            calldata: Option<Vec<u8>>,
        ) -> Self {
            State {
                pc: 0,
                stack: Stack::new(),
                memory: SimpleMemory::new(),
                storage: Storage::new(),
                sender,
                program,
                gas,
                value,
                calldata: calldata.unwrap_or_else(|| Vec::new()),
                stop_flag: false,
                revert_flag: false,
                returndata: Vec::new(),
                logs: Vec::new(),
            }
        }
    }

}