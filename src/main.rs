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
    ImmConst(u16),
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
        let token_str: String = String::from_utf8(input).unwrap_or_else(|err| {
            println!("Error: {:?}", err);
            "".to_owned()
        }).to_lowercase();

        if Stream::is_numeric(&token_str) {
            return Some(Token::ImmConst(token_str.parse::<u16>().unwrap()));
        }

        match token_str.as_str() {
            "add"   => Some(Token::Opcode(Mnemonic::Add)),
            "call"  => Some(Token::Opcode(Mnemonic::Call)),
            "cls"   => Some(Token::Opcode(Mnemonic::Cls)),
            "drw"   => Some(Token::Opcode(Mnemonic::Drw)),
            "jp"    => Some(Token::Opcode(Mnemonic::Jp)),
            "ld"    => Some(Token::Opcode(Mnemonic::Ld)),
            "or"    => Some(Token::Opcode(Mnemonic::Or)),
            "ret"   => Some(Token::Opcode(Mnemonic::Ret)),
            "rnd"   => Some(Token::Opcode(Mnemonic::Rnd)),
            "se"    => Some(Token::Opcode(Mnemonic::Se)),
            "shl"   => Some(Token::Opcode(Mnemonic::Shl)),
            "shr"   => Some(Token::Opcode(Mnemonic::Shr)),
            "sknp"  => Some(Token::Opcode(Mnemonic::Sknp)),
            "skp"   => Some(Token::Opcode(Mnemonic::Skp)),
            "sne"   => Some(Token::Opcode(Mnemonic::Sne)),
            "sub"   => Some(Token::Opcode(Mnemonic::Sub)),
            "subn"  => Some(Token::Opcode(Mnemonic::Subn)),
            "sys"   => Some(Token::Opcode(Mnemonic::Sys)),
            "xor"   => Some(Token::Opcode(Mnemonic::Xor)),
            _       => None
        }
    }

    fn is_separator(b: u8) -> bool {
        match b as char {
            ','|' '|'\n' => { true }
            _ => { false }
        }
    }

    fn is_numeric(input: &str) -> bool {
        if input.len() < 1 {
            return false;
        }

        let in_bytes = input.as_bytes();
        if !Stream::is_ascii_numeric(in_bytes[0]) {
            return false;
        }

        // hex
        if input.starts_with("0x") {
            for i in 2..(in_bytes.len()) {
                if !Stream::is_ascii_hex(in_bytes[i]) {
                    return false
                }
                return true
            }
        } else {
            for b in in_bytes {
                if !Stream::is_ascii_numeric(*b) {
                    return false
                }
                return true
            }
        }
        false
    }

    fn is_ascii_numeric(input: u8) -> bool {
        input >= 48 && input <= 57
    }

    fn is_ascii_hex(input: u8) -> bool {
        (input >= 48 && input <= 57) || ((input >= 97 && input <= 102) || (input >= 65 && input <= 70))
    }
}

fn main() {
}
