
tlentry(['MOVI'],
    '<Dd>,#<imm>', (('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'D, Imm',
    processor = 'R(0), Special(5, STRETCHED_IMMEDIATE)', # stretched 64-bit immediate
)

tlentry(['FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FRECPE', 'FRECPX', 'FRSQRTE', 'SCVTF', 'UCVTF'],
    '<Hd>,<Hn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, H',
    processor = 'R(0), R(5)',
)

tlentry(['FCMEQ', 'FCMGE', 'FCMGT', 'FCMLE', 'FCMLT'],
    '<Hd>,<Hn>,#0.0', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, H, LitFloat(0.0)',
    processor = 'R(0), R(5)',
)

tlentry(['FABD', 'FACGE', 'FACGT', 'FCMEQ', 'FCMGE', 'FCMGT', 'FMULX', 'FRECPS', 'FRSQRTS'],
    '<Hd>,<Hn>,<Hm>', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, H, H',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['FMLA', 'FMLS', 'FMUL', 'FMULX'],
    '<Hd>,<Hn>,<Vm>.H[<index>]', (('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, H, VElement(WORD)',
    processor = 'R(0), R(5), R4(16), Ufields(&[11, 21, 20])',
)

tlentry(['SHA512H', 'SHA512H2'],
    '<Qd>,<Qn>,<Vm>.2D', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Q, Q, VStatic(QWORD, 2)',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['SHA256H', 'SHA256H2'],
    '<Qd>,<Qn>,<Vm>.4S', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Q, Q, VStatic(DWORD, 4)',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['SHA1C', 'SHA1M', 'SHA1P'],
    '<Qd>,<Sn>,<Vm>.4S', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Q, S, VStatic(DWORD, 4)',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['SHA1H'],
    '<Sd>,<Sn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, S',
    processor = 'R(0), R(5)',
)

tlentry(['ABS', 'NEG'],
    '<V><d>,<V><n>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'D, D',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b11)',
    ],
)

tlentry(['SQABS', 'SQNEG', 'SUQADD', 'USQADD'],
    '<V><d>,<V><n>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'B, B',
        'H, H',
        'S, S',
        'D, D',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b00)',
        'R(0), R(5), Static(22, 0b01)',
        'R(0), R(5), Static(22, 0b10)',
        'R(0), R(5), Static(22, 0b11)',
    ],
)

tlentry(['FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FRECPE', 'FRECPX', 'FRSQRTE', 'SCVTF', 'UCVTF'],
    '<V><d>,<V><n>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'S, S',
        'D, D',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b0)',
        'R(0), R(5), Static(22, 0b1)',
    ],
)

tlentry(['CMEQ', 'CMGE', 'CMGT', 'CMLE', 'CMLT'],
    '<V><d>,<V><n>,#0', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'D, D, LitInt(0)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b11)',
    ],
)

tlentry(['FCMEQ', 'FCMGE', 'FCMGT', 'FCMLE', 'FCMLT'],
    '<V><d>,<V><n>,#0.0', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'S, S, LitFloat(0.0)',
        'D, D, LitFloat(0.0)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b0)',
        'R(0), R(5), Static(22, 0b1)',
    ],
)

tlentry(['FCVTZS', 'FCVTZU', 'SCVTF', 'UCVTF'],
    '<V><d>,<V><n>,#<fbits>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'H, H, Imm',
        'S, S, Imm',
        'D, D, Imm',
    ],
    processors = [
        'R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)',
        'R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)',
        'R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)',
    ],
)

tlentry(['SHL', 'SLI'],
    '<V><d>,<V><n>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'D, D, Imm',
    ],
    processors = [
        'R(0), R(5), Ubits(16, 6), Static(22, 0b1)',
    ],
)

tlentry(['SQSHL', 'SQSHLU', 'UQSHL'],
    '<V><d>,<V><n>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'B, B, Imm',
        'H, H, Imm',
        'S, S, Imm',
        'D, D, Imm',
    ],
    processors = [
        'R(0), R(5), Ubits(16, 3), Static(19, 0b1)',
        'R(0), R(5), Ubits(16, 4), Static(20, 0b1)',
        'R(0), R(5), Ubits(16, 5), Static(21, 0b1)',
        'R(0), R(5), Ubits(16, 6), Static(22, 0b1)',
    ],
)

tlentry(['SRI', 'SRSHR', 'SRSRA', 'SSHR', 'SSRA', 'URSHR', 'URSRA', 'USHR', 'USRA'],
    '<V><d>,<V><n>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'D, D, Imm',
    ],
    processors = [
        'R(0), R(5), BUrange(1, 64), Usub(16, 7, 128)',
    ],
)

tlentry(['ADD', 'CMEQ', 'CMGE', 'CMGT', 'CMHI', 'CMHS', 'CMTST', 'SRSHL', 'SSHL', 'SUB', 'URSHL', 'USHL'],
    '<V><d>,<V><n>,<V><m>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'D, D, D',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b11)',
    ],
)

tlentry(['SQADD', 'SQRSHL', 'SQSHL', 'SQSUB', 'UQADD', 'UQRSHL', 'UQSHL', 'UQSUB'],
    '<V><d>,<V><n>,<V><m>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'B, B, B',
        'H, H, H',
        'S, S, S',
        'D, D, D',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b00)',
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
        'R(0), R(5), R(16), Static(22, 0b11)',
    ],
)

tlentry(['SQDMULH', 'SQRDMLAH', 'SQRDMLSH', 'SQRDMULH'],
    '<V><d>,<V><n>,<V><m>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'H, H, H',
        'S, S, S',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
    ],
)

tlentry(['FABD', 'FACGE', 'FACGT', 'FCMEQ', 'FCMGE', 'FCMGT', 'FMULX', 'FRECPS', 'FRSQRTS'],
    '<V><d>,<V><n>,<V><m>', (('sz', 1, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'S, S, S',
        'D, D, D',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b0)',
        'R(0), R(5), R(16), Static(22, 0b1)',
    ],
)

tlentry(['SQDMULH', 'SQRDMLAH', 'SQRDMLSH', 'SQRDMULH'],
    '<V><d>,<V><n>,<Vm>.<Ts>[<index>]', (('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'H, H, VElement(WORD)',
        'S, S, VElement(DWORD)',
    ],
    processors = [
        'R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Static(22, 0b01)',
        'R(0), R(5), R(16), Ufields(&[11, 21]), Static(22, 0b10)',
    ],
)

tlentry(['FMLA', 'FMLS', 'FMUL', 'FMULX'],
    '<V><d>,<V><n>,<Vm>.<Ts>[<index>]', (('sz', 1, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'S, S, VElement(DWORD)',
        'D, D, VElement(QWORD)',
    ],
    processors = [
        'R(0), R(5), R(16), Ufields(&[11, 21]), Static(22, 0b0)',
        'R(0), R(5), R(16), Ufields(&[11]), Static(22, 0b1)',
    ],
)

tlentry(['FMAXNMV', 'FMAXV', 'FMINNMV', 'FMINV'],
    '<V><d>,<Vn>.<T>', (('Q', 1, 30), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'H, V(WORD)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30)',
    ],
)

tlentry(['ADDV', 'SMAXV', 'SMINV', 'UMAXV', 'UMINV'],
    '<V><d>,<Vn>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'B, V(BYTE)',
        'H, V(WORD)',
        'S, VStatic(DWORD, 4)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b00)',
        'R(0), R(5), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), Rwidth(30), Static(22, 0b10)',
    ],
)

tlentry(['SADDLV', 'UADDLV'],
    '<V><d>,<Vn>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'H, V(BYTE)',
        'S, V(WORD)',
        'D, VStatic(DWORD, 4)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b00)',
        'R(0), R(5), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), Rwidth(30), Static(22, 0b10)',
    ],
)

tlentry(['FMAXNMV', 'FMAXV', 'FMINNMV', 'FMINV'],
    '<V><d>,<Vn>.<T>', (('Q', 1, 30), ('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'S, VStatic(DWORD, 4)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b0)',
    ],
)

tlentry(['FADDP', 'FMAXNMP', 'FMAXP', 'FMINNMP', 'FMINP'],
    '<V><d>,<Vn>.<T>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'H, VStatic(WORD, 2)',
    ],
    processors = [
        'R(0), R(5)',
    ],
)

