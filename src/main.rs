use std::fs::File;


enum Mnemonic {
    Add, Call, Cls, Drw,
    Jp, Ld, Or, Ret,
    Rnd, Se, Shl, Shr,
    Sknp, Skp, Sne, Sub,
    Subn, Sys, Xor,
}

enum Register {
    V0, V1, V2, V3,
    V4, V5, V6, V7,
    V8, V9, Va, Vb,
    Vc, Vd, Ve, Vf,
    Sp, Pc, I, St, Dt,
}

enum Token {
    Opcode(Mnemonic),
    Reg(Register),
    ImmAddr(u16),
    ImmByte(u8),
    ImmNibb(u8),
}

struct Stream {
    input: File,
}

impl Stream {
    fn new(input: File) -> Stream {
        Stream {
            input: input,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        None
    }
}

fn main() {
}
