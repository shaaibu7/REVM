use crate::evm::state::state::State;

fn stop(state: &mut State<u8>) {
    state.stop_flag = true
}

fn add(state: &mut State<u8>) {
    let a = state.stack.pop().unwrap();
    let b = state.stack.pop().unwrap();

    let _ = state.stack.push(a + b);
    state.pc += 1;
    
}