tlentry(['ADDP'],
    '<V><d>,<Vn>.<T>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'D, VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b11)',
    ],
)

tlentry(['FADDP', 'FMAXNMP', 'FMAXP', 'FMINNMP', 'FMINP'],
    '<V><d>,<Vn>.<T>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'S, VStatic(DWORD, 2)',
        'D, VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b0)',
        'R(0), R(5), Static(22, 0b1)',
    ],
)

tlentry(['DUP', 'MOV'],
    '<V><d>,<Vn>.<T>[<index>]', (('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'B, VElement(BYTE)',
        'H, VElement(WORD)',
        'S, VElement(DWORD)',
        'D, VElement(QWORD)',
    ],
    processors = [
        'R(0), R(5), Ubits(17, 4), Static(16, 0b00001)',
        'R(0), R(5), Ubits(18, 3), Static(16, 0b00010)',
        'R(0), R(5), Ubits(19, 2), Static(16, 0b00100)',
        'R(0), R(5), Ubits(20, 1), Static(16, 0b01000)',
    ],
)

tlentry(['SQDMLAL', 'SQDMLSL', 'SQDMULL'],
    '<Va><d>,<Vb><n>,<Vb><m>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'S, H, H',
        'D, S, S',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
    ],
)

tlentry(['SQDMLAL', 'SQDMLSL', 'SQDMULL'],
    '<Va><d>,<Vb><n>,<Vm>.<Ts>[<index>]', (('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'S, H, VElement(WORD)',
        'D, S, VElement(DWORD)',
    ],
    processors = [
        'R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Static(22, 0b01)',
        'R(0), R(5), R(16), Ufields(&[11, 21]), Static(22, 0b10)',
    ],
)

tlentry(['SQXTN', 'SQXTUN', 'UQXTN'],
    '<Vb><d>,<Va><n>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'B, H',
        'H, S',
        'S, D',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b00)',
        'R(0), R(5), Static(22, 0b01)',
        'R(0), R(5), Static(22, 0b10)',
    ],
)

tlentry(['FCVTXN'],
    '<Vb><d>,<Va><n>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'S, D',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b1)',
    ],
)

tlentry(['SQRSHRN', 'SQRSHRUN', 'SQSHRN', 'SQSHRUN', 'UQRSHRN', 'UQSHRN'],
    '<Vb><d>,<Va><n>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'B, H, Imm',
        'H, S, Imm',
        'S, D, Imm',
    ],
    processors = [
        'R(0), R(5), BUrange(1, 8), Usub(16, 4, 16)',
        'R(0), R(5), BUrange(1, 16), Usub(16, 5, 32)',
        'R(0), R(5), BUrange(1, 32), Usub(16, 6, 64)',
    ],
)

tlentry(['AESD', 'AESE', 'AESIMC', 'AESMC'],
    '<Vd>.16B,<Vn>.16B', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(BYTE, 16), VStatic(BYTE, 16)',
    processor = 'R(0), R(5)',
)

tlentry(['BCAX', 'EOR3'],
    '<Vd>.16B,<Vn>.16B,<Vm>.16B,<Va>.16B', (('Rm', 5, 16), ('Ra', 5, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(BYTE, 16), VStatic(BYTE, 16), VStatic(BYTE, 16), VStatic(BYTE, 16)',
    processor = 'R(0), R(5), R(16), R(10)',
)

tlentry(['FMOV'],
    '<Vd>.2D,#<imm>', (('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(QWORD, 2), Imm',
    processor = 'R(0), Special(5, SPLIT_FLOAT_IMMEDIATE)', # split floating-point immediate
)

tlentry(['MOVI'],
    '<Vd>.2D,#<imm>', (('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(QWORD, 2), Imm',
    processor = 'R(0), Special(5, STRETCHED_IMMEDIATE)', # stretched 64-bit immediate
)

tlentry(['SHA512SU0'],
    '<Vd>.2D,<Vn>.2D', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(QWORD, 2), VStatic(QWORD, 2)',
    processor = 'R(0), R(5)',
)

tlentry(['RAX1', 'SHA512SU1'],
    '<Vd>.2D,<Vn>.2D,<Vm>.2D', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(QWORD, 2), VStatic(QWORD, 2), VStatic(QWORD, 2)',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['XAR'],
    '<Vd>.2D,<Vn>.2D,<Vm>.2D,#<imm6>', (('Rm', 5, 16), ('imm6', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(QWORD, 2), VStatic(QWORD, 2), VStatic(QWORD, 2), Imm',
    processor = 'R(0), R(5), R(16), Ubits(10, 6)',
)

tlentry(['SHA1SU1', 'SHA256SU0', 'SM4E'],
    '<Vd>.4S,<Vn>.4S', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(DWORD, 4), VStatic(DWORD, 4)',
    processor = 'R(0), R(5)',
)

tlentry(['SHA1SU0', 'SHA256SU1', 'SM3PARTW1', 'SM3PARTW2', 'SM4EKEY'],
    '<Vd>.4S,<Vn>.4S,<Vm>.4S', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(DWORD, 4), VStatic(DWORD, 4), VStatic(DWORD, 4)',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['SM3SS1'],
    '<Vd>.4S,<Vn>.4S,<Vm>.4S,<Va>.4S', (('Rm', 5, 16), ('Ra', 5, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(DWORD, 4), VStatic(DWORD, 4), VStatic(DWORD, 4), VStatic(DWORD, 4)',
    processor = 'R(0), R(5), R(16), R(10)',
)

tlentry(['SM3TT1A', 'SM3TT1B', 'SM3TT2A', 'SM3TT2B'],
    '<Vd>.4S,<Vn>.4S,<Vm>.S[<imm2>]', (('Rm', 5, 16), ('imm2', 2, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VStatic(DWORD, 4), VStatic(DWORD, 4), VElement(DWORD)',
    processor = 'R(0), R(5), R(16), Ubits(12, 2)',
)

tlentry(['MOVI', 'MVNI'],
    '<Vd>.<T>,#<imm8>,MSL#<amount>', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('cmode', 4, 12), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'V(DWORD), Imm, LitMod(MSL)',
    processor = 'R(0), BUbits(8), Uslice(5, 5, 0), Uslice(16, 3, 5), A, Ulist(12, &[8, 16]), Rwidth(30)',
)

tlentry(['MOVI'],
    '<Vd>.<T>,#<imm8>{,LSL#0}', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'V(BYTE), Imm, End, LitMod(LSL)',
    processor = 'R(0), BUbits(8), Uslice(5, 5, 0), Uslice(16, 3, 5), A, BUbits(0), A, Rwidth(30)',
)

tlentry(['MVNI'],
    '<Vd>.<T>,#<imm8>{,LSL#<amount>}', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('cmode', 4, 12), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), Imm, End, LitMod(LSL)',
        'V(DWORD), Imm, End, LitMod(LSL)',
    ],
    processors = [
        'R(0), BUbits(8), Uslice(5, 5, 0), Uslice(16, 3, 5), A, Ulist(13, &[0, 8]), Rwidth(30)',
        'R(0), BUbits(8), Uslice(5, 5, 0), Uslice(16, 3, 5), A, Ulist(13, &[0, 8, 16, 24]), Rwidth(30)',
    ],
    bits = [
        '0x10111100000xxx10x001xxxxxxxxxx', # 16-bit shifted immediate
        '0x10111100000xxx0xx001xxxxxxxxxx', # 32-bit shifted immediate 
    ],
)

tlentry(['MOVI'],
    '<Vd>.<T>,#<imm8>{,LSL#<amount>}', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('cmode', 4, 12), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), Imm, End, LitMod(LSL)',
        'V(DWORD), Imm, End, LitMod(LSL)',
    ],
    processors = [
        'R(0), BUbits(8), Uslice(5, 5, 0), Uslice(16, 3, 5), A, Ulist(13, &[0, 8]), Rwidth(30)',
        'R(0), BUbits(8), Uslice(5, 5, 0), Uslice(16, 3, 5), A, Ulist(13, &[0, 8, 16, 24]), Rwidth(30)',
    ],
    bits = [
        '0x00111100000xxx10x001xxxxxxxxxx', # 16-bit shifted immediate
        '0x00111100000xxx0xx001xxxxxxxxxx', # 32-bit shifted immediate 
    ],
)

tlentry(['ORR'],
    '<Vd>.<T>,#<imm8>{,LSL#<amount>}', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), Imm, End, LitMod(LSL)',
        'V(DWORD), Imm, End, LitMod(LSL)',
    ],
    processors = [
        'R(0), BUbits(8), Uslice(5, 5, 0), Uslice(16, 3, 5), A, Ulist(13, &[0, 8]), Rwidth(30)',
        'R(0), BUbits(8), Uslice(5, 5, 0), Uslice(16, 3, 5), A, Ulist(13, &[0, 8, 16, 24]), Rwidth(30)',
    ],
    bits = [
        '0x00111100000xxx10x101xxxxxxxxxx', # 16-bit variant
        '0x00111100000xxx0xx101xxxxxxxxxx', # 32-bit variant
    ],
)

tlentry(['BIC'],
    '<Vd>.<T>,#<imm8>{,LSL#<amount>}', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), Imm, End, LitMod(LSL)',
        'V(DWORD), Imm, End, LitMod(LSL)',
    ],
    processors = [
        'R(0), BUbits(8), Uslice(5, 5, 0), Uslice(16, 3, 5), A, Ulist(13, &[0, 8]), Rwidth(30)',
        'R(0), BUbits(8), Uslice(5, 5, 0), Uslice(16, 3, 5), A, Ulist(13, &[0, 8, 16, 24]), Rwidth(30)',
    ],
    bits = [
        '0x10111100000xxx10x101xxxxxxxxxx', # 16-bit variant
        '0x10111100000xxx0xx101xxxxxxxxxx', # 32-bit variant
    ],
)

tlentry(['FMOV'],
    '<Vd>.<T>,#<imm>', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), Imm',
        'V(DWORD), Imm',
    ],
    processors = [
        'R(0), Special(5, SPLIT_FLOAT_IMMEDIATE), Rwidth(30)', # split floating-point immediate
        'R(0), Special(5, SPLIT_FLOAT_IMMEDIATE), Rwidth(30)', # split floating-point immediate
    ],
    bits = [
        '0x00111100000xxx111111xxxxxxxxxx', # 16-bit variant
        '0x00111100000xxx111101xxxxxxxxxx', # 32-bit variant
    ],
)

tlentry(['DUP'],
    '<Vd>.<T>,<R><n>', (('Q', 1, 30), ('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), W',
        'V(WORD), W',
        'V(DWORD), W',
        'VStatic(QWORD, 2), X',
    ],
    processors = [
        'R(0), Rwidth(30), R(5), Static(16, 0b00001)',
        'R(0), Rwidth(30), R(5), Static(16, 0b00010)',
        'R(0), Rwidth(30), R(5), Static(16, 0b00100)',
        'R(0), Rwidth(30), R(5), Static(16, 0b01000)',
    ],
)

tlentry(['MOV'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE)',
    ],
    processors = [
        'R(0), R(5), C, R(16), Rwidth(30)',
    ],
)

tlentry(['FABS', 'FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FNEG', 'FRECPE', 'FRINTA', 'FRINTI', 'FRINTM', 'FRINTN', 'FRINTP', 'FRINTX', 'FRINTZ', 'FRSQRTE', 'FSQRT', 'SCVTF', 'UCVTF'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), V(WORD)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30)',
    ],
)

