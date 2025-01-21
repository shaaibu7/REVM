mod stack;

use stack::stackop::stack::Stack;
fn main() {
    let mut stack = Stack::new(); 

    stack.push(10).unwrap();
    stack.push(11).unwrap();
    stack.push(12).unwrap();  

    // Test for EVM stack overflow of 1024

    // for i in 1..1030 {
       
    //     stack.push(11).unwrap();
    //     println!("{}", i);
    // }

    println!("{}", stack.display());

    stack.pop().unwrap();
    println!("================================");
    println!("{}", stack.display());
}
