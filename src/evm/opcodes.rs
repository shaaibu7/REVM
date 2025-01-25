pub mod opcode {
    // STOP
    const STOP: u8 = 0x0;
    
    // MATH
    const ADD: u8 = 0x1;
    const MUL: u8 = 0x2;
    const SUB: u8 = 0x3;
    const DIV: u8 = 0x4;
    const SDIV: u8 = 0x5;
    const MOD: u8 = 0x6;
    const SMOD: u8 = 0x7;
    const ADDMOD: u8 = 0x8;
    const MULMOD: u8 = 0x9;
    const EXP: u8 = 0xA;
    const SIGNEXTEND: u8 = 0xB;

    // COMPARISONS
    const LT: u8 = 0x10;
    const GT: u8 = 0x11;
    const SLT: u8 = 0x12;
    const SGT: u8 = 0x13;
    const EQ: u8 = 0x14;
    const ISZERO: u8 = 0x15;

    // lOGIC
    const AND: u8 = 0x16;
    const OR: u8 = 0x17;
    const XOR: u8 = 0x18;
    const NOT: u8 = 0x19;

    //BIT OPS
    const BYTE: u8 = 0x1A;
    const SHL: u8 = 0x1B;
    const SHR: u8 = 0x1C;
    const SAR: u8 = 0x1D;

    //MISC
    const SHA3: u8 = 0x20;

    // ETHEREUM STATE
    const ADDRESS: u8 = 0x30;
    const BALANCE: u8 = 0x31;
    const ORIGIN: u8 = 0x32;
    const CALLER: u8 = 0x33;
    const CALLVALUE: u8 = 0x34;
    const CALLDATALOAD: u8 = 0x35;
    const CALLDATASIZE: u8 = 0x36;
    const CALLDATACOPY: u8 = 0x37;
    const CODESIZE: u8 = 0x38;
    const CODECOPY: u8 = 0x39;
    const GASPRICE: u8 = 0x3A;
    const EXTCODESIZE: u8 = 0x3B;
    const EXTCODECOPY: u8 = 0x3C;
    const RETURNDATASIZE: u8 = 0x3D;
    const RETURNDATACOPY: u8 = 0x3E;
    const EXTCODEHASH: u8 = 0x3F;
    const BLOCKHASH: u8 = 0x40;
    const COINBASE: u8 = 0x41;
    const TIMESTAMP: u8 = 0x42;
    const NUMBER: u8 = 0x43;
    const DIFFICULTY: u8 = 0x44;
    const GASLIMIT: u8 = 0x45;
    const CHAINID: u8 = 0x46;
    const SELFBALANCE: u8 = 0x47;
    const BASEFEE: u8 = 0x48;

    // POP
    const POP: u8 = 0x50;


    //MEMORY
    const MLOAD: u8 = 0x51;
    const MSTORE: u8 = 0x52;
    const MSTORE8: u8 = 0x53;

    // STORAGE
    const SLOAD: u8 = 0x54;
    const SSTORE: u8 = 0x55;

    // JUMP
    const JUMP: u8 = 0x56;
    const JUMP1: u8 = 0x57;
    const PC: u8 = 0x58;
    const JUMPDEST: u8 = 0x5B;

    // TRANSIENT STORAGE
    const TLOAD: u8 = 0x5c;
    const TSTORE: u8 = 0x5d;

    // PUSH
    const PUSH1: u8 = 0x60;
    const PUSH2: u8 = 0x61;
    const PUSH3: u8 = 0x62;
    const PUSH4: u8 = 0x63;
    const PUSH5: u8 = 0x64;
    const PUSH6: u8 = 0x65;
    const PUSH7: u8 = 0x66;
    const PUSH8: u8 = 0x67;
    const PUSH9: u8 = 0x68;
    const PUSH10: u8 = 0x69;
    const PUSH11: u8 = 0x6A;
    const PUSH12: u8 = 0x6B;
    const PUSH13: u8 = 0x6C;
    const PUSH14: u8 = 0x6D;
    const PUSH15: u8 = 0x6E;
    const PUSH16: u8 = 0x6F;
    const PUSH17: u8 = 0x70;
    const PUSH18: u8 = 0x71;
    const PUSH19: u8 = 0x72;
    const PUSH20: u8 = 0x73;
    const PUSH21: u8 = 0x74;
    const PUSH22: u8 = 0x75;
    const PUSH23: u8 = 0x76;
    const PUSH24: u8 = 0x77;
    const PUSH25: u8 = 0x78;
    const PUSH26: u8 = 0x79;
    const PUSH27: u8 = 0x7A;
    const PUSH28: u8 = 0x7B;
    const PUSH29: u8 = 0x7C;
    const PUSH30: u8 = 0x7D;
    const PUSH31: u8 = 0x7E;
    const PUSH32: u8 = 0x7F;

    // DUP
    const DUP1: u8 = 0x80;
    const DUP2: u8 = 0x81;
    const DUP3: u8 = 0x82;
    const DUP4: u8 = 0x83;
    const DUP5: u8 = 0x84;
    const DUP6: u8 = 0x85;
    const DUP7: u8 = 0x86;
    const DUP8: u8 = 0x87;
    const DUP9: u8 = 0x88;
    const DUP10: u8 = 0x89;
    const DUP11: u8 = 0x8A;
    const DUP12: u8 = 0x8B;
    const DUP13: u8 = 0x8C;
    const DUP14: u8 = 0x8D;
    const DUP15: u8 = 0x8E;
    const DUP16: u8 = 0x8F;

    // SWAP
    const SWAP1: u8 = 0x90;
    const SWAP2: u8 = 0x91;
    const SWAP3: u8 = 0x92;
    const SWAP4: u8 = 0x93;
    const SWAP5: u8 = 0x94;
    const SWAP6: u8 = 0x95;
    const SWAP7: u8 = 0x96;
    const SWAP8: u8 = 0x97;
    const SWAP9: u8 = 0x98;
    const SWAP10: u8 = 0x99;
    const SWAP11: u8 = 0x9A;
    const SWAP12: u8 = 0x9B;
    const SWAP13: u8 = 0x9C;
    const SWAP14: u8 = 0x9D;
    const SWAP15: u8 = 0x9E;
    const SWAP16: u8 = 0x9F;


    // LOG
    const LOG0: u8 = 0xA0;
    const LOG1: u8 = 0xA1;
    const LOG2: u8 = 0xA2;
    const LOG3: u8 = 0xA3;
    const LOG4: u8 = 0xA4;

    // CONTRACT

    pub const CREATE: u8 = 0xF0;
    const CALL: u8 = 0xF1;
    const CALLCODE: u8 = 0xF2;
    const RETURN: u8 = 0xF3;
    const DELEGATECALL: u8 = 0xF4;
    pub const CREATE2: u8 = 0xF5;
    const STATICCALL: u8 = 0xFA;
    const REVERT: u8 = 0xFD;
    const INVALID: u8 = 0xFE;
    const SELFDESTRUCT: u8 = 0xFF;


    

}