tlentry(['MVN', 'NOT', 'RBIT'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30)',
    ],
)

tlentry(['ABS', 'NEG', 'SQABS', 'SQNEG', 'SUQADD', 'USQADD'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE)',
        'V(WORD), V(WORD)',
        'V(DWORD), V(DWORD)',
        'VStatic(QWORD, 2), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b00)',
        'R(0), R(5), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), Rwidth(30), Static(22, 0b10)',
        'R(0), R(5), Rwidth(30), Static(22, 0b11)',
    ],
)

tlentry(['CLS', 'CLZ'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE)',
        'V(WORD), V(WORD)',
        'V(DWORD), V(DWORD)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b00)',
        'R(0), R(5), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), Rwidth(30), Static(22, 0b10)',
    ],
)

tlentry(['CNT', 'REV16'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b00)',
    ],
)

tlentry(['REV32'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE)',
        'V(WORD), V(WORD)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b00)',
        'R(0), R(5), Rwidth(30), Static(22, 0b01)',
    ],
)

tlentry(['REV64'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE)',
        'V(WORD), V(WORD)',
        'V(DWORD), V(DWORD)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b00)',
        'R(0), R(5), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), Rwidth(30), Static(22, 0b10)',
    ],
)

tlentry(['FABS', 'FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FNEG', 'FRECPE', 'FRINTA', 'FRINTI', 'FRINTM', 'FRINTN', 'FRINTP', 'FRINTX', 'FRINTZ', 'FRSQRTE', 'FSQRT', 'SCVTF', 'UCVTF'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(DWORD), V(DWORD)',
        'VStatic(QWORD, 2), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b0)',
        'R(0), R(5), Rwidth(30), Static(22, 0b1)',
    ],
)

tlentry(['URECPE', 'URSQRTE'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(DWORD), V(DWORD)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b0)',
    ],
)

tlentry(['CMEQ', 'CMGE', 'CMGT', 'CMLE', 'CMLT'],
    '<Vd>.<T>,<Vn>.<T>,#0', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE), LitInt(0)',
        'V(WORD), V(WORD), LitInt(0)',
        'V(DWORD), V(DWORD), LitInt(0)',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), LitInt(0)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b00)',
        'R(0), R(5), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), Rwidth(30), Static(22, 0b10)',
        'R(0), R(5), Rwidth(30), Static(22, 0b11)',
    ],
)

tlentry(['FCMEQ', 'FCMGE', 'FCMGT', 'FCMLE', 'FCMLT'],
    '<Vd>.<T>,<Vn>.<T>,#0.0', (('Q', 1, 30), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), V(WORD), LitFloat(0.0)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30)',
    ],
)

tlentry(['FCMEQ', 'FCMGE', 'FCMGT', 'FCMLE', 'FCMLT'],
    '<Vd>.<T>,<Vn>.<T>,#0.0', (('Q', 1, 30), ('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(DWORD), V(DWORD), LitFloat(0.0)',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), LitFloat(0.0)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b0)',
        'R(0), R(5), Rwidth(30), Static(22, 0b1)',
    ],
)

tlentry(['FCVTZS', 'FCVTZU', 'SCVTF', 'UCVTF'],
    '<Vd>.<T>,<Vn>.<T>,#<fbits>', (('Q', 1, 30), ('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), V(WORD), Imm',
        'V(DWORD), V(DWORD), Imm',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), Imm',
    ],
    processors = [
        'R(0), R(5), Usub(16, 4, 16), Rwidth(30), Static(19, 0b0010)',
        'R(0), R(5), Usub(16, 5, 32), Rwidth(30), Static(19, 0b0100)',
        'R(0), R(5), Usub(16, 6, 64), Rwidth(30), Static(19, 0b1000)',
    ],
)

tlentry(['SHL', 'SLI', 'SQSHL', 'SQSHLU', 'UQSHL'],
    '<Vd>.<T>,<Vn>.<T>,#<shift>', (('Q', 1, 30), ('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE), Imm',
        'V(WORD), V(WORD), Imm',
        'V(DWORD), V(DWORD), Imm',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), Imm',
    ],
    processors = [
        'R(0), R(5), Ubits(16, 3), Rwidth(30), Static(19, 0b0001)',
        'R(0), R(5), Ubits(16, 4), Rwidth(30), Static(19, 0b0010)',
        'R(0), R(5), Ubits(16, 5), Rwidth(30), Static(19, 0b0100)',
        'R(0), R(5), Ubits(16, 6), Rwidth(30), Static(19, 0b1000)',
    ],
)

tlentry(['SRI', 'SRSHR', 'SRSRA', 'SSHR', 'SSRA', 'URSHR', 'URSRA', 'USHR', 'USRA'],
    '<Vd>.<T>,<Vn>.<T>,#<shift>', (('Q', 1, 30), ('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE), Imm',
        'V(WORD), V(WORD), Imm',
        'V(DWORD), V(DWORD), Imm',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), Imm',
    ],
    processors = [
        'R(0), R(5), Usub(16, 3, 8), Rwidth(30), Static(19, 0b0001)',
        'R(0), R(5), Usub(16, 4, 16), Rwidth(30), Static(19, 0b0010)',
        'R(0), R(5), Usub(16, 5, 32), Rwidth(30), Static(19, 0b0100)',
        'R(0), R(5), Usub(16, 6, 64), Rwidth(30), Static(19, 0b1000)',
    ],
)

