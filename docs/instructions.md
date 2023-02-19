# Architecture
For now, the virtual CPU consists of 7 registers.
All the registers can store 32-bit values.

| Register | Description                    | Address | Capacity |
|----------|--------------------------------|---------|----------|
| `IP`     | Instruction pointer            | 0x00    | 32 bits  |
| `RO`     | General-purpose register       | 0x04    | 32 bits  |
| `R1`     | General-purpose register       | 0x08    | 32 bits  |
| `R2`     | General-purpose register       | 0x0C    | 32 bits  |
| `R3`     | General-purpose register       | 0x10    | 32 bits  |
| `CMP`    | Register for comparison result | 0x14    | 32 bits  |
| `END`    | Execution halt flag            | 0x18    | 32 bits  |

All of them are stored in virtual memory at the
addresses mentioned above.

It also provides a basic set of the following instructions:

| Instruction | Code | Corresponding class     |
|-------------|------|-------------------------|
| `ADD`       | 0x1  | AddInstruction          |
| `SUB`       | 0x2  | SubInstruction          |
| `MUL`       | 0x3  | MulInstruction          |
| DIV         | 0x4  | DivInstruction          |
| JMP         | 0x5  | JumpInstruction         |
| LD          | 0x6  | LoadInstruction         |
| FIN         | 0x7  | FinishInstruction       |
| OUT         | 0x8  | OutInstruction          |
| EQ          | 0x9  | EqualInstruction        |
| L           | 0xA  | LessInstruction         |
| LE          | 0xB  | LessEqualInstruction    |
| LDA         | 0xC  | LoadAbsoluteInstruction |

Each instruction is 32-bit and has unique rules of decoding. To get all information about
a specific instruction please refer to code documentation of the corresponding class
for the instruction in `src/vm/instruction.rs`.