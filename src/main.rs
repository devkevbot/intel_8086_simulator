use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let in_file_name = &args[1];

    let operations: Vec<_> = std::fs::read(in_file_name)
        .unwrap()
        .chunks(2)
        .map(|inst_bytes| decode_instruction([inst_bytes[0], inst_bytes[1]]))
        .collect();

    let out_file_name = format!("{}.out", in_file_name);
    let mut f = File::create(out_file_name).expect("Should be able to create file");
    for operation in &operations {
        writeln!(f, "{}", operation).unwrap();
    }
}

fn decode_instruction(instr: [u8; 2]) -> String {
    let upper = instr[0];

    let op_mask = 0b11111100;
    let op_value = (upper & op_mask) >> 2;

    let op_name = get_op_name(op_value);

    let d_mask = 0b00000010;
    let d_bit = (upper & d_mask) >> 1;

    let w_mask = 0b00000001;
    let w_bit = upper & w_mask;

    let lower = instr[1];
    // let mod_mask = 0b11000000;
    // let mod_value = (lower & mod_mask) >> 6;

    let reg_mask = 0b00111000;
    let reg_value = (lower & reg_mask) >> 3;
    let reg_reg_value = get_register_name(reg_value, w_bit);

    let rm_mask = 0b00000111;
    let rm_value = lower & rm_mask;
    let rm_reg_value = get_register_name(rm_value, w_bit);

    let mut dest = rm_reg_value;
    let mut src = reg_reg_value;
    if d_bit == 1 {
        dest = reg_reg_value;
        src = rm_reg_value;
    }

    let output = format!("{} {}, {}", op_name, dest, src);
    output
}

fn get_op_name(op_value: u8) -> &'static str {
    match op_value {
        0b100010 => "mov",
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
    fn mov_works() {
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

        let operations: String = std::fs::read("listing_0038_many_register_mov")
            .unwrap()
            .chunks(2)
            .map(|inst_bytes| decode_instruction([inst_bytes[0], inst_bytes[1]]))
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(operations, expected);
    }
}