tlentry(['AND', 'BIC', 'BIF', 'BIT', 'BSL', 'EOR', 'ORN', 'ORR'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V(BYTE), V(BYTE), V(BYTE)',
    processor = 'R(0), R(5), R(16), Rwidth(30)',
)

tlentry(['FABD', 'FACGE', 'FACGT', 'FADD', 'FADDP', 'FCMEQ', 'FCMGE', 'FCMGT', 'FDIV', 'FMAX', 'FMAXNM', 'FMAXNMP', 'FMAXP', 'FMIN', 'FMINNM', 'FMINNMP', 'FMINP', 'FMLA', 'FMLS', 'FMUL', 'FMULX', 'FRECPS', 'FRSQRTS', 'FSUB'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V(WORD), V(WORD), V(WORD)',
    processor = 'R(0), R(5), R(16), Rwidth(30)',
)

tlentry(['ADD', 'ADDP', 'CMEQ', 'CMGE', 'CMGT', 'CMHI', 'CMHS', 'CMTST', 'SQADD', 'SQRSHL', 'SQSHL', 'SQSUB', 'SRSHL', 'SSHL', 'SUB', 'TRN1', 'TRN2', 'UQADD', 'UQRSHL', 'UQSHL', 'UQSUB', 'URSHL', 'USHL', 'UZP1', 'UZP2', 'ZIP1', 'ZIP2'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE), V(BYTE)',
        'V(WORD), V(WORD), V(WORD)',
        'V(DWORD), V(DWORD), V(DWORD)',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b00)',
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b10)',
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b11)',
    ],
)

tlentry(['MLA', 'MLS', 'MUL', 'SABA', 'SABD', 'SHADD', 'SHSUB', 'SMAX', 'SMAXP', 'SMIN', 'SMINP', 'SRHADD', 'UABA', 'UABD', 'UHADD', 'UHSUB', 'UMAX', 'UMAXP', 'UMIN', 'UMINP', 'URHADD'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE), V(BYTE)',
        'V(WORD), V(WORD), V(WORD)',
        'V(DWORD), V(DWORD), V(DWORD)',
    ],
    processors = [
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b00)',
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b10)',
    ],
)

tlentry(['SQDMULH', 'SQRDMLAH', 'SQRDMLSH', 'SQRDMULH'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), V(WORD), V(WORD)',
        'V(DWORD), V(DWORD), V(DWORD)',
    ],
    processors = [
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b10)',
    ],
)

tlentry(['PMUL'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), V(BYTE), V(BYTE)',
    ],
    processors = [
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b00)',
    ],
)

tlentry(['FABD', 'FACGE', 'FACGT', 'FADD', 'FADDP', 'FCMEQ', 'FCMGE', 'FCMGT', 'FDIV', 'FMAX', 'FMAXNM', 'FMAXNMP', 'FMAXP', 'FMIN', 'FMINNM', 'FMINNMP', 'FMINP', 'FMLA', 'FMLS', 'FMUL', 'FMULX', 'FRECPS', 'FRSQRTS', 'FSUB'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>', (('Q', 1, 30), ('sz', 1, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(DWORD), V(DWORD), V(DWORD)',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b0)',
        'R(0), R(5), R(16), Rwidth(30), Static(22, 0b1)', # TODO check all ops
    ]
)

tlentry(['EXT'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>,#<index>', (('Q', 1, 30), ('Rm', 5, 16), ('imm4', 4, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(BYTE, 8), VStatic(BYTE, 8), VStatic(BYTE, 8), Imm',
        'VStatic(BYTE, 16), VStatic(BYTE, 16), VStatic(BYTE, 16), Imm',
    ],
    processors = [
        'R(0), R(5), R(16), Ubits(11, 3), Static(30, 0b0)',
        'R(0), R(5), R(16), Ubits(11, 4), Static(30, 0b1)',
    ],
)

tlentry(['FCADD'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>,#<rotate>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('rot', 1, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), V(WORD), V(WORD), Imm',
        'V(DWORD), V(DWORD), V(DWORD), Imm',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), VStatic(QWORD, 2), Imm',
    ],
    processors = [
        'R(0), R(5), R(16), Ulist(12, &[90, 270]), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), R(16), Ulist(12, &[90, 270]), Rwidth(30), Static(22, 0b10)',
        'R(0), R(5), R(16), Ulist(12, &[90, 270]), Rwidth(30), Static(22, 0b11)',
    ],
)

tlentry(['FCMLA'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>,#<rotate>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('rot', 2, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), V(WORD), V(WORD), Imm',
        'V(DWORD), V(DWORD), V(DWORD), Imm',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), VStatic(QWORD, 2), Imm',
    ],
    processors = [
        'R(0), R(5), R(16), Ulist(11, &[0, 90, 180, 270]), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), R(16), Ulist(11, &[0, 90, 180, 270]), Rwidth(30), Static(22, 0b10)',
        'R(0), R(5), R(16), Ulist(11, &[0, 90, 180, 270]), Rwidth(30), Static(22, 0b11)',
    ],
)

tlentry(['MLA', 'MLS', 'MUL', 'SQDMULH', 'SQRDMLAH', 'SQRDMLSH', 'SQRDMULH'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<Ts>[<index>]', (('Q', 1, 30), ('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), V(WORD), VElement(WORD)',
        'V(DWORD), V(DWORD), VElement(DWORD)',
    ],
    processors = [
        'R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30), Static(22, 0b10)',
    ],
)

tlentry(['FMLA', 'FMLS', 'FMUL', 'FMULX'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<Ts>[<index>]', (('Q', 1, 30), ('sz', 1, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(DWORD), V(DWORD), VElement(DWORD)',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), VElement(QWORD)',
    ],
    processors = [
        'R(0), R(5), R(16), Ufields(&[11, 21]), Rwidth(30), Static(22, 0b0)',
        'R(0), R(5), R(16), Ufields(&[11]), Rwidth(30), Static(22, 0b1)',
    ],
)

tlentry(['FCMLA'], # duplicate definition
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<Ts>[<index>],#<rotate>', (('Q', 1, 30), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('rot', 2, 13), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 4), VStatic(WORD, 4), VElement(WORD), Imm',
        'VStatic(WORD, 8), VStatic(WORD, 8), VElement(WORD), Imm',
        'VStatic(DWORD, 4), VStatic(DWORD, 4), VElement(DWORD), Imm',
    ],
    processors = [
        'R(0), R(5), R(16), Ufields(&[21]), Ulist(13, &[0, 90, 180, 270]), Static(30, 0b0)',
        'R(0), R(5), R(16), Ufields(&[11, 21]), Ulist(13, &[0, 90, 180, 270]), Static(30, 0b1)',
        'R(0), R(5), R(16), Ufields(&[11]), Ulist(13, &[0, 90, 180, 270]), Rwidth(30)',
    ],
    bits = [
        '0x10111101xxxxxx0xx1x0xxxxxxxxxx',
        '0x10111101xxxxxx0xx1x0xxxxxxxxxx',
        '0x10111110xxxxxx0xx1x0xxxxxxxxxx',
    ],
)

tlentry(['FMLA', 'FMLS', 'FMUL', 'FMULX'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.H[<index>]', (('Q', 1, 30), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), V(WORD), VElement(WORD)',
    ],
    processors = [
        'R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Rwidth(30)',
    ],
)

tlentry(['DUP'],
    '<Vd>.<T>,<Vn>.<Ts>[<index>]', (('Q', 1, 30), ('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(BYTE), VElement(BYTE)',
        'V(WORD), VElement(WORD)',
        'V(DWORD), VElement(DWORD)',
        'VStatic(QWORD, 2), VElement(QWORD)',
    ],
    processors = [
        'R(0), R(5), Ubits(17, 4), Rwidth(30), Static(16, 0b00001)',
        'R(0), R(5), Ubits(18, 3), Rwidth(30), Static(16, 0b00010)',
        'R(0), R(5), Ubits(19, 2), Rwidth(30), Static(16, 0b00100)',
        'R(0), R(5), Ubits(20, 1), Rwidth(30), Static(16, 0b01000)',
    ],
)

tlentry(['SADALP', 'SADDLP', 'UADALP', 'UADDLP'],
    '<Vd>.<Ta>,<Vn>.<Tb>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'V(WORD), V(BYTE)',
        'V(DWORD), V(WORD)',
        'V(QWORD), V(DWORD)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(22, 0b00)',
        'R(0), R(5), Rwidth(30), Static(22, 0b01)',
        'R(0), R(5), Rwidth(30), Static(22, 0b10)',
    ],
)

