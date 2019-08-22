
tlentry(['LDR', 'STR'],
    '<Bt>,[<Xn|SP>,#<simm>]!', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'B, RefPre',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<Bt>,[<Xn|SP>,(<Wm>|<Xm>),<extend>{<amount>}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'B, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 0])',
)

tlentry(['LDR', 'STR'],
    '<Bt>,[<Xn|SP>,<Xm>{,LSL<amount>}]', (('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    forget = True
)

tlentry(['LDR', 'STR'],
    '<Bt>,[<Xn|SP>],#<simm>', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'B, RefBase, Imm',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<Bt>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'B, RefOffset',
    processor = 'R(0), R(5), Ubits(10, 12)',
)

tlentry(['LDUR', 'STUR'],
    '<Bt>,[<Xn|SP>{,#<simm>}]', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'B, RefOffset',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDP', 'STP'],
    '<Dt1>,<Dt2>,[<Xn|SP>,#<imm>]!', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'D, D, RefPre',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 3)',
)

tlentry(['LDP', 'STP'],
    '<Dt1>,<Dt2>,[<Xn|SP>],#<imm>', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'D, D, RefBase, Imm',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 3)',
)

tlentry(['LDNP', 'LDP', 'STNP', 'STP'],
    '<Dt1>,<Dt2>,[<Xn|SP>{,#<imm>}]', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'D, D, RefOffset',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 3)',
)

tlentry(['LDR'],
    '<Dt>,<label>', (('imm19', 19, 5), ('Rt', 5, 0)),
    matcher   = 'D, Offset',
    processor = 'R(0), Offset(BCOND)',
)

tlentry(['LDR', 'STR'],
    '<Dt>,[<Xn|SP>,#<simm>]!', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'D, RefPre',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<Dt>,[<Xn|SP>,(<Wm>|<Xm>){,<extend>{<amount>}}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'D, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 3])',
)

tlentry(['LDR', 'STR'],
    '<Dt>,[<Xn|SP>],#<simm>', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'D, RefBase, Imm',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<Dt>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'D, RefOffset',
    processor = 'R(0), R(5), Uscaled(10, 12, 3)',
)

tlentry(['LDUR', 'STUR'],
    '<Dt>,[<Xn|SP>{,#<simm>}]', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'D, RefOffset',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<Ht>,[<Xn|SP>,#<simm>]!', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'H, RefPre',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<Ht>,[<Xn|SP>,(<Wm>|<Xm>){,<extend>{<amount>}}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'H, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 1])',
)

tlentry(['LDR', 'STR'],
    '<Ht>,[<Xn|SP>],#<simm>', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'H, RefBase, Imm',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<Ht>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'H, RefOffset',
    processor = 'R(0), R(5), Uscaled(10, 12, 1)',
)

tlentry(['LDUR', 'STUR'],
    '<Ht>,[<Xn|SP>{,#<simm>}]', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'H, RefOffset',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDP', 'STP'],
    '<Qt1>,<Qt2>,[<Xn|SP>,#<imm>]!', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'Q, Q, RefPre',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 4)',
)

tlentry(['LDP', 'STP'],
    '<Qt1>,<Qt2>,[<Xn|SP>],#<imm>', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'Q, Q, RefBase, Imm',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 4)',

)

tlentry(['LDNP', 'LDP', 'STNP', 'STP'],
    '<Qt1>,<Qt2>,[<Xn|SP>{,#<imm>}]', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'Q, Q, RefOffset',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 4)',
)

tlentry(['LDR'],
    '<Qt>,<label>', (('imm19', 19, 5), ('Rt', 5, 0)),
    matcher   = 'Q, Offset',
    processor = 'R(0), Offset(BCOND)',
)

tlentry(['LDR', 'STR'],
    '<Qt>,[<Xn|SP>,#<simm>]!', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'Q, RefPre',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<Qt>,[<Xn|SP>,(<Wm>|<Xm>){,<extend>{<amount>}}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'Q, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 4])',
)

tlentry(['LDR', 'STR'],
    '<Qt>,[<Xn|SP>],#<simm>', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'Q, RefBase, Imm',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<Qt>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'Q, RefOffset',
    processor = 'R(0), R(5), Uscaled(10, 12, 4)',
)

tlentry(['LDUR', 'STUR'],
    '<Qt>,[<Xn|SP>{,#<simm>}]', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'Q, RefOffset',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDP', 'STP'],
    '<St1>,<St2>,[<Xn|SP>,#<imm>]!', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'S, S, RefPre',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 2)',
)

tlentry(['LDP', 'STP'],
    '<St1>,<St2>,[<Xn|SP>],#<imm>', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'S, S, RefBase, Imm',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 2)',
)

tlentry(['LDNP', 'LDP', 'STNP', 'STP'],
    '<St1>,<St2>,[<Xn|SP>{,#<imm>}]', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'S, S, RefOffset',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 2)',
)

tlentry(['LDR'],
    '<St>,<label>', (('imm19', 19, 5), ('Rt', 5, 0)),
    matcher   = 'S, Offset',
    processor = 'R(0), Offset(BCOND)',
)

tlentry(['LDR', 'STR'],
    '<St>,[<Xn|SP>,#<simm>]!', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'S, RefPre',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<St>,[<Xn|SP>,(<Wm>|<Xm>){,<extend>{<amount>}}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'S, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 2])',
)

tlentry(['LDR', 'STR'],
    '<St>,[<Xn|SP>],#<simm>', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'S, RefBase, Imm',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDR', 'STR'],
    '<St>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'S, RefOffset',
    processor = 'R(0), R(5), Uscaled(10, 12, 2)',
)

tlentry(['LDUR', 'STUR'],
    '<St>,[<Xn|SP>{,#<simm>}]', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'S, RefOffset',
    processor = 'R(0), R(5), Sbits(12, 9)',
)
