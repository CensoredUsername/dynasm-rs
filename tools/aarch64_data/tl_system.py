
tlentry(['AUTIA1716', 'AUTIASP', 'AUTIAZ', 'AUTIB1716', 'AUTIBSP', 'AUTIBZ', 'CFINV', 'CSDB', 'DRPS', 'ERET', 'ESB', 'NOP', 'PACIA1716', 'PACIASP', 'PACIAZ', 'PACIB1716', 'PACIBSP', 'PACIBZ', 'PSSBB', 'SB', 'SEV', 'SEVL', 'SSBB', 'WFE', 'WFI', 'XPACLRI', 'YIELD'],
    '', (),
    matcher   = '',
    processor = '',
)

tlentry(['HINT'],
    '#<imm>', (('CRm', 4, 8), ('op2', 3, 5)),
    matcher   = 'Imm',
    processor = 'Ubits(5, 7)',
)

tlentry(['BRK', 'HLT', 'HVC', 'SMC', 'SVC'],
    '#<imm>', (('imm16', 16, 5),),
    matcher   = 'Imm',
    processor = 'Ubits(5, 16)',
)

tlentry(['SYS'],
    '#<op1>,<Cn>,<Cm>,#<op2>{,<Xt>}', (('op1', 3, 16), ('CRn', 4, 12), ('CRm', 4, 8), ('op2', 3, 5), ('Rt', 5, 0)),
    matcher   = 'Imm, Ident, Ident, Imm, End, X',
    processor = 'Ubits(16, 3), LitList(12, "CONTROL_REGS"), LitList(8, "CONTROL_REGS"), Ubits(5, 3), R(0)',
)

tlentry(['MSR'],
    '(<systemreg>|S<op0>_<op1>_<Cn>_<Cm>_<op2>),<Xt>', (('o0', 1, 19), ('op1', 3, 16), ('CRn', 4, 12), ('CRm', 4, 8), ('op2', 3, 5), ('Rt', 5, 0)),
    matcher   = 'Imm, X',
    processor = 'Ubits(5, 15), R(0)',
)

tlentry(['SYSL'],
    '<Xt>,#<op1>,<Cn>,<Cm>,#<op2>', (('op1', 3, 16), ('CRn', 4, 12), ('CRm', 4, 8), ('op2', 3, 5), ('Rt', 5, 0)),
    matcher   = 'X, Imm, Ident, Ident, Imm',
    processor = 'R(0), Ubits(16, 3), LitList(12, "CONTROL_REGS"), LitList(8, "CONTROL_REGS"), Ubits(5, 3)',
)

tlentry(['MRS'],
    '<Xt>,(<systemreg>|S<op0>_<op1>_<Cn>_<Cm>_<op2>)', (('o0', 1, 19), ('op1', 3, 16), ('CRn', 4, 12), ('CRm', 4, 8), ('op2', 3, 5), ('Rt', 5, 0)),
    matcher   = 'X, Imm',
    processor = 'R(0), Ubits(5, 15)',
)

tlentry(['AT'],
    '<at_op>,<Xt>', (('op1', 3, 16), ('op2', 3, 5), ('Rt', 5, 0)),
    matcher   = 'Ident, X',
    processor = 'LitList(5, "AT_OPS"), R(0)',
)

tlentry(['DC'],
    '<dc_op>,<Xt>', (('op1', 3, 16), ('CRm', 4, 8), ('op2', 3, 5), ('Rt', 5, 0)),
    matcher   = 'Ident, X',
    processor = 'LitList(5, "DC_OPS"), R(0)',
)

tlentry(['IC'],
    '<ic_op>{,<Xt>}', (('op1', 3, 16), ('CRm', 4, 8), ('op2', 3, 5), ('Rt', 5, 0)),
    matcher   = 'Lit("ivau"), X',
    processor = 'R(0), Static(5, 0b01101110101001)',
    matchers  =['Ident'],
    processors=['LitList(5, "IC_OPS"), Static(0, 0b11111)']
)

tlentry(['DMB', 'DSB'],
    '<option>|#<imm>', (('CRm', 4, 8),),
    matcher   = 'Ident',
    processor = 'LitList(8, "BARRIER_OPS")',
    matchers  =['Imm'],
    processors=['Ubits(8, 4)']
)

tlentry(['MSR'],
    '<pstatefield>,#<imm>', (('op1', 3, 16), ('CRm', 4, 8), ('op2', 3, 5)),
    matcher   = 'Ident, Imm',
    processor = 'LitList(5, "MSR_IMM_OPS"), Ubits(8, 4)',
)

tlentry(['TLBI'],
    '<tlbi_op>{,<Xt>}', (('op1', 3, 16), ('CRm', 4, 8), ('op2', 3, 5), ('Rt', 5, 0)),
    matcher   = 'Ident, End, X',
    processor = 'LitList(5, "TLBI_OPS"), R(0)',
)

tlentry(['PSB', 'TSB'],
    'CSYNC', (),
    matcher   = 'Lit("csync")',
    processor = '',
)

tlentry(['CFP', 'CPP', 'DVP'],
    'RCTX,<Xt>', (('Rt', 5, 0),),
    matcher   = 'Lit("rctx"), X',
    processor = 'R(0)',
)

tlentry(['CLREX'],
    '{#<imm>}', (('CRm', 4, 8),),
    matcher   = 'Imm',
    processor = 'Ubits(8, 4)',
    matchers  =[''],
    processors=['Static(8, 0b1111)']
)

tlentry(['DCPS1', 'DCPS2', 'DCPS3'],
    '{#<imm>}', (('imm16', 16, 5),),
    matcher   = 'End, Imm',
    processor = 'Ubits(5, 16)',
)

tlentry(['ISB'],
    '{<option>|#<imm>}', (('CRm', 4, 8),),
    matcher   = 'Lit("sy")',
    processor = 'Static(8, 0b1111)',
    matchers  =['Imm',
                ''],
    processors=['Ubits(8, 4)',
                'Static(8, 0b1111)'],
)