tlentry(['SDOT', 'UDOT'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.4B[<index>]', (('Q', 1, 30), ('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 2), VStatic(BYTE, 8), VStaticElement(BYTE, 4)',
        'VStatic(DWORD, 4), VStatic(BYTE, 16), VStaticElement(BYTE, 4)',
    ],
    processors = [
        'R(0), R(5), R(16), Ufields(&[11, 21]), Static(30, 0b0), Static(22, 0b10)',
        'R(0), R(5), R(16), Ufields(&[11, 21]), Static(30, 0b1), Static(22, 0b10)',
    ],
)

tlentry(['FMLAL', 'FMLAL2', 'FMLSL', 'FMLSL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 2), VStatic(WORD, 2), VStatic(WORD, 2)',
        'VStatic(DWORD, 4), VStatic(WORD, 4), VStatic(WORD, 4)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(30, 0b0)',
        'R(0), R(5), R(16), Static(30, 0b1)',
    ],
)

tlentry(['SDOT', 'UDOT'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 2), VStatic(BYTE, 8), VStatic(BYTE, 8)',
        'VStatic(DWORD, 4), VStatic(BYTE, 16), VStatic(BYTE, 16)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(30, 0b0), Static(22, 0b10)',
        'R(0), R(5), R(16), Static(30, 0b1), Static(22, 0b10)',
    ],
)

tlentry(['FMLAL', 'FMLAL2', 'FMLSL', 'FMLSL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.H[<index>]', (('Q', 1, 30), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 2), VStatic(WORD, 2), VElement(WORD)',
        'VStatic(DWORD, 4), VStatic(WORD, 4), VElement(WORD)',
    ],
    processors = [
        'R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Static(30, 0b0)',
        'R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Static(30, 0b1)',
    ],
)

tlentry(['TBL', 'TBX'],
    '<Vd>.<Ta>,{<Vn>.16B,<Vn+1>.16B,<Vn+2>.16B,<Vn+3>.16B},<Vm>.<Ta>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V(BYTE), RegListStatic(4, BYTE, 16), V(BYTE)',
    processor = 'R(0), R(5), R(16), Rwidth(30)',
)

tlentry(['TBL', 'TBX'],
    '<Vd>.<Ta>,{<Vn>.16B,<Vn+1>.16B,<Vn+2>.16B},<Vm>.<Ta>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V(BYTE), RegListStatic(3, BYTE, 16), V(BYTE)',
    processor = 'R(0), R(5), R(16), Rwidth(30)',
)

tlentry(['TBL', 'TBX'],
    '<Vd>.<Ta>,{<Vn>.16B,<Vn+1>.16B},<Vm>.<Ta>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V(BYTE), RegListStatic(2, BYTE, 16), V(BYTE)',
    processor = 'R(0), R(5), R(16), Rwidth(30)',
)

tlentry(['TBL', 'TBX'],
    '<Vd>.<Ta>,{<Vn>.16B},<Vm>.<Ta>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V(BYTE), RegListStatic(1, BYTE, 16), V(BYTE)',
    processor = 'R(0), R(5), R(16), Rwidth(30)',
)

tlentry(['INS', 'MOV'],
    '<Vd>.<Ts>[<index1>],<Vn>.<Ts>[<index2>]', (('imm5', 5, 16), ('imm4', 4, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VElement(BYTE), VElement(BYTE)',
        'VElement(WORD), VElement(WORD)',
        'VElement(DWORD), VElement(DWORD)',
        'VElement(QWORD), VElement(QWORD)',
    ],
    processors = [
        'R(0), Ubits(17, 4), R(5), Ubits(11, 4), Static(16, 0b00001)',
        'R(0), Ubits(18, 3), R(5), Ubits(12, 3), Static(16, 0b00010)',
        'R(0), Ubits(19, 2), R(5), Ubits(13, 2), Static(16, 0b00100)',
        'R(0), Ubits(20, 1), R(5), Ubits(14, 1), Static(16, 0b01000)',
    ],
)

tlentry(['INS', 'MOV'],
    '<Vd>.<Ts>[<index>],<R><n>', (('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VElement(BYTE), W',
        'VElement(WORD), W',
        'VElement(DWORD), W',
        'VElement(QWORD), X',
    ],
    processors = [
        'R(0), Ubits(17, 4), R(5), Static(16, 0b00001)',
        'R(0), Ubits(18, 3), R(5), Static(16, 0b00010)',
        'R(0), Ubits(19, 2), R(5), Static(16, 0b00100)',
        'R(0), Ubits(20, 1), R(5), Static(16, 0b01000)',
    ],
)

tlentry(['SMOV'],
    '<Wd>,<Vn>.<Ts>[<index>]', (('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'W, VElement(BYTE)',
        'W, VElement(WORD)',
    ],
    processors = [
        'R(0), R(5), Ubits(17, 4), Static(16, 0b00001)',
        'R(0), R(5), Ubits(18, 3), Static(16, 0b00010)',
    ],
)

tlentry(['UMOV'],
    '<Wd>,<Vn>.<Ts>[<index>]', (('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'W, VElement(BYTE)',
        'W, VElement(WORD)',
        'W, VElement(DWORD)',
    ],
    processors = [
        'R(0), R(5), Ubits(17, 4), Static(16, 0b00001)',
        'R(0), R(5), Ubits(18, 3), Static(16, 0b00010)',
        'R(0), R(5), Ubits(19, 2), Static(16, 0b00100)',
    ],
)

tlentry(['MOV'],
    '<Wd>,<Vn>.S[<index>]', (('imm5', 1, 20), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'W, VElement(DWORD)',
    ],
    processors = [
        'R(0), R(5), Ubits(19, 2), Static(16, 0b00100)',
    ],
)

tlentry(['SMOV'],
    '<Xd>,<Vn>.<Ts>[<index>]', (('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'X, VElement(BYTE)',
        'X, VElement(WORD)',
        'X, VElement(DWORD)',
    ],
    processors = [
        'R(0), R(5), Ubits(17, 4), Static(16, 0b00001)',
        'R(0), R(5), Ubits(18, 3), Static(16, 0b00010)',
        'R(0), R(5), Ubits(19, 2), Static(16, 0b00100)',
    ],
)

tlentry(['UMOV'],
    '<Xd>,<Vn>.<Ts>[<index>]', (('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'X, VElement(QWORD)',
    ],
    processors = [
        'R(0), R(5), Ubits(20, 1), Static(16, 0b01000)',
    ],
)

tlentry(['MOV'],
    '<Xd>,<Vn>.D[<index>]', (('imm5', 1, 20), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'X, VElement(QWORD)',
    ],
    processors = [
        'R(0), R(5), Ubits(20, 1), Static(16, 0b01000)',
    ],
)

tlentry(['SADDW', 'SSUBW', 'UADDW', 'USUBW'],
    '<Vd>.<Ta>,<Vn>.<Ta>,<Vm>.<Tb>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(WORD, 8), VStatic(BYTE, 8)',
        'VStatic(DWORD, 4), VStatic(DWORD, 4), VStatic(WORD, 4)',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), VStatic(DWORD, 2)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b00)',
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
    ],
)

tlentry(['SADDW2', 'SSUBW2', 'UADDW2', 'USUBW2'],
    '<Vd>.<Ta>,<Vn>.<Ta>,<Vm>.<Tb>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(WORD, 8), VStatic(BYTE, 16)',
        'VStatic(DWORD, 4), VStatic(DWORD, 4), VStatic(WORD, 8)',
        'VStatic(QWORD, 2), VStatic(QWORD, 2), VStatic(DWORD, 4)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b00)',
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
    ],
)

tlentry(['SXTL', 'UXTL'],
    '<Vd>.<Ta>,<Vn>.<Tb>', (('immh', 4, 19), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(BYTE, 8)',
        'VStatic(DWORD, 4), VStatic(WORD, 4)',
        'VStatic(QWORD, 2), VStatic(DWORD, 2)',
    ],
    processors = [
        'R(0), R(5), Static(19, 0b0001)',
        'R(0), R(5), Static(19, 0b0010)',
        'R(0), R(5), Static(19, 0b0100)',
    ],
)

tlentry(['SXTL2', 'UXTL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>', (('immh', 4, 19), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(BYTE, 16)',
        'VStatic(DWORD, 4), VStatic(WORD, 8)',
        'VStatic(QWORD, 2), VStatic(DWORD, 4)',
    ],
    processors = [
        'R(0), R(5), Static(19, 0b0001)',
        'R(0), R(5), Static(19, 0b0010)',
        'R(0), R(5), Static(19, 0b0100)',
    ],
)

tlentry(['FCVTL'],
    '<Vd>.<Ta>,<Vn>.<Tb>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 4), VStatic(WORD, 4)',
        'VStatic(QWORD, 2), VStatic(DWORD, 2)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b0)',
        'R(0), R(5), Static(22, 0b1)',
    ],
)

tlentry(['FCVTL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 4), VStatic(WORD, 8)',
        'VStatic(QWORD, 2), VStatic(DWORD, 4)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b0)',
        'R(0), R(5), Static(22, 0b1)',
    ],
)

tlentry(['SSHLL', 'USHLL'],
    '<Vd>.<Ta>,<Vn>.<Tb>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(BYTE, 8), Imm',
        'VStatic(DWORD, 4), VStatic(WORD, 4), Imm',
        'VStatic(QWORD, 2), VStatic(DWORD, 2), Imm',
    ],
    processors = [
        'R(0), R(5), Ubits(16, 3), Static(19, 0b0001)',
        'R(0), R(5), Ubits(16, 4), Static(19, 0b0010)',
        'R(0), R(5), Ubits(16, 5), Static(19, 0b0100)',
    ],
)

tlentry(['SSHLL2', 'USHLL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(BYTE, 16), Imm',
        'VStatic(DWORD, 4), VStatic(WORD, 8), Imm',
        'VStatic(QWORD, 2), VStatic(DWORD, 4), Imm',
    ],
    processors = [
        'R(0), R(5), Ubits(16, 3), Static(19, 0b0001)',
        'R(0), R(5), Ubits(16, 4), Static(19, 0b0010)',
        'R(0), R(5), Ubits(16, 5), Static(19, 0b0100)',
    ],
)

tlentry(['SHLL'],
    '<Vd>.<Ta>,<Vn>.<Tb>,#<shift>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(BYTE, 8), LitInt(8)',
        'VStatic(DWORD, 4), VStatic(WORD, 4), LitInt(16)',
        'VStatic(QWORD, 2), VStatic(DWORD, 2), LitInt(32)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b00)',
        'R(0), R(5), Static(22, 0b01)',
        'R(0), R(5), Static(22, 0b10)',
    ],
)

