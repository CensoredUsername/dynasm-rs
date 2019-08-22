
tlentry(['FMOV'],
    '<Dd>,#<imm>', (('imm8', 8, 13), ('Rd', 5, 0)),
    matcher   = 'D, Imm',
    processor = 'R(0), Special(13, FLOAT_IMMEDIATE)',
)

tlentry(['FABS', 'FMOV', 'FNEG', 'FRINTA', 'FRINTI', 'FRINTM', 'FRINTN', 'FRINTP', 'FRINTX', 'FRINTZ', 'FSQRT'],
    '<Dd>,<Dn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'D, D',
    processor = 'R(0), R(5)',
)

tlentry(['FADD', 'FDIV', 'FMAX', 'FMAXNM', 'FMIN', 'FMINNM', 'FMUL', 'FNMUL', 'FSUB'],
    '<Dd>,<Dn>,<Dm>', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'D, D, D',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['FMADD', 'FMSUB', 'FNMADD', 'FNMSUB'],
    '<Dd>,<Dn>,<Dm>,<Da>', (('Rm', 5, 16), ('Ra', 5, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'D, D, D, D',
    processor = 'R(0), R(5), R(16), R(10)',
)

tlentry(['FCSEL'],
    '<Dd>,<Dn>,<Dm>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'D, D, D, Cond',
    processor = 'R(0), R(5), R(16), Cond(12)',
)

tlentry(['FCVT'],
    '<Dd>,<Hn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'D, H',
    processor = 'R(0), R(5)',
)

tlentry(['FCVT'],
    '<Dd>,<Sn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'D, S',
    processor = 'R(0), R(5)',
)

tlentry(['SCVTF', 'UCVTF'],
    '<Dd>,<Wn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'D, W',
    processor = 'R(0), R(5)',
)

tlentry(['SCVTF', 'UCVTF'],
    '<Dd>,<Wn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'D, W, Imm',
    processor = 'R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)', # scale = 64 - scale, max 32
)

tlentry(['FMOV', 'SCVTF', 'UCVTF'],
    '<Dd>,<Xn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'D, X',
    processor = 'R(0), R(5)',
)

tlentry(['SCVTF', 'UCVTF'],
    '<Dd>,<Xn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'D, X, Imm',
    processor = 'R(0), R(5), Usub(10, 6, 64)', # scale = 64 - scale
)

tlentry(['FCMP', 'FCMPE'],
    '<Dn>,#0.0', (('Rm', 5, 16), ('Rn', 5, 5)),
    matcher   = 'D, LitFloat(0.0)',
    processor = 'R(5)',
)

tlentry(['FCMP', 'FCMPE'],
    '<Dn>,<Dm>', (('Rm', 5, 16), ('Rn', 5, 5)),
    matcher   = 'D, D',
    processor = 'R(5), R(16)',
)

tlentry(['FCCMP', 'FCCMPE'],
    '<Dn>,<Dm>,#<nzcv>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('nzcv', 4, 0)),
    matcher   = 'D, D, Imm, Cond',
    processor = 'R(5), R(16), Ubits(0, 4), Cond(12)',
)

tlentry(['FMOV'],
    '<Hd>,#<imm>', (('imm8', 8, 13), ('Rd', 5, 0)),
    matcher   = 'H, Imm',
    processor = 'R(0), Special(13, FLOAT_IMMEDIATE)',
)

tlentry(['FCVT'],
    '<Hd>,<Dn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, D',
    processor = 'R(0), R(5)',
)

tlentry(['FABS', 'FMOV', 'FNEG', 'FRINTA', 'FRINTI', 'FRINTM', 'FRINTN', 'FRINTP', 'FRINTX', 'FRINTZ', 'FSQRT'],
    '<Hd>,<Hn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, H',
    processor = 'R(0), R(5)',
)

tlentry(['FADD', 'FDIV', 'FMAX', 'FMAXNM', 'FMIN', 'FMINNM', 'FMUL', 'FNMUL', 'FSUB'],
    '<Hd>,<Hn>,<Hm>', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, H, H',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['FMADD', 'FMSUB', 'FNMADD', 'FNMSUB'],
    '<Hd>,<Hn>,<Hm>,<Ha>', (('Rm', 5, 16), ('Ra', 5, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, H, H, H',
    processor = 'R(0), R(5), R(16), R(10)',
)

tlentry(['FCSEL'],
    '<Hd>,<Hn>,<Hm>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, H, H, Cond',
    processor = 'R(0), R(5), R(16), Cond(12)',
)

tlentry(['FCVT'],
    '<Hd>,<Sn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, S',
    processor = 'R(0), R(5)',
)

tlentry(['FMOV', 'SCVTF', 'UCVTF'],
    '<Hd>,<Wn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, W',
    processor = 'R(0), R(5)',
)

tlentry(['SCVTF', 'UCVTF'],
    '<Hd>,<Wn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, W, Imm',
    processor = 'R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)', # scale = 64 - scale, max 32
)

tlentry(['FMOV', 'SCVTF', 'UCVTF'],
    '<Hd>,<Xn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, X',
    processor = 'R(0), R(5)',
)

tlentry(['SCVTF', 'UCVTF'],
    '<Hd>,<Xn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'H, X, Imm',
    processor = 'R(0), R(5), Usub(10, 6, 64)', # scale = 64 - scale
)

tlentry(['FCMP', 'FCMPE'],
    '<Hn>,#0.0', (('Rm', 5, 16), ('Rn', 5, 5)),
    matcher   = 'H, LitFloat(0.0)',
    processor = 'R(5)',
)

tlentry(['FCMP', 'FCMPE'],
    '<Hn>,<Hm>', (('Rm', 5, 16), ('Rn', 5, 5)),
    matcher   = 'H, H',
    processor = 'R(5), R(16)',
)

tlentry(['FCCMP', 'FCCMPE'],
    '<Hn>,<Hm>,#<nzcv>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('nzcv', 4, 0)),
    matcher   = 'H, H, Imm, Cond',
    processor = 'R(5), R(16), Ubits(0, 4), Cond(12)',
)

tlentry(['FMOV'],
    '<Sd>,#<imm>', (('imm8', 8, 13), ('Rd', 5, 0)),
    matcher   = 'S, Imm',
    processor = 'R(0), Special(13, FLOAT_IMMEDIATE)',
)

tlentry(['FCVT'],
    '<Sd>,<Dn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, D',
    processor = 'R(0), R(5)',
)

tlentry(['FCVT'],
    '<Sd>,<Hn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, H',
    processor = 'R(0), R(5)',
)

tlentry(['FABS', 'FMOV', 'FNEG', 'FRINTA', 'FRINTI', 'FRINTM', 'FRINTN', 'FRINTP', 'FRINTX', 'FRINTZ', 'FSQRT'],
    '<Sd>,<Sn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, S',
    processor = 'R(0), R(5)',
)

tlentry(['FADD', 'FDIV', 'FMAX', 'FMAXNM', 'FMIN', 'FMINNM', 'FMUL', 'FNMUL', 'FSUB'],
    '<Sd>,<Sn>,<Sm>', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, S, S',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['FMADD', 'FMSUB', 'FNMADD', 'FNMSUB'],
    '<Sd>,<Sn>,<Sm>,<Sa>', (('Rm', 5, 16), ('Ra', 5, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, S, S, S',
    processor = 'R(0), R(5), R(16), R(10)',
)

tlentry(['FCSEL'],
    '<Sd>,<Sn>,<Sm>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, S, S, Cond',
    processor = 'R(0), R(5), R(16), Cond(12)',
)

tlentry(['FMOV', 'SCVTF', 'UCVTF'],
    '<Sd>,<Wn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, W',
    processor = 'R(0), R(5)',
)

tlentry(['SCVTF', 'UCVTF'],
    '<Sd>,<Wn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, W, Imm',
    processor = 'R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)', # scale = 64 - scale, max 32
)

tlentry(['SCVTF', 'UCVTF'],
    '<Sd>,<Xn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, X',
    processor = 'R(0), R(5)',
)

tlentry(['SCVTF', 'UCVTF'],
    '<Sd>,<Xn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'S, X, Imm',
    processor = 'R(0), R(5), Usub(10, 6, 64)', # scale = 64 - scale
)

tlentry(['FCMP', 'FCMPE'],
    '<Sn>,#0.0', (('Rm', 5, 16), ('Rn', 5, 5)),
    matcher   = 'S, LitFloat(0.0)',
    processor = 'R(5)',
)

tlentry(['FCMP', 'FCMPE'],
    '<Sn>,<Sm>', (('Rm', 5, 16), ('Rn', 5, 5)),
    matcher   = 'S, S',
    processor = 'R(5), R(16)',
)

tlentry(['FCCMP', 'FCCMPE'],
    '<Sn>,<Sm>,#<nzcv>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('nzcv', 4, 0)),
    matcher   = 'S, S, Imm, Cond',
    processor = 'R(5), R(16), Ubits(0, 4), Cond(12)',
)

tlentry(['FMOV'],
    '<Vd>.D[1],<Xn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'VElementStatic(QWORD, 1), X',
    processor = 'R(0), R(5)',
)

tlentry(['FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FJCVTZS'],
    '<Wd>,<Dn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, D',
    processor = 'R(0), R(5)',
)

tlentry(['FCVTZS', 'FCVTZU'],
    '<Wd>,<Dn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, D, Imm',
    processor = 'R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)', # scale = 64 - scale, max 32
)

tlentry(['FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FMOV'],
    '<Wd>,<Hn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, H',
    processor = 'R(0), R(5)',
)

tlentry(['FCVTZS', 'FCVTZU'],
    '<Wd>,<Hn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, H, Imm',
    processor = 'R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)', # scale = 64 - scale, max 32
)

tlentry(['FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FMOV'],
    '<Wd>,<Sn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, S',
    processor = 'R(0), R(5)',
)

tlentry(['FCVTZS', 'FCVTZU'],
    '<Wd>,<Sn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, S, Imm',
    processor = 'R(0), R(5), BUrange(1, 32), Usub(10, 6, 64)', # scale = 64 - scale, max 32
)

tlentry(['FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FMOV'],
    '<Xd>,<Dn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, D',
    processor = 'R(0), R(5)',
)

tlentry(['FCVTZS', 'FCVTZU'],
    '<Xd>,<Dn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, D, Imm',
    processor = 'R(0), R(5), Usub(10, 6, 64)', # scale = 64 - scale
)

tlentry(['FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU', 'FMOV'],
    '<Xd>,<Hn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, H',
    processor = 'R(0), R(5)',
)

tlentry(['FCVTZS', 'FCVTZU'],
    '<Xd>,<Hn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, H, Imm',
    processor = 'R(0), R(5), Usub(10, 6, 64)', # scale = 64 - scale
)

tlentry(['FCVTAS', 'FCVTAU', 'FCVTMS', 'FCVTMU', 'FCVTNS', 'FCVTNU', 'FCVTPS', 'FCVTPU', 'FCVTZS', 'FCVTZU'],
    '<Xd>,<Sn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, S',
    processor = 'R(0), R(5)',
)

tlentry(['FCVTZS', 'FCVTZU'],
    '<Xd>,<Sn>,#<fbits>', (('scale', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, S, Imm',
    processor = 'R(0), R(5), Usub(10, 6, 64)', # scale = 64 - scale
)

tlentry(['FMOV'],
    '<Xd>,<Vn>.D[1]', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, VElementStatic(QWORD, 1)',
    processor = 'R(0), R(5)',
)
