use std::fs::File;
use std::io::Bytes;


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
    Sp, Pc, I, St, Dt, IVal,
}

enum Token {
    Opcode(Mnemonic),
    Reg(Register),
    ImmAddr(u16),
    ImmByte(u8),
    ImmNibb(u8),
}

struct Stream {
    input: Bytes<File>,
}

impl Stream {
    fn new(input: Bytes<File>) -> Stream {
        Stream {
            input: input,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let mut buffer: Vec<u8> = vec![];

        loop {
            let bs = self.input.next().map_or(None, |bm| match bm {
                Ok(b) => Some(b),
                _ => None
            });

            match bs {
                Some(b) => {
                    if Stream::is_separator(b) {
                        return Stream::create_token(buffer);
                    }
                    buffer.push(b);
                },
                None => {
                    return Stream::create_token(buffer);
                }
            }
        }
    }

    fn create_token(input: Vec<u8>) -> Option<Token> {
        None
    }

    fn is_separator(b: u8) -> bool {
        match b as char {
            ','|' '|'\n' => { true }
            _ => { false }
        }
    }
}

fn main() {
}
