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
| `SP`     | Stack pointer                  | 0x1C    | 32 bits  |

All of them are stored in virtual memory at the
addresses mentioned above.

It also provides a basic set of the following instructions:

| Instruction | Code | Corresponding class        |
|-------------|------|----------------------------|
| `ADD`       | 0x01 | AddInstruction             |
| `SUB`       | 0x02 | SubInstruction             |
| `MUL`       | 0x03 | MulInstruction             |
| `DIV`       | 0x04 | DivInstruction             |
| `JMP`       | 0x05 | JumpInstruction            |
| `LD`        | 0x06 | LoadInstruction            |
| `FIN`       | 0x07 | FinishInstruction          |
| `OUT`       | 0x08 | OutInstruction             |
| `EQ`        | 0x09 | EqualInstruction           |
| `L`         | 0x0A | LessInstruction            |
| `LE`        | 0x0B | LessEqualInstruction       |
| `INP`       | 0x0D | InputInstruction           |
| `JCMP`      | 0x0E | JumpCompareInstruction     |
| `JNCMP`     | 0x0F | JumpNotCompareInstruction  |
| `OUTR`      | 0x10 | OutFromRegisterInstruction |
| `SKIP`      | 0x11 | SkipInstruction            |
| `OUTN`      | 0x12 | OutNumberInstruction       |
| `MOV`       | 0x13 | MoveInstruction            |
| `INPN`      | 0x14 | InputNumberInstruction     |
| `PUSH`      | 0x15 | PushToStackInstruction     |
| `POP`       | 0x16 | PopFromStackInstruction    |

Each instruction is 32-bit and has unique rules of decoding. To get all information about
a specific instruction please refer to code documentation of the corresponding class
for the instruction in `src/vm/arch/instruction.rs`.
