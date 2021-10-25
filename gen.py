ops = {
    0x0C: 'ADD',
    0x14: 'AND',
    0x19: 'ASHR',
    0x00: 'BREAK',
    0x03: 'CALL',
    0x05: 'CMPeq',
    0x06: 'CMPlte',
    0x07: 'CMPgte',
    0x08: 'CMPulte',
    0x09: 'CMPugte',
    0x2D: 'CMPIeq',
    0x2E: 'CMPIlte',
    0x2F: 'CMPIgte',
    0x30: 'CMPIulte',
    0x31: 'CMPIugte',
    0x10: 'DIV',
    0x11: 'DIVU',
    0x1A: 'EXTNDB',
    0x1C: 'EXTNDD',
    0x1B: 'EXTNDW',
    0x01: 'JMP',
    0x02: 'JMP8',
    0x29: 'LOADSP',
    0x12: 'MOD',
    0x13: 'MODU',
    0x1D: 'MOVbw',
    0x1E: 'MOVww',
    0x1F: 'MOVdw',
    0x20: 'MOVqw',
    0x21: 'MOVbd',
    0x22: 'MOVwd',
    0x23: 'MOVdd',
    0x24: 'MOVqd',
    0x28: 'MOVqq',
    0x37: 'MOVI',
    0x38: 'MOVIn',
    0x32: 'MOVnw',
    0x33: 'MOVnd',
    0x39: 'MOVREL',
    0x25: 'MOVsnw',
    0x26: 'MOVsnd',
    0x0E: 'MUL',
    0x0F: 'MULU',
    0x0B: 'NEG',
    0x0A: 'NOT',
    0x15: 'OR',
    0x2C: 'POP',
    0x36: 'POPn',
    0x2B: 'PUSH',
    0x35: 'PUSHn',
    0x04: 'RET',
    0x17: 'SHL',
    0x18: 'SHR',
    0x2A: 'STORESP',
    0x0D: 'SUB',
    0x16: 'XOR'
}

assembly1 = '''
assert_eq!(
    "{}",
    dis(
        opts,
        cur,
        &[
            &[byte({}, {}, OpCode::{}), 0b{}][..],
'''
assembly2 = '            &({}{}{}).to_le_bytes()[..],\n'
assembly3 = '''        ].concat()
    )
);
'''

widths = {2: 16, 4: 32, 8: 64, 16: 2, 32: 4, 64: 8}
indices = dict(
    NATIND16=36879,
    NATIND32=2954019116,
    NATIND64=11529215048034579760
)

with open('in.py') as tests:
    with open('out.rs', 'w') as out:
        all_tests = tests.read().split('\n\n')
        for test in all_tests:
            lines = test.split('\n')

            assert len(lines) > 1

            try:
                # import ipdb; ipdb.set_trace()
                _, opcode, bit8, bit7, byte1_and_asm = lines[0].split(',', maxsplit=4)

                bit8 = 1 if bit8.strip() == 'True' else 0
                bit7 = 1 if bit7.strip() == 'True' else 0
                byte1, asm = byte1_and_asm.split('$')
                byte1 = byte1.split(')')[0].strip()[2:]
                asm = asm.strip()
                opcode = int(opcode.strip(), 16)

                print(opcode, bit8, bit7, byte1, asm)

                # width, asm = asmbl.split('$')
                # width = width[0]
                # asm = asm.strip()

                op = ops[opcode]

                print(
                    assembly1.format(asm, bit8, bit7, op, byte1),
                    file=out,
                    end=''
                )

                for line in lines[1:]:
                    _, ind_or_imm, width, sign = line.split(',', maxsplit=3)

                    ind_or_imm = ind_or_imm.strip()
                    width = int(width.strip())
                    sign = 'i' if 'signed=True' in sign else 'u'
                    value = indices.get(ind_or_imm, ind_or_imm)

                    print(
                        assembly2.format(value, sign, widths[width]),
                        file=out,
                        end=''
                    )

                print(assembly3, file=out, end='')
            except Exception as e:
                import traceback
                traceback.print_exc()
                print('-' * 80)
                print(e)
                # print(lines)
                break
