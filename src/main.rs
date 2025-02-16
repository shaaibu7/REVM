mod evm;

use crate::evm::opcodes::opcode;
use evm::stackop::stack::Stack;
use evm::memory::memory::SimpleMemory;
use evm::storage::storage::Storage;
use evm::state::state::State;





fn main() {
    //STACK TEST

    // let mut stack = Stack::new(); 

    // stack.push(10).unwrap();
    // stack.push(11).unwrap();
    // stack.push(12).unwrap();  

    // stack.pop();

    // // Test for EVM stack overflow of 1024

    // // for i in 1..1030 {
       
    // //     stack.push(11).unwrap();
    // //     println!("{}", i);
    // // }

    // println!("{}", stack.display());

    // stack.pop().unwrap();
    // println!("================================");
    // println!("{}", stack.display());

    // MEMORY TEST

    // let mut memory = SimpleMemory::new();

    // memory.store(0, &[0x01, 0x02, 0x03, 0x0a]);

    // let data = memory.access(0, 4);
    // println!("{:?}", data);

    // let data = memory.load(0);
    // println!("{:?}", data);

    //STORAGE TEST

    // let mut storage = Storage::new();

    // storage.store("name", 77);
    // storage.store("age", 71);

    // let (warm, value1) = storage.load("name");
    // println!("{} {}", warm, value1);

    // let (warm1, value2) = storage.load("name");
    // println!("{} {}", warm1, value2);

    // let (warm3, value3) = storage.load("names");
    // println!("{} {}", warm3, value3);

    // STATE
    let sender = String::from("0x123");
    let program = "contract".to_string();
    let gas = 1000;
    let value = 50;

    let state: State<u32> = State::new(sender, program, gas, value, Some(vec![1, 2, 3]));

    println!("{:?}", state.sender);
    println!("{}", opcode::CREATE2)

}
