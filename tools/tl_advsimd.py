
tlentry(['MOVI'],
    '<Dd>,#<imm>', (('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'D, Imm',
    processor = 'R(0), BUbits(8), BUslice(5, 5, 0), BUslice(16, 3, 5)',
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
    matcher   = 'H, H, VLanes(WORD)',
    processor = 'Unimp',
)

tlentry(['SHA512H', 'SHA512H2'],
    '<Qd>,<Qn>,<Vm>.2D', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Q, Q, VSizedStatic(QWORD, 2)',
    processor = 'Unimp',
)

tlentry(['SHA256H', 'SHA256H2'],
    '<Qd>,<Qn>,<Vm>.4S', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Q, Q, VSizedStatic(DWORD, 4)',
    processor = 'Unimp',
)

tlentry(['SHA1C', 'SHA1M', 'SHA1P'],
    '<Qd>,<Sn>,<Vm>.4S', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Q, S, VSizedStatic(DWORD, 4)',
    processor = 'Unimp',
)

tlentry(['SHA1H'],
    '<Sd>,<Sn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, S',
    processor = 'R(0), R(5)',
)

tlentry(['ABS', 'NEG', 'SQABS', 'SQNEG', 'SUQADD', 'USQADD'],
    '<V><d>,<V><n>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, V',
    processor = 'Unimp',
)

tlentry(['FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FRECPE', 'FRECPX', 'FRSQRTE', 'SCVTF', 'UCVTF'],
    '<V><d>,<V><n>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, V',
    processor = 'Unimp',
)

tlentry(['CMEQ', 'CMGE', 'CMGT', 'CMLE', 'CMLT'],
    '<V><d>,<V><n>,#0', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, V, LitInt(0)',
    processor = 'Unimp',
)

tlentry(['FCMEQ', 'FCMGE', 'FCMGT', 'FCMLE', 'FCMLT'],
    '<V><d>,<V><n>,#0.0', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, V, LitFloat(0.0)',
    processor = 'Unimp',
)

tlentry(['FCVTZS', 'FCVTZU', 'SCVTF', 'UCVTF'],
    '<V><d>,<V><n>,#<fbits>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, V, Imm',
    processor = 'Unimp',
)

tlentry(['SHL', 'SLI', 'SQSHL', 'SQSHLU', 'SRI', 'SRSHR', 'SRSRA', 'SSHR', 'SSRA', 'UQSHL', 'URSHR', 'URSRA', 'USHR', 'USRA'],
    '<V><d>,<V><n>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, V, Imm',
    processor = 'Unimp',
)

tlentry(['ADD', 'CMEQ', 'CMGE', 'CMGT', 'CMHI', 'CMHS', 'CMTST', 'SQADD', 'SQDMULH', 'SQRDMLAH', 'SQRDMLSH', 'SQRDMULH', 'SQRSHL', 'SQSHL', 'SQSUB', 'SRSHL', 'SSHL', 'SUB', 'UQADD', 'UQRSHL', 'UQSHL', 'UQSUB', 'URSHL', 'USHL'],
    '<V><d>,<V><n>,<V><m>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, V, V',
    processor = 'Unimp',
)

tlentry(['FABD', 'FACGE', 'FACGT', 'FCMEQ', 'FCMGE', 'FCMGT', 'FMULX', 'FRECPS', 'FRSQRTS'],
    '<V><d>,<V><n>,<V><m>', (('sz', 1, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, V, V',
    processor = 'Unimp',
)

tlentry(['SQDMULH', 'SQRDMLAH', 'SQRDMLSH', 'SQRDMULH'],
    '<V><d>,<V><n>,<Vm>.<Ts>[<index>]', (('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, V, Unimp',
    processor = 'Unimp',
)

tlentry(['FMLA', 'FMLS', 'FMUL', 'FMULX'],
    '<V><d>,<V><n>,<Vm>.<Ts>[<index>]', (('sz', 1, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, V, Unimp',
    processor = 'Unimp',
)

tlentry(['FMAXNMV', 'FMAXV', 'FMINNMV', 'FMINV'],
    '<V><d>,<Vn>.<T>', (('Q', 1, 30), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, VSized(WORD)',
    processor = 'R(0), R(5), Unimp',
)

tlentry(['ADDV', 'SADDLV', 'SMAXV', 'SMINV', 'UADDLV', 'UMAXV', 'UMINV'],
    '<V><d>,<Vn>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'V, VWild',
    processor = 'Unimp',
)

tlentry(['FMAXNMV', 'FMAXV', 'FMINNMV', 'FMINV'],
    '<V><d>,<Vn>.<T>', (('Q', 1, 30), ('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FADDP', 'FMAXNMP', 'FMAXP', 'FMINNMP', 'FMINP'],
    '<V><d>,<Vn>.<T>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ADDP'],
    '<V><d>,<Vn>.<T>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FADDP', 'FMAXNMP', 'FMAXP', 'FMINNMP', 'FMINP'],
    '<V><d>,<Vn>.<T>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DUP', 'MOV'],
    '<V><d>,<Vn>.<T>[<index>]', (('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SQDMLAL', 'SQDMLSL', 'SQDMULL'],
    '<Va><d>,<Vb><n>,<Vb><m>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SQDMLAL', 'SQDMLSL', 'SQDMULL'],
    '<Va><d>,<Vb><n>,<Vm>.<Ts>[<index>]', (('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SQXTN', 'SQXTUN', 'UQXTN'],
    '<Vb><d>,<Va><n>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVTXN'],
    '<Vb><d>,<Va><n>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SQRSHRN', 'SQRSHRUN', 'SQSHRN', 'SQSHRUN', 'UQRSHRN', 'UQSHRN'],
    '<Vb><d>,<Va><n>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['AESD', 'AESE', 'AESIMC', 'AESMC'],
    '<Vd>.16B,<Vn>.16B', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)',
    processor = 'Unimp',
)

tlentry(['BCAX', 'EOR3'],
    '<Vd>.16B,<Vn>.16B,<Vm>.16B,<Va>.16B', (('Rm', 5, 16), ('Ra', 5, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16), VSizedStatic(BYTE, 16)',
    processor = 'Unimp',
)

tlentry(['FMOV', 'MOVI'],
    '<Vd>.2D,#<imm>', (('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'VSizedStatic(QWORD, 2), Imm',
    processor = 'Unimp',
)

tlentry(['SHA512SU0'],
    '<Vd>.2D,<Vn>.2D', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)',
    processor = 'Unimp',
)

tlentry(['RAX1', 'SHA512SU1'],
    '<Vd>.2D,<Vn>.2D,<Vm>.2D', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2)',
    processor = 'Unimp',
)

tlentry(['XAR'],
    '<Vd>.2D,<Vn>.2D,<Vm>.2D,#<imm6>', (('Rm', 5, 16), ('imm6', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), VSizedStatic(QWORD, 2), Imm',
    processor = 'Unimp',
)

tlentry(['SHA1SU1', 'SHA256SU0', 'SM4E'],
    '<Vd>.4S,<Vn>.4S', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)',
    processor = 'Unimp',
)

tlentry(['SHA1SU0', 'SHA256SU1', 'SM3PARTW1', 'SM3PARTW2', 'SM4EKEY'],
    '<Vd>.4S,<Vn>.4S,<Vm>.4S', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)',
    processor = 'Unimp',
)

tlentry(['SM3SS1'],
    '<Vd>.4S,<Vn>.4S,<Vm>.4S,<Va>.4S', (('Rm', 5, 16), ('Ra', 5, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4)',
    processor = 'Unimp',
)

tlentry(['SM3TT1A', 'SM3TT1B', 'SM3TT2A', 'SM3TT2B'],
    '<Vd>.4S,<Vn>.4S,<Vm>.S[<imm2>]', (('Rm', 5, 16), ('imm2', 2, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VSizedStatic(DWORD, 4), VSizedStatic(DWORD, 4), VLanes(DWORD)',
    processor = 'Unimp',
)

tlentry(['MOVI', 'MVNI'],
    '<Vd>.<T>,#<imm8>,MSL#<amount>', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('cmode', 4, 12), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOVI'],
    '<Vd>.<T>,#<imm8>{,LSL#0}', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOVI', 'MVNI'],
    '<Vd>.<T>,#<imm8>{,LSL#<amount>}', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('cmode', 4, 12), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
    # DUPLICATE ENTRY FOR BOTH MOVI AND MVNI
)

tlentry(['BIC', 'ORR'],
    '<Vd>.<T>,#<imm8>{,LSL#<amount>}', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
    # DUPLICATE ENTRY FOR BOTH BIC AND ORR
)

tlentry(['FMOV'],
    '<Vd>.<T>,#<imm>', (('Q', 1, 30), ('a', 1, 18), ('b', 1, 17), ('c', 1, 16), ('d', 1, 9), ('e', 1, 8), ('f', 1, 7), ('g', 1, 6), ('h', 1, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
    # DUPLICATE ENTRY FOR FMOV
)

tlentry(['DUP'],
    '<Vd>.<T>,<R><n>', (('Q', 1, 30), ('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOV'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FABS', 'FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FNEG', 'FRECPE', 'FRINTA', 'FRINTI', 'FRINTM', 'FRINTN', 'FRINTP', 'FRINTX', 'FRINTZ', 'FRSQRTE', 'FSQRT', 'MVN', 'NOT', 'RBIT', 'SCVTF', 'UCVTF'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ABS', 'CLS', 'CLZ', 'CNT', 'NEG', 'REV16', 'REV32', 'REV64', 'SQABS', 'SQNEG', 'SUQADD', 'USQADD'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FABS', 'FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FNEG', 'FRECPE', 'FRINTA', 'FRINTI', 'FRINTM', 'FRINTN', 'FRINTP', 'FRINTX', 'FRINTZ', 'FRSQRTE', 'FSQRT', 'SCVTF', 'UCVTF', 'URECPE', 'URSQRTE'],
    '<Vd>.<T>,<Vn>.<T>', (('Q', 1, 30), ('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CMEQ', 'CMGE', 'CMGT', 'CMLE', 'CMLT'],
    '<Vd>.<T>,<Vn>.<T>,#0', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCMEQ', 'FCMGE', 'FCMGT', 'FCMLE', 'FCMLT'],
    '<Vd>.<T>,<Vn>.<T>,#0.0', (('Q', 1, 30), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCMEQ', 'FCMGE', 'FCMGT', 'FCMLE', 'FCMLT'],
    '<Vd>.<T>,<Vn>.<T>,#0.0', (('Q', 1, 30), ('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVTZS', 'FCVTZU', 'SCVTF', 'UCVTF'],
    '<Vd>.<T>,<Vn>.<T>,#<fbits>', (('Q', 1, 30), ('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SHL', 'SLI', 'SQSHL', 'SQSHLU', 'SRI', 'SRSHR', 'SRSRA', 'SSHR', 'SSRA', 'UQSHL', 'URSHR', 'URSRA', 'USHR', 'USRA'],
    '<Vd>.<T>,<Vn>.<T>,#<shift>', (('Q', 1, 30), ('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['AND', 'BIC', 'BIF', 'BIT', 'BSL', 'EOR', 'FABD', 'FACGE', 'FACGT', 'FADD', 'FADDP', 'FCMEQ', 'FCMGE', 'FCMGT', 'FDIV', 'FMAX', 'FMAXNM', 'FMAXNMP', 'FMAXP', 'FMIN', 'FMINNM', 'FMINNMP', 'FMINP', 'FMLA', 'FMLS', 'FMUL', 'FMULX', 'FRECPS', 'FRSQRTS', 'FSUB', 'ORN', 'ORR'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ADD', 'ADDP', 'CMEQ', 'CMGE', 'CMGT', 'CMHI', 'CMHS', 'CMTST', 'MLA', 'MLS', 'MUL', 'PMUL', 'SABA', 'SABD', 'SHADD', 'SHSUB', 'SMAX', 'SMAXP', 'SMIN', 'SMINP', 'SQADD', 'SQDMULH', 'SQRDMLAH', 'SQRDMLSH', 'SQRDMULH', 'SQRSHL', 'SQSHL', 'SQSUB', 'SRHADD', 'SRSHL', 'SSHL', 'SUB', 'TRN1', 'TRN2', 'UABA', 'UABD', 'UHADD', 'UHSUB', 'UMAX', 'UMAXP', 'UMIN', 'UMINP', 'UQADD', 'UQRSHL', 'UQSHL', 'UQSUB', 'URHADD', 'URSHL', 'USHL', 'UZP1', 'UZP2', 'ZIP1', 'ZIP2'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FABD', 'FACGE', 'FACGT', 'FADD', 'FADDP', 'FCMEQ', 'FCMGE', 'FCMGT', 'FDIV', 'FMAX', 'FMAXNM', 'FMAXNMP', 'FMAXP', 'FMIN', 'FMINNM', 'FMINNMP', 'FMINP', 'FMLA', 'FMLS', 'FMUL', 'FMULX', 'FRECPS', 'FRSQRTS', 'FSUB'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>', (('Q', 1, 30), ('sz', 1, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['EXT'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>,#<index>', (('Q', 1, 30), ('Rm', 5, 16), ('imm4', 4, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCADD'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>,#<rotate>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('rot', 1, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCMLA'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<T>,#<rotate>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('rot', 2, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MLA', 'MLS', 'MUL', 'SQDMULH', 'SQRDMLAH', 'SQRDMLSH', 'SQRDMULH'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<Ts>[<index>]', (('Q', 1, 30), ('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMLA', 'FMLS', 'FMUL', 'FMULX'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<Ts>[<index>]', (('Q', 1, 30), ('sz', 1, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCMLA'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.<Ts>[<index>],#<rotate>', (('Q', 1, 30), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('rot', 2, 13), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
    # DUPLICATE FOR FCMLA
)

tlentry(['FMLA', 'FMLS', 'FMUL', 'FMULX'],
    '<Vd>.<T>,<Vn>.<T>,<Vm>.H[<index>]', (('Q', 1, 30), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DUP'],
    '<Vd>.<T>,<Vn>.<Ts>[<index>]', (('Q', 1, 30), ('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SADALP', 'SADDLP', 'UADALP', 'UADDLP'],
    '<Vd>.<Ta>,<Vn>.<Tb>', (('Q', 1, 30), ('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SDOT', 'UDOT'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.4B[<index>]', (('Q', 1, 30), ('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMLAL', 'FMLAL2', 'FMLSL', 'FMLSL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SDOT', 'UDOT'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('Q', 1, 30), ('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMLAL', 'FMLAL2', 'FMLSL', 'FMLSL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.H[<index>]', (('Q', 1, 30), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['TBL', 'TBX'],
    '<Vd>.<Ta>,{<Vn>.16B,<Vn+1>.16B,<Vn+2>.16B,<Vn+3>.16B},<Vm>.<Ta>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['TBL', 'TBX'],
    '<Vd>.<Ta>,{<Vn>.16B,<Vn+1>.16B,<Vn+2>.16B},<Vm>.<Ta>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['TBL', 'TBX'],
    '<Vd>.<Ta>,{<Vn>.16B,<Vn+1>.16B},<Vm>.<Ta>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['TBL', 'TBX'],
    '<Vd>.<Ta>,{<Vn>.16B},<Vm>.<Ta>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['INS', 'MOV'],
    '<Vd>.<Ts>[<index1>],<Vn>.<Ts>[<index2>]', (('imm5', 5, 16), ('imm4', 4, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['INS', 'MOV'],
    '<Vd>.<Ts>[<index>],<R><n>', (('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SMOV', 'UMOV'],
    '<Wd>,<Vn>.<Ts>[<index>]', (('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOV'],
    '<Wd>,<Vn>.S[<index>]', (('imm5', 1, 20), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SMOV', 'UMOV'],
    '<Xd>,<Vn>.<Ts>[<index>]', (('imm5', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOV'],
    '<Xd>,<Vn>.D[<index>]', (('imm5', 1, 20), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SADDW', 'SADDW2', 'SSUBW', 'SSUBW2', 'UADDW', 'UADDW2', 'USUBW', 'USUBW2'],
    '<Vd>.<Ta>,<Vn>.<Ta>,<Vm>.<Tb>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SXTL', 'SXTL2', 'UXTL', 'UXTL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>', (('immh', 4, 19), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVTL', 'FCVTL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SSHLL', 'SSHLL2', 'USHLL', 'USHLL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SHLL', 'SHLL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,#<shift>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PMULL', 'PMULL2', 'SABAL', 'SABAL2', 'SABDL', 'SABDL2', 'SADDL', 'SADDL2', 'SMLAL', 'SMLAL2', 'SMLSL', 'SMLSL2', 'SMULL', 'SMULL2', 'SQDMLAL', 'SQDMLAL2', 'SQDMLSL', 'SQDMLSL2', 'SQDMULL', 'SQDMULL2', 'SSUBL', 'SSUBL2', 'UABAL', 'UABAL2', 'UABDL', 'UABDL2', 'UADDL', 'UADDL2', 'UMLAL', 'UMLAL2', 'UMLSL', 'UMLSL2', 'UMULL', 'UMULL2', 'USUBL', 'USUBL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Tb>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SMLAL', 'SMLAL2', 'SMLSL', 'SMLSL2', 'SMULL', 'SMULL2', 'SQDMLAL', 'SQDMLAL2', 'SQDMLSL', 'SQDMLSL2', 'SQDMULL', 'SQDMULL2', 'UMLAL', 'UMLAL2', 'UMLSL', 'UMLSL2', 'UMULL', 'UMULL2'],
    '<Vd>.<Ta>,<Vn>.<Tb>,<Vm>.<Ts>[<index>]', (('size', 2, 22), ('L', 1, 21), ('M', 1, 20), ('Rm', 4, 16), ('H', 1, 11), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SQXTN', 'SQXTN2', 'SQXTUN', 'SQXTUN2', 'UQXTN', 'UQXTN2', 'XTN', 'XTN2'],
    '<Vd>.<Tb>,<Vn>.<Ta>', (('size', 2, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVTN', 'FCVTN2', 'FCVTXN', 'FCVTXN2'],
    '<Vd>.<Tb>,<Vn>.<Ta>', (('sz', 1, 22), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['RSHRN', 'RSHRN2', 'SHRN', 'SHRN2', 'SQRSHRN', 'SQRSHRN2', 'SQRSHRUN', 'SQRSHRUN2', 'SQSHRN', 'SQSHRN2', 'SQSHRUN', 'SQSHRUN2', 'UQRSHRN', 'UQRSHRN2', 'UQSHRN', 'UQSHRN2'],
    '<Vd>.<Tb>,<Vn>.<Ta>,#<shift>', (('immh', 4, 19), ('immb', 3, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ADDHN', 'ADDHN2', 'RADDHN', 'RADDHN2', 'RSUBHN', 'RSUBHN2', 'SUBHN', 'SUBHN2'],
    '<Vd>.<Tb>,<Vn>.<Ta>,<Vm>.<Ta>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD4', 'LD4R', 'ST1', 'ST4'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>,<Vt4>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(4), RefBase',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD4', 'LD4R', 'ST1', 'ST4'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>,<Vt4>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(4), RefBase, X',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD4', 'LD4R', 'ST1', 'ST4'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>,<Vt4>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(4), RefBase, Imm',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD3', 'LD3R', 'ST1', 'ST3'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(3), RefBase',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD3', 'LD3R', 'ST1', 'ST3'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(3), RefBase, X',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD3', 'LD3R', 'ST1', 'ST3'],
    '{<Vt>.<T>,<Vt2>.<T>,<Vt3>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(3), RefBase, Imm',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD2', 'LD2R', 'ST1', 'ST2'],
    '{<Vt>.<T>,<Vt2>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(2), RefBase',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD2', 'LD2R', 'ST1', 'ST2'],
    '{<Vt>.<T>,<Vt2>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(2), RefBase, X',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD2', 'LD2R', 'ST1', 'ST2'],
    '{<Vt>.<T>,<Vt2>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(2), RefBase, Imm',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD1R', 'ST1'],
    '{<Vt>.<T>},[<Xn|SP>]', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(1), RefBase',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD1R', 'ST1'],
    '{<Vt>.<T>},[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(1), RefBase, X',
    processor = 'Unimp',
)

tlentry(['LD1', 'LD1R', 'ST1'],
    '{<Vt>.<T>},[<Xn|SP>],<imm>', (('Q', 1, 30), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegList(1), RefBase, Imm',
    processor = 'Unimp',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B,<Vt4>.B}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, BYTE), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B,<Vt4>.B}[<index>],[<Xn|SP>],#4', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, BYTE), RefBase, LitInt(4)',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B,<Vt4>.B}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, BYTE), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, BYTE), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B}[<index>],[<Xn|SP>],#3', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, BYTE), RefBase, LitInt(3)',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.B,<Vt2>.B,<Vt3>.B}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, BYTE), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.B,<Vt2>.B}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, BYTE), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.B,<Vt2>.B}[<index>],[<Xn|SP>],#2', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, BYTE), RefBase, LitInt(2)',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.B,<Vt2>.B}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, BYTE), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.B}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, BYTE), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.B}[<index>],[<Xn|SP>],#1', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, BYTE), RefBase, LitInt(1)',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.B}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, BYTE), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11, 10]), R(5), R(16)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D,<Vt4>.D}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, QWORD), RefBase',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D,<Vt4>.D}[<index>],[<Xn|SP>],#32', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, QWORD), RefBase, LitInt(32)',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D,<Vt4>.D}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, QWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30]), R(5), R(16)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, QWORD), RefBase',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D}[<index>],[<Xn|SP>],#24', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, QWORD), RefBase, LitInt(24)',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.D,<Vt2>.D,<Vt3>.D}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, QWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30]), R(5), R(16)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.D,<Vt2>.D}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, QWORD), RefBase',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.D,<Vt2>.D}[<index>],[<Xn|SP>],#16', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, QWORD), RefBase, LitInt(16)',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.D,<Vt2>.D}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, QWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30]), R(5), R(16)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.D}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, QWORD), RefBase',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.D}[<index>],[<Xn|SP>],#8', (('Q', 1, 30), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, QWORD), RefBase, LitInt(8)',
    processor = 'R(0), Ufields(&[30]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.D}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, QWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30]), R(5), R(16)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H,<Vt4>.H}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, WORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H,<Vt4>.H}[<index>],[<Xn|SP>],#8', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, WORD), RefBase, LitInt(8)',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H,<Vt4>.H}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, WORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5), R(16)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, WORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H}[<index>],[<Xn|SP>],#6', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, WORD), RefBase, LitInt(6)',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.H,<Vt2>.H,<Vt3>.H}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, WORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5), R(16)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.H,<Vt2>.H}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, WORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.H,<Vt2>.H}[<index>],[<Xn|SP>],#4', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, WORD), RefBase, LitInt(4)',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.H,<Vt2>.H}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, WORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5), R(16)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.H}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, WORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.H}[<index>],[<Xn|SP>],#2', (('Q', 1, 30), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, WORD), RefBase, LitInt(2)',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.H}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('size', 2, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, WORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12, 11]), R(5), R(16)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S,<Vt4>.S}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, DWORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S,<Vt4>.S}[<index>],[<Xn|SP>],#16', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, DWORD), RefBase, LitInt(16)',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD4', 'ST4'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S,<Vt4>.S}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(4, DWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12]), R(5), R(16)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, DWORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S}[<index>],[<Xn|SP>],#12', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, DWORD), RefBase, LitInt(12)',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD3', 'ST3'],
    '{<Vt>.S,<Vt2>.S,<Vt3>.S}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(3, DWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12]), R(5), R(16)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.S,<Vt2>.S}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, DWORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.S,<Vt2>.S}[<index>],[<Xn|SP>],#8', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, DWORD), RefBase, LitInt(8)',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD2', 'ST2'],
    '{<Vt>.S,<Vt2>.S}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(2, DWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12]), R(5), R(16)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.S}[<index>],[<Xn|SP>]', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, DWORD), RefBase',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.S}[<index>],[<Xn|SP>],#4', (('Q', 1, 30), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, DWORD), RefBase, LitInt(4)',
    processor = 'R(0), Ufields(&[30, 12]), R(5)',
)

tlentry(['LD1', 'ST1'],
    '{<Vt>.S}[<index>],[<Xn|SP>],<Xm>', (('Q', 1, 30), ('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'RegListLanes(1, DWORD), RefBase, X',
    processor = 'R(0), Ufields(&[30, 12]), R(5), R(16)',
)
