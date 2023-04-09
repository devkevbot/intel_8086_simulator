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

    let op_mask = 63u8 << 2;
    let op_value = (upper & op_mask) >> 2;

    let op_name = get_op_name(op_value);

    let d_mask = 1u8 << 1;
    let d_bit = (upper & d_mask) >> 1;

    let w_mask = 1u8;
    let w_bit = upper & w_mask;

    let lower = instr[1];

    // let mod_mask = 3u8 << 6;
    // let mod_value = (lower & mod_mask) >> 6;

    let reg_mask = 7u8 << 3;
    let reg_value = (lower & reg_mask) >> 3;
    let reg_reg_value = get_register_name(reg_value, w_bit);

    let rm_mask = 7u8;
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
        34 => "mov",
        _ => "",
    }
}

fn get_register_name(reg_or_rm_value: u8, w_value: u8) -> &'static str {
    match (reg_or_rm_value, w_value) {
        (0, 0) => "al",
        (1, 0) => "cl",
        (2, 0) => "dl",
        (3, 0) => "bl",
        (4, 0) => "ah",
        (5, 0) => "ch",
        (6, 0) => "dh",
        (7, 0) => "bh",

        (0, 1) => "ax",
        (1, 1) => "cx",
        (2, 1) => "dx",
        (3, 1) => "bx",
        (4, 1) => "sp",
        (5, 1) => "bp",
        (6, 1) => "si",
        (7, 1) => "di",

        _ => "",
    }
}