tlentry(['SHLL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,#<shift>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(BYTE, 16), LitInt(8)',
        'VStatic(DWORD, 4), VStatic(WORD, 8), LitInt(16)',
        'VStatic(QWORD, 2), VStatic(DWORD, 4), LitInt(32)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b00)',
        'R(0), R(5), Static(22, 0b01)',
        'R(0), R(5), Static(22, 0b10)',
    ],
)

tlentry(['PMULL'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(BYTE, 8), VStatic(BYTE, 8)',
        'VStatic(OWORD, 1), VStatic(QWORD, 1), VStatic(QWORD, 1)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b00)',
        'R(0), R(5), R(16), Static(22, 0b11)',
    ],
)

tlentry(['PMULL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(BYTE, 16), VStatic(BYTE, 16)',
        'VStatic(OWORD, 1), VStatic(QWORD, 2), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b00)',
        'R(0), R(5), R(16), Static(22, 0b11)',
    ],
)

tlentry(['SABAL', 'SABDL', 'SADDL', 'SMLAL', 'SMLSL', 'SMULL', 'SSUBL', 'UABAL', 'UABDL', 'UADDL', 'UMLAL', 'UMLSL', 'UMULL', 'USUBL'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(BYTE, 8), VStatic(BYTE, 8)',
        'VStatic(DWORD, 4), VStatic(WORD, 4), VStatic(WORD, 4)',
        'VStatic(QWORD, 2), VStatic(DWORD, 2), VStatic(DWORD, 2)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b00)',
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
    ],
)

tlentry(['SABAL2', 'SABDL2', 'SADDL2', 'SMLAL2', 'SMLSL2', 'SMULL2', 'SSUBL2', 'UABAL2', 'UABDL2', 'UADDL2', 'UMLAL2', 'UMLSL2', 'UMULL2', 'USUBL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(WORD, 8), VStatic(BYTE, 16), VStatic(BYTE, 16)',
        'VStatic(DWORD, 4), VStatic(WORD, 8), VStatic(WORD, 8)',
        'VStatic(QWORD, 2), VStatic(DWORD, 4), VStatic(DWORD, 4)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b00)',
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
    ],
)

tlentry(['SQDMLAL', 'SQDMLSL', 'SQDMULL'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 4), VStatic(WORD, 4), VStatic(WORD, 4)',
        'VStatic(QWORD, 2), VStatic(DWORD, 2), VStatic(DWORD, 2)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
    ],
)

tlentry(['SQDMLAL2', 'SQDMLSL2', 'SQDMULL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 4), VStatic(WORD, 8), VStatic(WORD, 8)',
        'VStatic(QWORD, 2), VStatic(DWORD, 4), VStatic(DWORD, 4)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
    ],
)

tlentry(['SMLAL', 'SMLSL', 'SMULL', 'SQDMLAL', 'SQDMLSL', 'SQDMULL', 'UMLAL', 'UMLSL', 'UMULL'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Ts>[<index>]', (('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 4), VStatic(WORD, 4), VElement(WORD)',
        'VStatic(QWORD, 2), VStatic(DWORD, 2), VElement(DWORD)',
    ],
    processors = [
        'R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Static(22, 0b01)',
        'R(0), R(5), R(16), Ufields(&[11, 21]), Static(22, 0b10)',
    ],
)

tlentry(['SMLAL2', 'SMLSL2', 'SMULL2', 'SQDMLAL2', 'SQDMLSL2', 'SQDMULL2', 'UMLAL2', 'UMLSL2', 'UMULL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Ts>[<index>]', (('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 4), VStatic(WORD, 8), VElement(WORD)',
        'VStatic(QWORD, 2), VStatic(DWORD, 4), VElement(DWORD)',
    ],
    processors = [
        'R(0), R(5), R4(16), Ufields(&[11, 21, 20]), Static(22, 0b01)',
        'R(0), R(5), R(16), Ufields(&[11, 21]), Static(22, 0b10)',
    ],
)

tlentry(['SQXTN', 'SQXTUN', 'UQXTN', 'XTN'],
    '<Vd>.<Tb>,<Vn>.<Ta>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(BYTE, 8), VStatic(WORD, 8)',
        'VStatic(WORD, 4), VStatic(DWORD, 4)',
        'VStatic(DWORD, 2), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b00)',
        'R(0), R(5), Static(22, 0b01)',
        'R(0), R(5), Static(22, 0b10)',
    ],
)

tlentry(['SQXTN2', 'SQXTUN2', 'UQXTN2', 'XTN2'],
    '<Vd>.<Tb>,<Vn>.<Ta>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(BYTE, 16), VStatic(WORD, 8)',
        'VStatic(WORD, 8), VStatic(DWORD, 4)',
        'VStatic(DWORD, 4), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b00)',
        'R(0), R(5), Static(22, 0b01)',
        'R(0), R(5), Static(22, 0b10)',
    ],
)

tlentry(['FCVTN', 'FCVTXN'],
    '<Vd>.<Tb>,<Vn>.<Ta>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 2), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b1)',
    ],
)

tlentry(['FCVTN2', 'FCVTXN2'],
    '<Vd>.<Tb>,<Vn>.<Ta>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(DWORD, 4), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), Static(22, 0b1)',
    ],
)

