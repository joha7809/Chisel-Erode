# ISA Encoder - Custom Assembly Language Assembler

A command-line assembler for a custom instruction set architecture (ISA) with comprehensive error reporting.

## Features

- ✅ Full ISA support (arithmetic, logical, memory, and control flow instructions)
- ✅ Label support for jumps and branches
- ✅ Pretty error messages with source code context
- ✅ Multiple output formats (binary, hex)
- ✅ Syntax checking without assembly

## Installation

```bash
cargo build --release
```

## Usage

### Assemble a file

```bash
# Generate hex output (default)
cargo run -- assemble -i example.asm -o output

# Generate binary output
cargo run -- assemble -i example.asm -o output.bin -f binary

# Generate both formats
cargo run -- assemble -i example.asm -o output -f both
```

### Check syntax without assembling

```bash
cargo run -- check example.asm
```

## Supported Instructions

### Arithmetic

- `ADD Rd, Rs1, Rs2` - Add two registers
- `SUB Rd, Rs1, Rs2` - Subtract registers
- `MULT Rd, Rs1, Rs2` - Multiply registers
- `ADDI Rd, Rs, imm` - Add immediate
- `SUBI Rd, Rs, imm` - Subtract immediate

### Logical

- `AND Rd, Rs1, Rs2` - Bitwise AND
- `OR Rd, Rs1, Rs2` - Bitwise OR
- `NOT Rd, Rs` - Bitwise NOT

### Memory

- `LI Rd, imm` - Load immediate
- `LD Rd, Rs` - Load from memory
- `SD Rs, Rd` - Store to memory

### Control Flow

- `JR imm` - Jump to address
- `JEQ imm, Rs1, Rs2` - Jump if equal
- `JLT imm, Rs1, Rs2` - Jump if less than
- `JGT imm, Rs1, Rs2` - Jump if greater than
- `JETV imm, Rs, val` - Jump if equal to value
- `NOP` - No operation
- `END` - End program

### Labels

Labels can be defined with a colon and referenced in jump instructions:

```assembly
loop_start:
    ADDI R1, R1, 1
    JLT R1, 100, loop_start
```

## Error Reporting

The assembler provides detailed error messages with source context:

```
Parse Error: Operand count mismatch, expected 3, found 2
  |
3 | ADD R3, R1
  | ^^^ wrong number of operands
```

## Example

See `example.asm` for a comprehensive demonstration of all features.

```bash
cargo run -- assemble -i example.asm -o example
```

This will create `example` (or `example.hex` depending on format) with the encoded machine code.
