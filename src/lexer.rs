use std::fs::File;
use std::io::Bytes;


#[derive(Debug)]
pub enum Mnemonic {
    Add, And, Call, Cls,
    Drw, Jp, Ld, Or,
    Ret, Rnd, Se, Shl,
    Shr, Sknp, Skp, Sne,
    Sub, Subn, Sys, Xor,
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    V0, V1, V2, V3,
    V4, V5, V6, V7,
    V8, V9, Va, Vb,
    Vc, Vd, Ve, Vf,
}

impl Register {
    fn number(&self) -> u8 {
        match *self {
            Register::V0 => {0}, Register::V1 => {1},
            Register::V2 => {2}, Register::V3 => {3},
            Register::V4 => {4}, Register::V5 => {5},
            Register::V6 => {6}, Register::V7 => {7},
            Register::V8 => {8}, Register::V9 => {9},
            Register::Va => {0xa}, Register::Vb => {0xb},
            Register::Vc => {0xc}, Register::Vd => {0xd},
            Register::Ve => {0xe}, Register::Vf => {0xf},
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Opcode(Mnemonic, u32),
    Reg(Register, u32),
    ImmConst(u16, u32),
    F(u32), B(u32), K(u32),
    I(u32), St(u32), Dt(u32),
    IVal(u32),
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
                            let tt = Stream::create_token(buffer, self.line);
                            if (b as char) == '\n' {
                                self.line += 1;
                            }
                            return tt;
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
                "st"   => Some(Token::St(line)),
                "dt"   => Some(Token::Dt(line)),
                "i"    => Some(Token::I(line)),
                "[i]"  => Some(Token::IVal(line)),
                _      => None
            }
        }