tlentry(['RSHRN', 'SHRN', 'SQRSHRN', 'SQRSHRUN', 'SQSHRN', 'SQSHRUN', 'UQRSHRN', 'UQSHRN'],
    '<Vd>.<Tb>,<Vn>.<Ta>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(BYTE, 8), VStatic(WORD, 8), Imm',
        'VStatic(WORD, 4), VStatic(DWORD, 4), Imm',
        'VStatic(DWORD, 2), VStatic(QWORD, 2), Imm',
    ],
    processors = [
        'R(0), R(5), Usub(16, 3, 8), Static(19, 0b0001)',
        'R(0), R(5), Usub(16, 4, 16), Static(19, 0b0010)',
        'R(0), R(5), Usub(16, 5, 32), Static(19, 0b0100)',
    ],
)

tlentry(['RSHRN2', 'SHRN2', 'SQRSHRN2', 'SQRSHRUN2', 'SQSHRN2', 'SQSHRUN2', 'UQRSHRN2', 'UQSHRN2'],
    '<Vd>.<Tb>,<Vn>.<Ta>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(BYTE, 16), VStatic(WORD, 8), Imm',
        'VStatic(WORD, 8), VStatic(DWORD, 4), Imm',
        'VStatic(DWORD, 4), VStatic(QWORD, 2), Imm',
    ],
    processors = [
        'R(0), R(5), Usub(16, 3, 8), Static(19, 0b0001)',
        'R(0), R(5), Usub(16, 4, 16), Static(19, 0b0010)',
        'R(0), R(5), Usub(16, 5, 32), Static(19, 0b0100)',
    ],
)

tlentry(['ADDHN', 'RADDHN', 'RSUBHN', 'SUBHN'],
    '<Vd>.<Tb>,<Vn>.<Ta>,<Vm>.<Ta>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(BYTE, 8), VStatic(WORD, 8), VStatic(WORD, 8)',
        'VStatic(WORD, 4), VStatic(DWORD, 4), VStatic(DWORD, 4)',
        'VStatic(DWORD, 2), VStatic(QWORD, 2), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b00)',
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
    ],
)

tlentry(['ADDHN2', 'RADDHN2', 'RSUBHN2', 'SUBHN2'],
    '<Vd>.<Tb>,<Vn>.<Ta>,<Vm>.<Ta>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matchers = [
        'VStatic(BYTE, 16), VStatic(WORD, 8), VStatic(WORD, 8)',
        'VStatic(WORD, 8), VStatic(DWORD, 4), VStatic(DWORD, 4)',
        'VStatic(DWORD, 4), VStatic(QWORD, 2), VStatic(QWORD, 2)',
    ],
    processors = [
        'R(0), R(5), R(16), Static(22, 0b00)',
        'R(0), R(5), R(16), Static(22, 0b01)',
        'R(0), R(5), R(16), Static(22, 0b10)',
    ],
)

tlentry(['LD1', 'LD4R', 'ST1'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>,<Vt4>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(4, BYTE), RefBase',
        'RegList(4, WORD), RefBase',
        'RegList(4, DWORD), RefBase',
        'RegList(4, QWORD), RefBase',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>,<Vt4>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(4, BYTE), RefBase',
        'RegList(4, WORD), RefBase',
        'RegList(4, DWORD), RefBase',
        'RegListStatic(4, QWORD, 2), RefBase',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'LD4R', 'ST1'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>,<Vt4>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(4, BYTE), RefBase, X',
        'RegList(4, WORD), RefBase, X',
        'RegList(4, DWORD), RefBase, X',
        'RegList(4, QWORD), RefBase, X',
    ],
    processors = [
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>,<Vt4>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(4, BYTE), RefBase, X',
        'RegList(4, WORD), RefBase, X',
        'RegList(4, DWORD), RefBase, X',
        'RegListStatic(4, QWORD, 2), RefBase, X',
    ],
    processors = [
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>,<Vt4>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegListStatic(4, BYTE, 8), RefBase, LitInt(32)',
        'RegListStatic(4, WORD, 4), RefBase, LitInt(32)',
        'RegListStatic(4, DWORD, 2), RefBase, LitInt(32)',
        'RegListStatic(4, QWORD, 1), RefBase, LitInt(32)',
        'RegListStatic(4, BYTE, 16), RefBase, LitInt(64)',
        'RegListStatic(4, WORD, 8), RefBase, LitInt(64)',
        'RegListStatic(4, DWORD, 4), RefBase, LitInt(64)',
        'RegListStatic(4, QWORD, 2), RefBase, LitInt(64)',
    ],
    processors = [
        'R(0), R(5), Static(30, 0b0), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b11)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b11)',
    ],
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>,<Vt4>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegListStatic(4, BYTE, 8), RefBase, LitInt(32)',
        'RegListStatic(4, WORD, 4), RefBase, LitInt(32)',
        'RegListStatic(4, DWORD, 2), RefBase, LitInt(32)',
        'RegListStatic(4, BYTE, 16), RefBase, LitInt(64)',
        'RegListStatic(4, WORD, 8), RefBase, LitInt(64)',
        'RegListStatic(4, DWORD, 4), RefBase, LitInt(64)',
        'RegListStatic(4, QWORD, 2), RefBase, LitInt(64)',
    ],
    processors = [
        'R(0), R(5), Static(30, 0b0), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b11)',
    ],
)

tlentry(['LD4R'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>,<Vt4>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(4, BYTE), RefBase, LitInt(4)',
        'RegList(4, WORD), RefBase, LitInt(8)',
        'RegList(4, DWORD), RefBase, LitInt(16)',
        'RegList(4, QWORD), RefBase, LitInt(32)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'LD3R', 'ST1'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(3, BYTE), RefBase',
        'RegList(3, WORD), RefBase',
        'RegList(3, DWORD), RefBase',
        'RegList(3, QWORD), RefBase',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(3, BYTE), RefBase',
        'RegList(3, WORD), RefBase',
        'RegList(3, DWORD), RefBase',
        'RegListStatic(3, QWORD, 2), RefBase',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'LD3R', 'ST1'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(3, BYTE), RefBase, X',
        'RegList(3, WORD), RefBase, X',
        'RegList(3, DWORD), RefBase, X',
        'RegList(3, QWORD), RefBase, X',
    ],
    processors = [
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(3, BYTE), RefBase, X',
        'RegList(3, WORD), RefBase, X',
        'RegList(3, DWORD), RefBase, X',
        'RegListStatic(3, QWORD, 2), RefBase, X',
    ],
    processors = [
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegListStatic(3, BYTE, 8), RefBase, LitInt(24)',
        'RegListStatic(3, WORD, 4), RefBase, LitInt(24)',
        'RegListStatic(3, DWORD, 2), RefBase, LitInt(24)',
        'RegListStatic(3, QWORD, 1), RefBase, LitInt(24)',
        'RegListStatic(3, BYTE, 16), RefBase, LitInt(48)',
        'RegListStatic(3, WORD, 8), RefBase, LitInt(48)',
        'RegListStatic(3, DWORD, 4), RefBase, LitInt(48)',
        'RegListStatic(3, QWORD, 2), RefBase, LitInt(48)',
    ],
    processors = [
        'R(0), R(5), Static(30, 0b0), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b11)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b11)',
    ],
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegListStatic(3, BYTE, 8), RefBase, LitInt(24)',
        'RegListStatic(3, WORD, 4), RefBase, LitInt(24)',
        'RegListStatic(3, DWORD, 2), RefBase, LitInt(24)',
        'RegListStatic(3, BYTE, 16), RefBase, LitInt(48)',
        'RegListStatic(3, WORD, 8), RefBase, LitInt(48)',
        'RegListStatic(3, DWORD, 4), RefBase, LitInt(48)',
        'RegListStatic(3, QWORD, 2), RefBase, LitInt(48)',
    ],
    processors = [
        'R(0), R(5), Static(30, 0b0), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b11)',
    ],
)

