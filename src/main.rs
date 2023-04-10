use std::io::Write;
use std::{env, fs::File};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Expectd input file name!")
    }
    let in_file_name = &args[1];
    let bytes = std::fs::read(in_file_name).unwrap();

    let decoded_instructions = decode(bytes);

    let out_file_name = format!("{}.out", in_file_name);
    let mut out = File::create(out_file_name).expect("Should be able to create file");
    for operation in &decoded_instructions {
        writeln!(out, "{}", operation).unwrap();
    }
}

fn decode(input_bytes: Vec<u8>) -> Vec<String> {
    let mut iter = input_bytes.into_iter();
    let mut out: Vec<String> = vec![];

    while let Some(byte) = iter.next() {
        match byte {
            // Register/memory to/from register
            b if b >> 2 == 0b100010 => {
                let d_val = byte >> 1 & 1;
                let w_val = byte & 1;

                // Read second byte
                let next = iter.next().unwrap();
                let mod_val = next >> 6;
                let reg_val = next >> 3 & 0b111;
                let rm_val = next & 0b111;

                // Register-to-register move
                if mod_val == 0b11 {
                    if d_val == 0b1 {
                        out.push(format!(
                            "mov {}, {}",
                            get_register_name(reg_val, w_val),
                            get_register_name(rm_val, w_val)
                        ))
                    } else {
                        out.push(format!(
                            "mov {}, {}",
                            get_register_name(rm_val, w_val),
                            get_register_name(reg_val, w_val)
                        ));
                    }
                }
                // Effective address calculation
                else {
                    // Direct address
                    let address = if mod_val == 0b00 && rm_val == 0b110 {
                        let lo = iter.next().unwrap();
                        let hi = iter.next().unwrap();
                        let offset: u16 = (hi as u16) << 8 | (lo as u16);
                        format!("{}", offset)
                    } else {
                        let rm_eq = get_rm_address_equation(rm_val);
                        if mod_val == 0b10 {
                            let lo = iter.next().unwrap();
                            let hi = iter.next().unwrap();
                            let offset: u16 = (hi as u16) << 8 | (lo as u16);
                            format!("[{} + {}]", rm_eq, offset)
                        } else if mod_val == 0b01 {
                            let lo = iter.next().unwrap();
                            format!("[{} + {}]", rm_eq, lo)
                        } else {
                            format!("[{}]", rm_eq)
                        }
                    };

                    if d_val == 0b1 {
                        out.push(format!(
                            "mov {}, {}",
                            get_register_name(reg_val, w_val),
                            address
                        ));
                    } else {
                        out.push(format!(
                            "mov {}, {}",
                            address,
                            get_register_name(reg_val, w_val)
                        ));
                    }
                }
            }
            // Immediate to register
            b if b >> 4 == 0b1011 => {
                let w_val = byte >> 3 & 1;
                let reg_val = byte & 0b111;

                let immediate = if w_val == 0b1 {
                    let lo = iter.next().unwrap();
                    let hi = iter.next().unwrap();
                    let immediate = (hi as u16) << 8 | (lo as u16);
                    format!("{}", immediate)
                } else {
                    let immediate = iter.next().unwrap();
                    format!("{}", immediate)
                };

                out.push(format!(
                    "mov {}, {}",
                    get_register_name(reg_val, w_val),
                    immediate
                ));
            }
            _ => {}
        }
    }

    out
}

fn get_rm_address_equation(rm_val: u8) -> &'static str {
    match rm_val {
        0b000 => "bx + si",
        0b001 => "bx + di",
        0b010 => "bp + si",
        0b011 => "bp + di",
        0b100 => "si",
        0b101 => "di",
        0b110 => "bp",
        0b111 => "bx",
        _ => "",
    }
}

fn get_register_name(reg_or_rm_value: u8, w_value: u8) -> &'static str {
    match (reg_or_rm_value, w_value) {
        (0b000, 0) => "al",
        (0b001, 0) => "cl",
        (0b010, 0) => "dl",
        (0b011, 0) => "bl",
        (0b100, 0) => "ah",
        (0b101, 0) => "ch",
        (0b110, 0) => "dh",
        (0b111, 0) => "bh",

        (0b000, 1) => "ax",
        (0b001, 1) => "cx",
        (0b010, 1) => "dx",
        (0b011, 1) => "bx",
        (0b100, 1) => "sp",
        (0b101, 1) => "bp",
        (0b110, 1) => "si",
        (0b111, 1) => "di",

        _ => "",
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mov_simple_works() {
        let expected = "mov cx, bx
mov ch, ah
mov dx, bx
mov si, bx
mov bx, di
mov al, cl
mov ch, ch
mov bx, ax
mov bx, si
mov sp, di
mov bp, ax";

        let bytes = std::fs::read("listing_0038_many_register_mov").unwrap();

        let decoded_instructions = decode(bytes);

        let operations = decoded_instructions.join("\n");
        assert_eq!(operations, expected);
    }

    #[test]
    fn move_complex_works() {
        let expected = "mov si, bx
mov dh, al
mov cl, 12
mov ch, 244
mov cx, 12
mov cx, 65524
mov dx, 3948
mov dx, 61588
mov al, [bx + si]
mov bx, [bp + di]
mov dx, [bp + 0]
mov ah, [bx + si + 4]
mov al, [bx + si + 4999]
mov [bx + di], cx
mov [bp + si], cl
mov [bp + 0], ch";

        let bytes = std::fs::read("listing_0039_more_movs").unwrap();

        let decoded_instructions = decode(bytes);

        let operations = decoded_instructions.join("\n");
        assert_eq!(operations, expected);
    }
}
