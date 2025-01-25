mod evm;

use evm::stackop::stack::Stack;
use evm::memory::memory::SimpleMemory;
use evm::storage::storage::Storage;

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

}
