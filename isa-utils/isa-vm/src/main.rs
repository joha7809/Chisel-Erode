// Virtual Machine for the custom ISA
// Use the shared isa-core library for instruction decoding

use isa_core::{Opcode, Operand, REGISTER_LIMIT, ResolvedInstruction};

fn main() {
    println!("ISA Virtual Machine");
    println!("TODO: Implement VM");

    // Example: You can decode instructions like this:
    // let word: u32 = 0x08400064; // Some encoded instruction
    // if let Some(instr) = DecodedInstruction::decode(word) {
    //     println!("Decoded: {:?}", instr);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_instruction() {
        // Example: Test decoding an ADD instruction
        // ADD R1, R2, R3 encoded as a word
        let add_instr = ResolvedInstruction {
            opcode: Opcode::ADD,
            operands: vec![
                Operand::Register(1),
                Operand::Register(2),
                Operand::Register(3),
            ],
        };

        let encoded = add_instr.encode().unwrap();
        let decoded = ResolvedInstruction::decode(encoded).unwrap();

        assert_eq!(decoded, add_instr);
    }
}
