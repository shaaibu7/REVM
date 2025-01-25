mod memory;
mod stackop;
mod storage;

use std::vec::Vec;


pub mod state {

    pub struct State {
        pc: usize,
        stack: Stack,
    }
}