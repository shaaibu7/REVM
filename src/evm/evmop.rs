use crate::evm::memory::memory::SimpleMemory;
use crate::evm::stackop::stack::Stack;
use crate::evm::storage::storage::Storage;
use std::vec::Vec;


pub mod evm {
    use super::*;

    #[derive(Debug)]
    pub struct Evm<T> {
        pub pc: usize,
        pub stack: Stack<T>,
        pub memory: SimpleMemory,
        pub storage: Storage,
        pub program: Vec<u8>,
        pub gas: u64,
        pub value: u64,
        pub calldata: Vec<u8>,
        pub stop_flag: bool,
        pub revert_flag: bool,
        pub returndata: Vec<u8>,
        pub logs: Vec<String>,
    }

    impl <T: std::fmt::Display> Evm<T> {
        pub fn new(
            program: Vec<u8>,
            gas: u64,
            value: u64,
            calldata: Option<Vec<u8>>,
        ) -> Self {
            Evm {
                pc: 0,
                stack: Stack::new(),
                memory: SimpleMemory::new(),
                storage: Storage::new(),
                program: program,
                gas,
                value,
                calldata: calldata.unwrap_or_else(|| Vec::new()),
                stop_flag: false,
                revert_flag: false,
                returndata: Vec::new(),
                logs: Vec::new(),
            }
        }

        pub fn peek(&self) -> Option<u8> {
            self.program.get(self.pc).cloned()
        }

        pub fn gas_dec(&mut self, amount: u64) -> Result<(), String> {
            if self.gas < amount {
                Err(String::from("Out of gas"))
            } else {
                self.gas -= amount;
                Ok(())
            }
        }

        pub fn should_execute_next_opcode(&self) -> bool {
            if self.pc >= self.program.len() {
                return false
            } 

            if self.stop_flag || self.revert_flag {
                return  false
            }

            true
        }

        // dummy functions
        pub fn stop(&mut self) {}
        pub fn add(&mut self) {}
        pub fn _push(&mut self) {}

        pub fn run(&mut self) {
            while self.should_execute_next_opcode() {
               let op = self.pc;

               match op {
                STOP => self.stop(),
                ADD => self.add(),
                PUSH1 => self._push(),
                _ => ()
               }
            }
        }

        pub fn reset(&mut self) {
            self.pc = 0;
            self.stack = Stack::new();
            self.memory = SimpleMemory::new();
            self.storage = Storage::new();
        }
    }

}