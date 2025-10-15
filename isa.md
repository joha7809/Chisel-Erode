# ISA

## Arithmetic and Logic Instructions

| Instruction    | Syntax (Example)  | Meaning (Example) |
| -------------- | ----------------- | ----------------- |
| Addition       | `ADD R1, R2, R3`  | `R1 = R2 + R3`    |
| Subtraction    | `SUB R1, R2, R3`  | `R1 = R2 - R3`    |
| Multiplication | `MULT R1, R2, R3` | `R1 = R2 * R3`    |
| Immediate Add. | `ADDI R1, R2, 4`  | `R1 = R2 + 4`     |
| Immediate Sub. | `SUBI R1, R2, 5`  | `R1 = R2 - 5`     |
| Bitwise OR     | `OR R1, R2, R3`   | `R1 = R2 or R3`   |
| Bitwise AND    | `AND R1, R2, R3`  | `R1 = R2 and R3`  |
| Bitwise NOT    | `NOT R1, R2`      | `R1 = not(R2)`    |

## Data Transfer Instructions

| Instruction    | Syntax (Example) | Meaning (Example) |
| -------------- | ---------------- | ----------------- |
| Load immediate | `LI R1, 6`       | `R1 = 6`          |
| Load data      | `LD R1, R2`      | `R1 = memory(R2)` |
| Store data     | `SD R1, R2`      | `memory(R2) = R1` |

## Control and Flow Instructions

| Instruction          | Syntax (Example)  | Meaning (Example)             |
| -------------------- | ----------------- | ----------------------------- |
| Jump                 | `JR 7`            | `goto inst. 7`                |
| Jump if equal        | `JEQ 8, R2, R3`   | `if (R2 == R3) goto inst. 8`  |
| Jump if less than    | `JLT 9, R2, R3`   | `if (R2 < R3) goto inst. 9`   |
| Jump if greater than | `JGT 10, R2, R3`  | `if (R2 > R3) goto inst. 10`  |
| Jump if eq. to value | `JETV 10, R2, 10` | `if (R2 == 10) goto inst. 10` |
| No operation         | `NOP`             | do nothing                    |
| End execution        | `END`             | terminates execution          |

## Labels

Labels are defined by placing a name followed by a colon at the beginning of a line. They serve as targets for jump instructions.
When parsed into machine code, labels are replaced with the corresponding instruction address, which is determined by the position of the label in the source code.
Example:

```
JEQ loop_start, R1, R2
start:          # This is a label
    ADD R1, R2, R3
    JEQ end, R1, R4
    SUB R1, R1, R5

end:            # Another label
    END
```
