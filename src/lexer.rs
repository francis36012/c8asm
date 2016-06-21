use std::fs::File;
use std::io::Bytes;


#[derive(Debug)]
pub enum Mnemonic {
    Add, Call, Cls, Drw,
    Jp, Ld, Or, Ret,
    Rnd, Se, Shl, Shr,
    Sknp, Skp, Sne, Sub,
    Subn, Sys, Xor,
}

#[derive(Debug)]
pub enum Register {
    V0, V1, V2, V3,
    V4, V5, V6, V7,
    V8, V9, Va, Vb,
    Vc, Vd, Ve, Vf,
    Sp, Pc, I, St, Dt, IVal,
}

#[derive(Debug)]
pub enum Token {
    Opcode(Mnemonic, u32),
    Reg(Register, u32),
    ImmConst(u16, u32),
}

pub struct Stream {
    input: Bytes<File>,
    line: u32,
}

impl Stream {
    pub fn new(input: Bytes<File>) -> Stream {
        Stream {
            input: input,
            line: 1,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut buffer: Vec<u8> = vec![];

        loop {
            let bs = self.input.next().map_or(None, |bm| match bm {
                Ok(b) => Some(b),
                _ => None
            });

            match bs {
                Some(b) => {
                    if Stream::is_separator(b) {
                        if buffer.len() > 0 {
                            return Stream::create_token(buffer, self.line);
                        }
                        if (b as char) == '\n' {
                            self.line += 1;
                        }
                    } else {
                        buffer.push(b);
                    }
                },
                None => {
                    return Stream::create_token(buffer, self.line);
                }
            }
        }
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    fn create_token(input: Vec<u8>, line: u32) -> Option<Token> {
        let token_str: String = String::from_utf8(input).unwrap_or_else(|err| {
            println!("Error: {:?}", err);
            "".to_owned()
        }).to_lowercase();

        let (numeric, radix) = Stream::is_numeric(&token_str);
        if numeric {
            return match radix {
                // base 10
                0 => Some(Token::ImmConst(token_str.parse::<u16>().unwrap(), line)),
                // base 16
                1 => Some(Token::ImmConst(u16::from_str_radix(&token_str[2..], 16).unwrap(), line)),
                _ => None,
            }

        }

        if Stream::is_register(&token_str) {
            return match token_str.as_str() {
                "v0"   => Some(Token::Reg(Register::V0, line)),
                "v1"   => Some(Token::Reg(Register::V1, line)),
                "v2"   => Some(Token::Reg(Register::V2, line)),
                "v3"   => Some(Token::Reg(Register::V3, line)),
                "v4"   => Some(Token::Reg(Register::V4, line)),
                "v5"   => Some(Token::Reg(Register::V5, line)),
                "v6"   => Some(Token::Reg(Register::V6, line)),
                "v7"   => Some(Token::Reg(Register::V7, line)),
                "v8"   => Some(Token::Reg(Register::V8, line)),
                "v9"   => Some(Token::Reg(Register::V9, line)),
                "va"   => Some(Token::Reg(Register::Va, line)),
                "vb"   => Some(Token::Reg(Register::Vb, line)),
                "vc"   => Some(Token::Reg(Register::Vc, line)),
                "vd"   => Some(Token::Reg(Register::Vd, line)),
                "ve"   => Some(Token::Reg(Register::Ve, line)),
                "vf"   => Some(Token::Reg(Register::Vf, line)),
                "sp"   => Some(Token::Reg(Register::Sp, line)),
                "st"   => Some(Token::Reg(Register::St, line)),
                "dt"   => Some(Token::Reg(Register::Dt, line)),
                "pc"   => Some(Token::Reg(Register::Pc, line)),
                "i"    => Some(Token::Reg(Register::I, line)),
                "[i]"  => Some(Token::Reg(Register::IVal, line)),
                _      => None
            }
        }

        match token_str.as_str() {
            "add"   => Some(Token::Opcode(Mnemonic::Add, line)),
            "call"  => Some(Token::Opcode(Mnemonic::Call, line)),
            "cls"   => Some(Token::Opcode(Mnemonic::Cls, line)),
            "drw"   => Some(Token::Opcode(Mnemonic::Drw, line)),
            "jp"    => Some(Token::Opcode(Mnemonic::Jp, line)),
            "ld"    => Some(Token::Opcode(Mnemonic::Ld, line)),
            "or"    => Some(Token::Opcode(Mnemonic::Or, line)),
            "ret"   => Some(Token::Opcode(Mnemonic::Ret, line)),
            "rnd"   => Some(Token::Opcode(Mnemonic::Rnd, line)),
            "se"    => Some(Token::Opcode(Mnemonic::Se, line)),
            "shl"   => Some(Token::Opcode(Mnemonic::Shl, line)),
            "shr"   => Some(Token::Opcode(Mnemonic::Shr, line)),
            "sknp"  => Some(Token::Opcode(Mnemonic::Sknp, line)),
            "skp"   => Some(Token::Opcode(Mnemonic::Skp, line)),
            "sne"   => Some(Token::Opcode(Mnemonic::Sne, line)),
            "sub"   => Some(Token::Opcode(Mnemonic::Sub, line)),
            "subn"  => Some(Token::Opcode(Mnemonic::Subn, line)),
            "sys"   => Some(Token::Opcode(Mnemonic::Sys, line)),
            "xor"   => Some(Token::Opcode(Mnemonic::Xor, line)),
            _       => None
        }
    }

    fn is_separator(b: u8) -> bool {
        match b as char {
            ','|' '|'\n' | '\t' => { true }
            _ => { false }
        }
    }

    fn is_register(input: &str) -> bool {
        input == "v0" || input == "v1" || input == "v2" ||
        input == "v3" || input == "v4" || input == "v5" ||
        input == "v6" || input == "v7" || input == "v8" ||
        input == "v9" || input == "va" || input == "vb" ||
        input == "vc" || input == "vd" || input == "ve" ||
        input == "vf" || input == "sp" || input == "st" ||
        input == "dt" || input == "pc" || input == "i" ||
        input == "[i]"
    }

    // Returns a tuple of boolean and integer
    // The input is numeric if the boolean component of the result is true
    // Integer component of result:
    //   0 => base 10
    //   1 => base 16
    fn is_numeric(input: &str) -> (bool, u8) {
        if input.len() < 1 {
            return (false, ::std::u8::MAX);
        }

        let in_bytes = input.as_bytes();
        if !Stream::is_ascii_numeric(in_bytes[0]) {
            return (false, ::std::u8::MAX);
        }

        // hex
        if input.starts_with("0x") {
            for i in 2..(in_bytes.len()) {
                if !Stream::is_ascii_hex(in_bytes[i]) {
                    return (false, ::std::u8::MAX);
                }
                return (true, 1);
            }
        } else {
            for b in in_bytes {
                if !Stream::is_ascii_numeric(*b) {
                    return (false, ::std::u8::MAX);
                }
                return (true, 0);
            }
        }
        return (false, ::std::u8::MAX);
    }

    fn is_ascii_numeric(input: u8) -> bool {
        input >= 48 && input <= 57
    }

    fn is_ascii_hex(input: u8) -> bool {
        (input >= 48 && input <= 57) || ((input >= 97 && input <= 102) || (input >= 65 && input <= 70))
    }
}