tlentry(['LD3R'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(3, BYTE), RefBase, LitInt(3)',
        'RegList(3, WORD), RefBase, LitInt(6)',
        'RegList(3, DWORD), RefBase, LitInt(12)',
        'RegList(3, QWORD), RefBase, LitInt(24)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'LD2R', 'ST1'],
    '{<Vt>.<T>,<Vt2>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(2, BYTE), RefBase',
        'RegList(2, WORD), RefBase',
        'RegList(2, DWORD), RefBase',
        'RegList(2, QWORD), RefBase',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.<T>,<Vt2>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(2, BYTE), RefBase',
        'RegList(2, WORD), RefBase',
        'RegList(2, DWORD), RefBase',
        'RegListStatic(2, QWORD, 2), RefBase',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'LD2R', 'ST1'],
    '{<Vt>.<T>,<Vt2>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(2, BYTE), RefBase, X',
        'RegList(2, WORD), RefBase, X',
        'RegList(2, DWORD), RefBase, X',
        'RegList(2, QWORD), RefBase, X',
    ],
    processors = [
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.<T>,<Vt2>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(2, BYTE), RefBase, X',
        'RegList(2, WORD), RefBase, X',
        'RegList(2, DWORD), RefBase, X',
        'RegListStatic(2, QWORD, 2), RefBase, X',
    ],
    processors = [
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.<T>,<Vt2>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegListStatic(2, BYTE, 8), RefBase, LitInt(16)',
        'RegListStatic(2, WORD, 4), RefBase, LitInt(16)',
        'RegListStatic(2, DWORD, 2), RefBase, LitInt(16)',
        'RegListStatic(2, QWORD, 1), RefBase, LitInt(16)',
        'RegListStatic(2, BYTE, 16), RefBase, LitInt(32)',
        'RegListStatic(2, WORD, 8), RefBase, LitInt(32)',
        'RegListStatic(2, DWORD, 4), RefBase, LitInt(32)',
        'RegListStatic(2, QWORD, 2), RefBase, LitInt(32)',
    ],
    processors = [
        'R(0), R(5), Static(30, 0b0), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b11)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b11)',
    ],
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.<T>,<Vt2>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegListStatic(2, BYTE, 8), RefBase, LitInt(16)',
        'RegListStatic(2, WORD, 4), RefBase, LitInt(16)',
        'RegListStatic(2, DWORD, 2), RefBase, LitInt(16)',
        'RegListStatic(2, BYTE, 16), RefBase, LitInt(32)',
        'RegListStatic(2, WORD, 8), RefBase, LitInt(32)',
        'RegListStatic(2, DWORD, 4), RefBase, LitInt(32)',
        'RegListStatic(2, QWORD, 2), RefBase, LitInt(32)',
    ],
    processors = [
        'R(0), R(5), Static(30, 0b0), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b11)',
    ],
)

tlentry(['LD2R'],
    '{<Vt>.<T>,<Vt2>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(2, BYTE), RefBase, LitInt(2)',
        'RegList(2, WORD), RefBase, LitInt(4)',
        'RegList(2, DWORD), RefBase, LitInt(8)',
        'RegList(2, QWORD), RefBase, LitInt(16)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'LD1R', 'ST1'],
    '{<Vt>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(1, BYTE), RefBase',
        'RegList(1, WORD), RefBase',
        'RegList(1, DWORD), RefBase',
        'RegList(1, QWORD), RefBase',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'LD1R', 'ST1'],
    '{<Vt>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(1, BYTE), RefBase, X',
        'RegList(1, WORD), RefBase, X',
        'RegList(1, DWORD), RefBase, X',
        'RegList(1, QWORD), RefBase, X',
    ],
    processors = [
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), RNoZr(16), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegListStatic(1, BYTE, 8), RefBase, LitInt(8)',
        'RegListStatic(1, WORD, 4), RefBase, LitInt(8)',
        'RegListStatic(1, DWORD, 2), RefBase, LitInt(8)',
        'RegListStatic(1, QWORD, 1), RefBase, LitInt(8)',
        'RegListStatic(1, BYTE, 16), RefBase, LitInt(16)',
        'RegListStatic(1, WORD, 8), RefBase, LitInt(16)',
        'RegListStatic(1, DWORD, 4), RefBase, LitInt(16)',
        'RegListStatic(1, QWORD, 2), RefBase, LitInt(16)',
    ],
    processors = [
        'R(0), R(5), Static(30, 0b0), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b0), Static(10, 0b11)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b00)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b01)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b10)',
        'R(0), R(5), Static(30, 0b1), Static(10, 0b11)',
    ],
)

tlentry(['LD1R'],
    '{<Vt>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matchers = [
        'RegList(1, BYTE), RefBase, LitInt(1)',
        'RegList(1, WORD), RefBase, LitInt(2)',
        'RegList(1, DWORD), RefBase, LitInt(4)',
        'RegList(1, QWORD), RefBase, LitInt(8)',
    ],
    processors = [
        'R(0), R(5), Rwidth(30), Static(10, 0b00)',
        'R(0), R(5), Rwidth(30), Static(10, 0b01)',
        'R(0), R(5), Rwidth(30), Static(10, 0b10)',
        'R(0), R(5), Rwidth(30), Static(10, 0b11)',
    ],
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B,<Vt4>.B}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, BYTE), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B,<Vt4>.B}[<index>],[<Xn|SP>],#4', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, BYTE), RefBase, LitInt(4)',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B,<Vt4>.B}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, BYTE), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5), RNoZr(16)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, BYTE), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B}[<index>],[<Xn|SP>],#3', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, BYTE), RefBase, LitInt(3)',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, BYTE), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5), RNoZr(16)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.B,<Vt2>.B}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, BYTE), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.B,<Vt2>.B}[<index>],[<Xn|SP>],#2', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, BYTE), RefBase, LitInt(2)',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.B,<Vt2>.B}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, BYTE), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5), RNoZr(16)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.B}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, BYTE), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.B}[<index>],[<Xn|SP>],#1', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, BYTE), RefBase, LitInt(1)',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.B}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, BYTE), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5), RNoZr(16)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D,<Vt4>.D}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, QWORD), RefBase',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D,<Vt4>.D}[<index>],[<Xn|SP>],#32', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, QWORD), RefBase, LitInt(32)',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D,<Vt4>.D}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, QWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30]), R(5), RNoZr(16)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, QWORD), RefBase',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D}[<index>],[<Xn|SP>],#24', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, QWORD), RefBase, LitInt(24)',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, QWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30]), R(5), RNoZr(16)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.D,<Vt2>.D}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, QWORD), RefBase',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.D,<Vt2>.D}[<index>],[<Xn|SP>],#16', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, QWORD), RefBase, LitInt(16)',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.D,<Vt2>.D}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, QWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30]), R(5), RNoZr(16)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.D}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, QWORD), RefBase',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.D}[<index>],[<Xn|SP>],#8', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, QWORD), RefBase, LitInt(8)',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.D}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, QWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30]), R(5), RNoZr(16)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H,<Vt4>.H}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, WORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H,<Vt4>.H}[<index>],[<Xn|SP>],#8', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, WORD), RefBase, LitInt(8)',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H,<Vt4>.H}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, WORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5), RNoZr(16)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, WORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H}[<index>],[<Xn|SP>],#6', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, WORD), RefBase, LitInt(6)',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, WORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5), RNoZr(16)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.H,<Vt2>.H}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, WORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.H,<Vt2>.H}[<index>],[<Xn|SP>],#4', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, WORD), RefBase, LitInt(4)',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.H,<Vt2>.H}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, WORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5), RNoZr(16)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.H}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, WORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.H}[<index>],[<Xn|SP>],#2', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, WORD), RefBase, LitInt(2)',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.H}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, WORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5), RNoZr(16)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S,<Vt4>.S}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, DWORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S,<Vt4>.S}[<index>],[<Xn|SP>],#16', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, DWORD), RefBase, LitInt(16)',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S,<Vt4>.S}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(4, DWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12]), R(5), RNoZr(16)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, DWORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S}[<index>],[<Xn|SP>],#12', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, DWORD), RefBase, LitInt(12)',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(3, DWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12]), R(5), RNoZr(16)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.S,<Vt2>.S}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, DWORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.S,<Vt2>.S}[<index>],[<Xn|SP>],#8', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, DWORD), RefBase, LitInt(8)',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.S,<Vt2>.S}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(2, DWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12]), R(5), RNoZr(16)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.S}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, DWORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.S}[<index>],[<Xn|SP>],#4', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, DWORD), RefBase, LitInt(4)',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.S}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListElement(1, DWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12]), R(5), RNoZr(16)',
)
