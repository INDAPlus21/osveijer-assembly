# Formula 1 Language Documentation

## Repositories
There are four repositories availible in this language: `P0`, `P1`, `P2` and `P3`.


## Instructions
| Instruction | Synatax | Explaniation |
| ----------- | ------- | ------------ |
| `oversteer` | `oversteer Reg1 Reg2 Imm` | Set value of `Reg1` to value of `Reg1` + value of `Reg2` + immediate `Imm` (1-bit unsigned) |
| `understeer` | `understeer Reg1 Reg2 Imm` | Set value of `Reg1` to value of `Reg1` - value of `Reg2` - immediate `Imm` (1-bit unsigned) |
| `overtake` | `overtake Reg1 Reg2 Imm` | Set value of `Reg1` to value of `Reg2` - immediate `Imm` (1-bit unsigned) |
| `pitstop` | `pitstop Reg1 Reg2 Imm` | Set value of `Reg1` to immediate `Imm` (1-bit unsigned) |
| `sidebysideintot1` | `sidebysideintot1 Reg1 Reg2 Imm` | Skip next instruction if value of `Reg1` is equal to immediate `Imm` (1-bit unsigned) |
| `divebomb` | `divebomb Imm` | Jump `Imm` (5-bit signed) rows |
| `steering` | `steering` | Takes user input and puts it into registry `P1` |
| `radiocall` | `radiocall` | Prints contents of registry `P1` to default print stream |