        match token_str.as_str() {
            "add"   => Some(Token::Opcode(Mnemonic::Add, line)),
            "and"   => Some(Token::Opcode(Mnemonic::And, line)),
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
            "f"     => Some(Token::F(line)),
            "b"     => Some(Token::B(line)),
            "k"     => Some(Token::K(line)),
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
        input == "vf" || input == "st" || input == "dt" ||
        input == "i" || input == "[i]"
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

pub fn code_gen(tokens: &Vec<Token>) -> Result<Vec<u16>, u32> {
    let mut result: Vec<u16> = vec![];

    let mut curr_opcode: Option<&Mnemonic> = None;
    let mut last_token: Option<TokenRef> = None;
    let mut draw_first_reg: Option<(&Register, u32)> = None;

    #[derive(Copy, Clone)]
    enum TokenRef<'a> {
        Reg(&'a Register, u32),
        ImmConst(&'a u16, u32),
        F(u32), B(u32), K(u32),
        I(u32), St(u32), Dt(u32),
        IVal(u32),
    }

    for token in tokens {
        let mut temp_last_token: Option<TokenRef> = last_token.clone();
        match token {
            &Token::Reg(ref nr, nl) => {
                match curr_opcode {
                    Some(&Mnemonic::Ld) => {
                        match last_token {
                            Some(TokenRef::Reg(ref or, _)) => {
                                result.push((0x8u16 << 12) | ((or.number() as u16) << 8) | ((nr.number() as u16) << 4) | 0x0);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            Some(TokenRef::B(_)) => {
                                result.push((0xfu16 << 12) | ((nr.number() as u16) << 8) | 0x21);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            Some(TokenRef::F(_)) => {
                                result.push((0xfu16 << 12) | ((nr.number() as u16) << 8) | 0x1d);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            Some(TokenRef::Dt(_)) => {
                                result.push((0xfu16 << 12) | ((nr.number() as u16) << 8) | 0x0f);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            Some(TokenRef::St(_)) => {
                                result.push((0xfu16 << 12) | ((nr.number() as u16) << 8) | 0x12);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            Some(TokenRef::IVal(_)) => {
                                result.push((0xfu16 << 12) | ((nr.number() as u16) << 8) | 0x37);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)) },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Se) => {
                        match last_token {
                            Some(TokenRef::Reg(ref or, _)) => {
                                result.push((0x5u16 << 12) | ((or.number() as u16) << 8) | ((nr.number() as u16) << 4) | 0x0);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)) },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Sne) => {
                        match last_token {
                            Some(TokenRef::Reg(ref or, _)) => {
                                result.push((0x9u16 << 12) | ((or.number() as u16) << 8) | ((nr.number() as u16) << 4) | 0x0);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)) },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Skp) => {
                        match last_token {
                            None => {
                                result.push((0xeu16 << 12) | ((nr.number() as u16) << 8) | 0x9e);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Sknp) => {
                        match last_token {
                            None => {
                                result.push((0xeu16 << 12) | ((nr.number() as u16) << 8) | 0xa1);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Add) => {
                        match last_token {
                            Some(TokenRef::Reg(ref or, _)) => {
                                result.push((0x8u16 << 12) | ((or.number() as u16) << 8) | ((nr.number() as u16) << 4) | 0x4);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)) },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Sub) => {
                        match last_token {
                            Some(TokenRef::Reg(ref or, _)) => {
                                result.push((0x8u16 << 12) | ((or.number() as u16) << 8) | ((nr.number() as u16) << 4) | 0x5);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)) },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Subn) => {
                        match last_token {
                            Some(TokenRef::Reg(ref or, _)) => {
                                result.push((0x8u16 << 12) | ((or.number() as u16) << 8) | ((nr.number() as u16) << 4) | 0x7);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)) },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Rnd) => {
                        match last_token {
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)) },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Or) => {
                        match last_token {
                            Some(TokenRef::Reg(ref or, _)) => {
                                result.push((0x8u16 << 12) | ((or.number() as u16) << 8) | ((nr.number() as u16) << 4) | 0x1);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)) },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::And) => {
                        match last_token {
                            Some(TokenRef::Reg(ref or, _)) => {
                                result.push((0x8u16 << 12) | ((or.number() as u16) << 8) | ((nr.number() as u16) << 4) | 0x2);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)) },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Xor) => {
                        match last_token {
                            Some(TokenRef::Reg(ref or, _)) => {
                                result.push((0x8u16 << 12) | ((or.number() as u16) << 8) | ((nr.number() as u16) << 4) | 0x3);
                                temp_last_token = None;
                                curr_opcode = None;
                            },
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)); },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Shr) => {
                    },
                    Some(&Mnemonic::Shl) => {
                    },
                    Some(&Mnemonic::Cls) => {
                        match last_token {
                            None => {
                                result.push(0u16 | 0xe0);
                                temp_last_token = None;
                            },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Drw) => {
                        match last_token {
                            Some(TokenRef::Reg(ref or, ol)) => {
                                draw_first_reg = Some((or, ol));
                                temp_last_token = Some(TokenRef::Reg(nr, nl));
                                curr_opcode = None;
                            },
                            None => { temp_last_token = Some(TokenRef::Reg(nr, nl)); },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Ret) => {
                        match last_token {
                            None => {
                                result.push(0u16 | 0xee);
                                temp_last_token = None;
                            },
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Jp) => {
                        match last_token {
                            None => {
                                if nr.number() != 0 {
                                    return Err(nl);
                                }
                                temp_last_token = Some(TokenRef::Reg(nr, nl));
                            }
                            _ => {
                                return Err(nl);
                            }
                        }
                    },
                    Some(&Mnemonic::Call) => {
                        return Err(nl);
                    },
                    Some(&Mnemonic::Sys) => {
                        return Err(nl);
                    },
                    None => {
                        return Err(nl);
                    }
                }
                last_token = temp_last_token;
            },
            &Token::Opcode(ref no, nl) => {
                match curr_opcode {
                    None => {
                        curr_opcode = Some(no);
                    },
                    _ => {
                        return Err(nl)
                    }
                }
            },
            &Token::ImmConst(ref nc, nl) => {
                last_token = Some(TokenRef::ImmConst(nc, nl));
            },
            &Token::K(l) => {
                last_token = Some(TokenRef::K(l));
            },
            &Token::F(l) => {
                last_token = Some(TokenRef::F(l));
            },
            &Token::B(l) => {
                last_token = Some(TokenRef::B(l));
            },
            &Token::I(l) => {
                last_token = Some(TokenRef::I(l));
            },
            &Token::IVal(l) => {
                last_token = Some(TokenRef::IVal(l));
            },
            &Token::St(l) => {
                last_token = Some(TokenRef::St(l));
            },
            &Token::Dt(l) => {
                last_token = Some(TokenRef::Dt(l));
            },
        }
    }
    Ok(result)
}
