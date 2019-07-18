
tlentry(['SETFFR'],
    '', (),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SADDV', 'UADDV'],
    '<Dd>,<Pg>,<Zn>.<T>', (('size', 2, 22), ('Pg', 3, 10), ('Zn', 5, 5), ('Vd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CMPLE', 'CMPLO', 'CMPLS', 'CMPLT', 'FACLE', 'FACLT', 'FCMLE', 'FCMLT'],
    '<Pd>.<T>,<Pg>/Z,<Zm>.<T>,<Zn>.<T>', (('size', 2, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCMEQ', 'FCMGE', 'FCMGT', 'FCMLE', 'FCMLT', 'FCMNE'],
    '<Pd>.<T>,<Pg>/Z,<Zn>.<T>,#0.0', (('size', 2, 22), ('Pg', 3, 10), ('Zn', 5, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CMPEQ', 'CMPGE', 'CMPGT', 'CMPLE', 'CMPLT', 'CMPNE'],
    '<Pd>.<T>,<Pg>/Z,<Zn>.<T>,#<imm>', (('size', 2, 22), ('imm5', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CMPHI', 'CMPHS', 'CMPLO', 'CMPLS'],
    '<Pd>.<T>,<Pg>/Z,<Zn>.<T>,#<imm>', (('size', 2, 22), ('imm7', 7, 14), ('Pg', 3, 10), ('Zn', 5, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CMPEQ', 'CMPGE', 'CMPGT', 'CMPHI', 'CMPHS', 'CMPNE', 'FACGE', 'FACGT', 'FCMEQ', 'FCMGE', 'FCMGT', 'FCMNE', 'FCMUO'],
    '<Pd>.<T>,<Pg>/Z,<Zn>.<T>,<Zm>.<T>', (('size', 2, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CMPEQ', 'CMPGE', 'CMPGT', 'CMPHI', 'CMPHS', 'CMPLE', 'CMPLO', 'CMPLS', 'CMPLT', 'CMPNE'],
    '<Pd>.<T>,<Pg>/Z,<Zn>.<T>,<Zm>.D', (('size', 2, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['REV'],
    '<Pd>.<T>,<Pn>.<T>', (('size', 2, 22), ('Pn', 4, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['TRN1', 'TRN2', 'UZP1', 'UZP2', 'ZIP1', 'ZIP2'],
    '<Pd>.<T>,<Pn>.<T>,<Pm>.<T>', (('size', 2, 22), ('Pm', 4, 16), ('Pn', 4, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['WHILELE', 'WHILELO', 'WHILELS', 'WHILELT'],
    '<Pd>.<T>,<R><n>,<R><m>', (('size', 2, 22), ('Rm', 5, 16), ('sf', 1, 12), ('Rn', 5, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PTRUE', 'PTRUES'],
    '<Pd>.<T>{,<pattern>}', (('size', 2, 22), ('pattern', 5, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PFALSE', 'RDFFR'],
    '<Pd>.B', (('Pd', 4, 0),),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SEL'],
    '<Pd>.B,<Pg>,<Pn>.B,<Pm>.B', (('Pm', 4, 16), ('Pg', 4, 10), ('Pn', 4, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['BRKA', 'BRKB'],
    '<Pd>.B,<Pg>/<ZM>,<Pn>.B', (('Pg', 4, 10), ('Pn', 4, 5), ('M', 1, 4), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOV'],
    '<Pd>.B,<Pg>/M,<Pn>.B', (('Pm', 4, 16), ('Pg', 4, 10), ('Pn', 4, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['RDFFR', 'RDFFRS'],
    '<Pd>.B,<Pg>/Z', (('Pg', 4, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['BRKAS', 'BRKBS'],
    '<Pd>.B,<Pg>/Z,<Pn>.B', (('Pg', 4, 10), ('Pn', 4, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOV', 'MOVS', 'NOT', 'NOTS'],
    '<Pd>.B,<Pg>/Z,<Pn>.B', (('Pm', 4, 16), ('Pg', 4, 10), ('Pn', 4, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['AND', 'ANDS', 'BIC', 'BICS', 'BRKPA', 'BRKPAS', 'BRKPB', 'BRKPBS', 'EOR', 'EORS', 'NAND', 'NANDS', 'NOR', 'NORS', 'ORN', 'ORNS', 'ORR', 'ORRS'],
    '<Pd>.B,<Pg>/Z,<Pn>.B,<Pm>.B', (('Pm', 4, 16), ('Pg', 4, 10), ('Pn', 4, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOV', 'MOVS'],
    '<Pd>.B,<Pn>.B', (('Pm', 4, 16), ('Pg', 4, 10), ('Pn', 4, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PUNPKHI', 'PUNPKLO'],
    '<Pd>.H,<Pn>.B', (('Pn', 4, 5), ('Pd', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['BRKN', 'BRKNS'],
    '<Pdm>.B,<Pg>/Z,<Pn>.B,<Pdm>.B', (('Pg', 4, 10), ('Pn', 4, 5), ('Pdm', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PNEXT'],
    '<Pdn>.<T>,<Pg>,<Pdn>.<T>', (('size', 2, 22), ('Pg', 4, 5), ('Pdn', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PFIRST'],
    '<Pdn>.B,<Pg>,<Pdn>.B', (('Pg', 4, 5), ('Pdn', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PTEST'],
    '<Pg>,<Pn>.B', (('Pg', 4, 10), ('Pn', 4, 5)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['WRFFR'],
    '<Pn>.B', (('Pn', 4, 5),),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDR', 'STR'],
    '<Pt>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm9h', 6, 16), ('imm9l', 3, 10), ('Rn', 5, 5), ('Pt', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LASTA', 'LASTB'],
    '<R><d>,<Pg>,<Zn>.<T>', (('size', 2, 22), ('Pg', 3, 10), ('Zn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CLASTA', 'CLASTB'],
    '<R><dn>,<Pg>,<R><dn>,<Zm>.<T>', (('size', 2, 22), ('Pg', 3, 10), ('Zm', 5, 5), ('Rdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CTERMEQ', 'CTERMNE'],
    '<R><n>,<R><m>', (('sz', 1, 22), ('Rm', 5, 16), ('Rn', 5, 5)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ANDV', 'EORV', 'FADDV', 'FMAXNMV', 'FMAXV', 'FMINNMV', 'FMINV', 'LASTA', 'LASTB', 'ORV', 'SMAXV', 'SMINV', 'UMAXV', 'UMINV'],
    '<V><d>,<Pg>,<Zn>.<T>', (('size', 2, 22), ('Pg', 3, 10), ('Zn', 5, 5), ('Vd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CLASTA', 'CLASTB', 'FADDA'],
    '<V><dn>,<Pg>,<V><dn>,<Zm>.<T>', (('size', 2, 22), ('Pg', 3, 10), ('Zm', 5, 5), ('Vdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['UQDECP', 'UQINCP'],
    '<Wdn>,<Pg>.<T>', (('size', 2, 22), ('Pg', 4, 5), ('Rdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['UQDECB', 'UQDECD', 'UQDECH', 'UQDECW', 'UQINCB', 'UQINCD', 'UQINCH', 'UQINCW'],
    '<Wdn>{,<pattern>{,MUL#<imm>}}', (('imm4', 4, 16), ('pattern', 5, 5), ('Rdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['RDVL'],
    '<Xd>,#<imm>', (('imm6', 6, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CNTP'],
    '<Xd>,<Pg>,<Pn>.<T>', (('size', 2, 22), ('Pg', 4, 10), ('Pn', 4, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CNTB', 'CNTD', 'CNTH', 'CNTW'],
    '<Xd>{,<pattern>{,MUL#<imm>}}', (('imm4', 4, 16), ('pattern', 5, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DECP', 'INCP', 'SQDECP', 'SQINCP', 'UQDECP', 'UQINCP'],
    '<Xdn>,<Pg>.<T>', (('size', 2, 22), ('Pg', 4, 5), ('Rdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SQDECP', 'SQINCP'],
    '<Xdn>,<Pg>.<T>,<Wdn>', (('size', 2, 22), ('Pg', 4, 5), ('Rdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SQDECB', 'SQDECD', 'SQDECH', 'SQDECW', 'SQINCB', 'SQINCD', 'SQINCH', 'SQINCW'],
    '<Xdn>,<Wdn>{,<pattern>{,MUL#<imm>}}', (('imm4', 4, 16), ('pattern', 5, 5), ('Rdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DECB', 'DECD', 'DECH', 'DECW', 'INCB', 'INCD', 'INCH', 'INCW', 'SQDECB', 'SQDECD', 'SQDECH', 'SQDECW', 'SQINCB', 'SQINCD', 'SQINCH', 'SQINCW', 'UQDECB', 'UQDECD', 'UQDECH', 'UQDECW', 'UQINCB', 'UQINCD', 'UQINCH', 'UQINCW'],
    '<Xdn>{,<pattern>{,MUL#<imm>}}', (('imm4', 4, 16), ('pattern', 5, 5), ('Rdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ADDPL', 'ADDVL'],
    '<Xd|SP>,<Xn|SP>,#<imm>', (('Rn', 5, 16), ('imm6', 6, 5), ('Rd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOVPRFX'],
    '<Zd>,<Zn>', (('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMOV'],
    '<Zd>.<T>,#0.0', (('size', 2, 22), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DUPM', 'MOV'],
    '<Zd>.<T>,#<const>', (('imm13', 13, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FDUP', 'FMOV'],
    '<Zd>.<T>,#<const>', (('size', 2, 22), ('imm8', 8, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['INDEX'],
    '<Zd>.<T>,#<imm1>,#<imm2>', (('size', 2, 22), ('imm5b', 5, 16), ('imm5', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['INDEX'],
    '<Zd>.<T>,#<imm>,<R><m>', (('size', 2, 22), ('Rm', 5, 16), ('imm5', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DUP', 'MOV'],
    '<Zd>.<T>,#<imm>{,<shift>}', (('size', 2, 22), ('sh', 1, 13), ('imm8', 8, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['COMPACT'],
    '<Zd>.<T>,<Pg>,<Zn>.<T>', (('sz', 1, 22), ('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SEL'],
    '<Zd>.<T>,<Pg>,<Zn>.<T>,<Zm>.<T>', (('size', 2, 22), ('Zm', 5, 16), ('Pg', 4, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CPY', 'MOV'],
    '<Zd>.<T>,<Pg>/<ZM>,#<imm>{,<shift>}', (('size', 2, 22), ('Pg', 4, 16), ('M', 1, 14), ('sh', 1, 13), ('imm8', 8, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOVPRFX'],
    '<Zd>.<T>,<Pg>/<ZM>,<Zn>.<T>', (('size', 2, 22), ('M', 1, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMOV'],
    '<Zd>.<T>,<Pg>/M,#0.0', (('size', 2, 22), ('Pg', 4, 16), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCPY', 'FMOV'],
    '<Zd>.<T>,<Pg>/M,#<const>', (('size', 2, 22), ('Pg', 4, 16), ('imm8', 8, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CPY', 'MOV'],
    '<Zd>.<T>,<Pg>/M,<R><n|SP>', (('size', 2, 22), ('Pg', 3, 10), ('Rn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CPY', 'MOV'],
    '<Zd>.<T>,<Pg>/M,<V><n>', (('size', 2, 22), ('Pg', 3, 10), ('Vn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ABS', 'CLS', 'CLZ', 'CNOT', 'CNT', 'FABS', 'FNEG', 'FRECPX', 'FRINTA', 'FRINTI', 'FRINTM', 'FRINTN', 'FRINTP', 'FRINTX', 'FRINTZ', 'FSQRT', 'NEG', 'NOT', 'RBIT', 'REVB', 'REVH', 'SXTB', 'SXTH', 'UXTB', 'UXTH'],
    '<Zd>.<T>,<Pg>/M,<Zn>.<T>', (('size', 2, 22), ('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOV'],
    '<Zd>.<T>,<Pg>/M,<Zn>.<T>', (('size', 2, 22), ('Zm', 5, 16), ('Pg', 4, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['INDEX'],
    '<Zd>.<T>,<R><n>,#<imm>', (('size', 2, 22), ('imm5', 5, 16), ('Rn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['INDEX'],
    '<Zd>.<T>,<R><n>,<R><m>', (('size', 2, 22), ('Rm', 5, 16), ('Rn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DUP', 'MOV'],
    '<Zd>.<T>,<R><n|SP>', (('size', 2, 22), ('Rn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOV'],
    '<Zd>.<T>,<V><n>', (('imm2', 2, 22), ('tsz', 5, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FEXPA', 'FRECPE', 'FRSQRTE', 'REV'],
    '<Zd>.<T>,<Zn>.<T>', (('size', 2, 22), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ASR', 'LSL', 'LSR'],
    '<Zd>.<T>,<Zn>.<T>,#<const>', (('tszh', 2, 22), ('tszl', 2, 19), ('imm3', 3, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ADD', 'FADD', 'FMUL', 'FRECPS', 'FRSQRTS', 'FSUB', 'FTSMUL', 'FTSSEL', 'SQADD', 'SQSUB', 'SUB', 'TRN1', 'TRN2', 'UQADD', 'UQSUB', 'UZP1', 'UZP2', 'ZIP1', 'ZIP2'],
    '<Zd>.<T>,<Zn>.<T>,<Zm>.<T>', (('size', 2, 22), ('Zm', 5, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ASR', 'LSL', 'LSR'],
    '<Zd>.<T>,<Zn>.<T>,<Zm>.D', (('size', 2, 22), ('Zm', 5, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DUP', 'MOV'],
    '<Zd>.<T>,<Zn>.<T>[<imm>]', (('imm2', 2, 22), ('tsz', 5, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SUNPKHI', 'SUNPKLO', 'UUNPKHI', 'UUNPKLO'],
    '<Zd>.<T>,<Zn>.<Tb>', (('size', 2, 22), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ADR'],
    '<Zd>.<T>,[<Zn>.<T>,<Zm>.<T>{,<mod><amount>}]', (('sz', 1, 22), ('Zm', 5, 16), ('msz', 2, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['TBL'],
    '<Zd>.<T>,{<Zn>.<T>},<Zm>.<T>', (('size', 2, 22), ('Zm', 5, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVTZS', 'FCVTZU', 'SCVTF', 'UCVTF'],
    '<Zd>.D,<Pg>/M,<Zn>.D', (('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['REVW', 'SXTW', 'UXTW'],
    '<Zd>.D,<Pg>/M,<Zn>.D', (('size', 2, 22), ('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVT', 'FCVTZS', 'FCVTZU'],
    '<Zd>.D,<Pg>/M,<Zn>.H', (('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVT', 'FCVTZS', 'FCVTZU', 'SCVTF', 'UCVTF'],
    '<Zd>.D,<Pg>/M,<Zn>.S', (('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MOV'],
    '<Zd>.D,<Zn>.D', (('Zm', 5, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['AND', 'BIC', 'EOR', 'ORR'],
    '<Zd>.D,<Zn>.D,<Zm>.D', (('Zm', 5, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMUL'],
    '<Zd>.D,<Zn>.D,<Zm>.D[<imm>]', (('i1', 1, 20), ('Zm', 4, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ADR'],
    '<Zd>.D,[<Zn>.D,<Zm>.D,SXTW{<amount>}]', (('Zm', 5, 16), ('msz', 2, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ADR'],
    '<Zd>.D,[<Zn>.D,<Zm>.D,UXTW{<amount>}]', (('Zm', 5, 16), ('msz', 2, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVT', 'SCVTF', 'UCVTF'],
    '<Zd>.H,<Pg>/M,<Zn>.D', (('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVTZS', 'FCVTZU', 'SCVTF', 'UCVTF'],
    '<Zd>.H,<Pg>/M,<Zn>.H', (('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVT', 'SCVTF', 'UCVTF'],
    '<Zd>.H,<Pg>/M,<Zn>.S', (('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMUL'],
    '<Zd>.H,<Zn>.H,<Zm>.H[<imm>]', (('i3h', 1, 22), ('i3l', 2, 19), ('Zm', 3, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVT', 'FCVTZS', 'FCVTZU', 'SCVTF', 'UCVTF'],
    '<Zd>.S,<Pg>/M,<Zn>.D', (('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVT', 'FCVTZS', 'FCVTZU'],
    '<Zd>.S,<Pg>/M,<Zn>.H', (('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCVTZS', 'FCVTZU', 'SCVTF', 'UCVTF'],
    '<Zd>.S,<Pg>/M,<Zn>.S', (('Pg', 3, 10), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMUL'],
    '<Zd>.S,<Zn>.S,<Zm>.S[<imm>]', (('i2', 2, 19), ('Zm', 3, 16), ('Zn', 5, 5), ('Zd', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMLA', 'FMLS', 'FNMLA', 'FNMLS', 'MLA', 'MLS'],
    '<Zda>.<T>,<Pg>/M,<Zn>.<T>,<Zm>.<T>', (('size', 2, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('Zda', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCMLA'],
    '<Zda>.<T>,<Pg>/M,<Zn>.<T>,<Zm>.<T>,<const>', (('size', 2, 22), ('Zm', 5, 16), ('rot', 2, 13), ('Pg', 3, 10), ('Zn', 5, 5), ('Zda', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SDOT', 'UDOT'],
    '<Zda>.<T>,<Zn>.<Tb>,<Zm>.<Tb>', (('size', 2, 22), ('Zm', 5, 16), ('Zn', 5, 5), ('Zda', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMLA', 'FMLS'],
    '<Zda>.D,<Zn>.D,<Zm>.D[<imm>]', (('i1', 1, 20), ('Zm', 4, 16), ('Zn', 5, 5), ('Zda', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SDOT', 'UDOT'],
    '<Zda>.D,<Zn>.H,<Zm>.H[<imm>]', (('i1', 1, 20), ('Zm', 4, 16), ('Zn', 5, 5), ('Zda', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMLA', 'FMLS'],
    '<Zda>.H,<Zn>.H,<Zm>.H[<imm>]', (('i3h', 1, 22), ('i3l', 2, 19), ('Zm', 3, 16), ('Zn', 5, 5), ('Zda', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCMLA'],
    '<Zda>.H,<Zn>.H,<Zm>.H[<imm>],<const>', (('i2', 2, 19), ('Zm', 3, 16), ('rot', 2, 10), ('Zn', 5, 5), ('Zda', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['SDOT', 'UDOT'],
    '<Zda>.S,<Zn>.B,<Zm>.B[<imm>]', (('i2', 2, 19), ('Zm', 3, 16), ('Zn', 5, 5), ('Zda', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMLA', 'FMLS'],
    '<Zda>.S,<Zn>.S,<Zm>.S[<imm>]', (('i2', 2, 19), ('Zm', 3, 16), ('Zn', 5, 5), ('Zda', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCMLA'],
    '<Zda>.S,<Zn>.S,<Zm>.S[<imm>],<const>', (('i1', 1, 20), ('Zm', 4, 16), ('rot', 2, 10), ('Zn', 5, 5), ('Zda', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DECP', 'INCP', 'SQDECP', 'SQINCP', 'UQDECP', 'UQINCP'],
    '<Zdn>.<T>,<Pg>', (('size', 2, 22), ('Pg', 4, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['CLASTA', 'CLASTB', 'SPLICE'],
    '<Zdn>.<T>,<Pg>,<Zdn>.<T>,<Zm>.<T>', (('size', 2, 22), ('Pg', 3, 10), ('Zm', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ASR', 'ASRD', 'LSL', 'LSR'],
    '<Zdn>.<T>,<Pg>/M,<Zdn>.<T>,#<const>', (('tszh', 2, 22), ('Pg', 3, 10), ('tszl', 2, 8), ('imm3', 3, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ADD', 'AND', 'ASR', 'ASRR', 'BIC', 'EOR', 'FABD', 'FADD', 'FDIV', 'FDIVR', 'FMAX', 'FMAXNM', 'FMIN', 'FMINNM', 'FMUL', 'FMULX', 'FSCALE', 'FSUB', 'FSUBR', 'LSL', 'LSLR', 'LSR', 'LSRR', 'MUL', 'ORR', 'SABD', 'SDIV', 'SDIVR', 'SMAX', 'SMIN', 'SMULH', 'SUB', 'SUBR', 'UABD', 'UDIV', 'UDIVR', 'UMAX', 'UMIN', 'UMULH'],
    '<Zdn>.<T>,<Pg>/M,<Zdn>.<T>,<Zm>.<T>', (('size', 2, 22), ('Pg', 3, 10), ('Zm', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FCADD'],
    '<Zdn>.<T>,<Pg>/M,<Zdn>.<T>,<Zm>.<T>,<const>', (('size', 2, 22), ('rot', 1, 16), ('Pg', 3, 10), ('Zm', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ASR', 'LSL', 'LSR'],
    '<Zdn>.<T>,<Pg>/M,<Zdn>.<T>,<Zm>.D', (('size', 2, 22), ('Pg', 3, 10), ('Zm', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FADD', 'FMAX', 'FMAXNM', 'FMIN', 'FMINNM', 'FMUL', 'FSUB', 'FSUBR'],
    '<Zdn>.<T>,<Pg>/M,<Zdn>.<T>,<const>', (('size', 2, 22), ('Pg', 3, 10), ('i1', 1, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FMAD', 'FMSB', 'FNMAD', 'FNMSB'],
    '<Zdn>.<T>,<Pg>/M,<Zm>.<T>,<Za>.<T>', (('size', 2, 22), ('Za', 5, 16), ('Pg', 3, 10), ('Zm', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MAD', 'MSB'],
    '<Zdn>.<T>,<Pg>/M,<Zm>.<T>,<Za>.<T>', (('size', 2, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Za', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['INSR'],
    '<Zdn>.<T>,<R><m>', (('size', 2, 22), ('Rm', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['INSR'],
    '<Zdn>.<T>,<V><m>', (('size', 2, 22), ('Vm', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['AND', 'BIC', 'EON', 'EOR', 'ORN', 'ORR'],
    '<Zdn>.<T>,<Zdn>.<T>,#<const>', (('imm13', 13, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['MUL', 'SMAX', 'SMIN', 'UMAX', 'UMIN'],
    '<Zdn>.<T>,<Zdn>.<T>,#<imm>', (('size', 2, 22), ('imm8', 8, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ADD', 'SQADD', 'SQSUB', 'SUB', 'SUBR', 'UQADD', 'UQSUB'],
    '<Zdn>.<T>,<Zdn>.<T>,#<imm>{,<shift>}', (('size', 2, 22), ('sh', 1, 13), ('imm8', 8, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['FTMAD'],
    '<Zdn>.<T>,<Zdn>.<T>,<Zm>.<T>,#<imm>', (('size', 2, 22), ('imm3', 3, 16), ('Zm', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['EXT'],
    '<Zdn>.B,<Zdn>.B,<Zm>.B,#<imm>', (('imm8h', 5, 16), ('imm8l', 3, 10), ('Zm', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DECD', 'INCD', 'SQDECD', 'SQINCD', 'UQDECD', 'UQINCD'],
    '<Zdn>.D{,<pattern>{,MUL#<imm>}}', (('imm4', 4, 16), ('pattern', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DECH', 'INCH', 'SQDECH', 'SQINCH', 'UQDECH', 'UQINCH'],
    '<Zdn>.H{,<pattern>{,MUL#<imm>}}', (('imm4', 4, 16), ('pattern', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['DECW', 'INCW', 'SQDECW', 'SQINCW', 'UQDECW', 'UQINCW'],
    '<Zdn>.S{,<pattern>{,MUL#<imm>}}', (('imm4', 4, 16), ('pattern', 5, 5), ('Zdn', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDR', 'STR'],
    '<Zt>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm9h', 6, 16), ('imm9l', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFH'],
    '<prfop>,<Pg>,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFW'],
    '<prfop>,<Pg>,[<Xn|SP>,<Xm>,LSL#2]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFD'],
    '<prfop>,<Pg>,[<Xn|SP>,<Xm>,LSL#3]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFB'],
    '<prfop>,<Pg>,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFH'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.D,<mod>#1]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFW'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.D,<mod>#2]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFD'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.D,<mod>#3]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFB'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.D,<mod>]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFH'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.D,LSL#1]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFW'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.D,LSL#2]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFD'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.D,LSL#3]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFB'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.D]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFH'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.S,<mod>#1]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFW'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.S,<mod>#2]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFD'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.S,<mod>#3]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFB'],
    '<prfop>,<Pg>,[<Xn|SP>,<Zm>.S,<mod>]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFB', 'PRFD', 'PRFH', 'PRFW'],
    '<prfop>,<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm6', 6, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFB', 'PRFD', 'PRFH', 'PRFW'],
    '<prfop>,<Pg>,[<Zn>.D{,#<imm>}]', (('imm5', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['PRFB', 'PRFD', 'PRFH', 'PRFW'],
    '<prfop>,<Pg>,[<Zn>.S{,#<imm>}]', (('imm5', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('prfop', 4, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST4B'],
    '{<Zt1>.B,<Zt2>.B,<Zt3>.B,<Zt4>.B},<Pg>,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST4B'],
    '{<Zt1>.B,<Zt2>.B,<Zt3>.B,<Zt4>.B},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD4B'],
    '{<Zt1>.B,<Zt2>.B,<Zt3>.B,<Zt4>.B},<Pg>/Z,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD4B'],
    '{<Zt1>.B,<Zt2>.B,<Zt3>.B,<Zt4>.B},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST3B'],
    '{<Zt1>.B,<Zt2>.B,<Zt3>.B},<Pg>,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST3B'],
    '{<Zt1>.B,<Zt2>.B,<Zt3>.B},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD3B'],
    '{<Zt1>.B,<Zt2>.B,<Zt3>.B},<Pg>/Z,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD3B'],
    '{<Zt1>.B,<Zt2>.B,<Zt3>.B},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST2B'],
    '{<Zt1>.B,<Zt2>.B},<Pg>,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST2B'],
    '{<Zt1>.B,<Zt2>.B},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD2B'],
    '{<Zt1>.B,<Zt2>.B},<Pg>/Z,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD2B'],
    '{<Zt1>.B,<Zt2>.B},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST4D'],
    '{<Zt1>.D,<Zt2>.D,<Zt3>.D,<Zt4>.D},<Pg>,[<Xn|SP>,<Xm>,LSL#3]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST4D'],
    '{<Zt1>.D,<Zt2>.D,<Zt3>.D,<Zt4>.D},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD4D'],
    '{<Zt1>.D,<Zt2>.D,<Zt3>.D,<Zt4>.D},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#3]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD4D'],
    '{<Zt1>.D,<Zt2>.D,<Zt3>.D,<Zt4>.D},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST3D'],
    '{<Zt1>.D,<Zt2>.D,<Zt3>.D},<Pg>,[<Xn|SP>,<Xm>,LSL#3]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST3D'],
    '{<Zt1>.D,<Zt2>.D,<Zt3>.D},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD3D'],
    '{<Zt1>.D,<Zt2>.D,<Zt3>.D},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#3]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD3D'],
    '{<Zt1>.D,<Zt2>.D,<Zt3>.D},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST2D'],
    '{<Zt1>.D,<Zt2>.D},<Pg>,[<Xn|SP>,<Xm>,LSL#3]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST2D'],
    '{<Zt1>.D,<Zt2>.D},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD2D'],
    '{<Zt1>.D,<Zt2>.D},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#3]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD2D'],
    '{<Zt1>.D,<Zt2>.D},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST4H'],
    '{<Zt1>.H,<Zt2>.H,<Zt3>.H,<Zt4>.H},<Pg>,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST4H'],
    '{<Zt1>.H,<Zt2>.H,<Zt3>.H,<Zt4>.H},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD4H'],
    '{<Zt1>.H,<Zt2>.H,<Zt3>.H,<Zt4>.H},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD4H'],
    '{<Zt1>.H,<Zt2>.H,<Zt3>.H,<Zt4>.H},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST3H'],
    '{<Zt1>.H,<Zt2>.H,<Zt3>.H},<Pg>,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST3H'],
    '{<Zt1>.H,<Zt2>.H,<Zt3>.H},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD3H'],
    '{<Zt1>.H,<Zt2>.H,<Zt3>.H},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD3H'],
    '{<Zt1>.H,<Zt2>.H,<Zt3>.H},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST2H'],
    '{<Zt1>.H,<Zt2>.H},<Pg>,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST2H'],
    '{<Zt1>.H,<Zt2>.H},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD2H'],
    '{<Zt1>.H,<Zt2>.H},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD2H'],
    '{<Zt1>.H,<Zt2>.H},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST4W'],
    '{<Zt1>.S,<Zt2>.S,<Zt3>.S,<Zt4>.S},<Pg>,[<Xn|SP>,<Xm>,LSL#2]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST4W'],
    '{<Zt1>.S,<Zt2>.S,<Zt3>.S,<Zt4>.S},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD4W'],
    '{<Zt1>.S,<Zt2>.S,<Zt3>.S,<Zt4>.S},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#2]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD4W'],
    '{<Zt1>.S,<Zt2>.S,<Zt3>.S,<Zt4>.S},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST3W'],
    '{<Zt1>.S,<Zt2>.S,<Zt3>.S},<Pg>,[<Xn|SP>,<Xm>,LSL#2]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST3W'],
    '{<Zt1>.S,<Zt2>.S,<Zt3>.S},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD3W'],
    '{<Zt1>.S,<Zt2>.S,<Zt3>.S},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#2]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD3W'],
    '{<Zt1>.S,<Zt2>.S,<Zt3>.S},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST2W'],
    '{<Zt1>.S,<Zt2>.S},<Pg>,[<Xn|SP>,<Xm>,LSL#2]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST2W'],
    '{<Zt1>.S,<Zt2>.S},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD2W'],
    '{<Zt1>.S,<Zt2>.S},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#2]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD2W'],
    '{<Zt1>.S,<Zt2>.S},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1H'],
    '{<Zt>.<T>},<Pg>,[<Xn|SP>,<Xm>,LSL#1]', (('size', 2, 21), ('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1W'],
    '{<Zt>.<T>},<Pg>,[<Xn|SP>,<Xm>,LSL#2]', (('size', 2, 21), ('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1B'],
    '{<Zt>.<T>},<Pg>,[<Xn|SP>,<Xm>]', (('size', 2, 21), ('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1B', 'ST1H', 'ST1W'],
    '{<Zt>.<T>},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('size', 2, 21), ('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['STNT1B'],
    '{<Zt>.B},<Pg>,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['STNT1B'],
    '{<Zt>.B},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1RQB', 'LDNT1B'],
    '{<Zt>.B},<Pg>/Z,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LDNF1B', 'LDNT1B'],
    '{<Zt>.B},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1RQB'],
    '{<Zt>.B},<Pg>/Z,[<Xn|SP>{,#<imm>}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1RB'],
    '{<Zt>.B},<Pg>/Z,[<Xn|SP>{,#<imm>}]', (('imm6', 6, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDFF1B'],
    '{<Zt>.B},<Pg>/Z,[<Xn|SP>{,<Xm>}]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1D', 'STNT1D'],
    '{<Zt>.D},<Pg>,[<Xn|SP>,<Xm>,LSL#3]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1H'],
    '{<Zt>.D},<Pg>,[<Xn|SP>,<Zm>.D,<mod>#1]', (('Zm', 5, 16), ('xs', 1, 14), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1W'],
    '{<Zt>.D},<Pg>,[<Xn|SP>,<Zm>.D,<mod>#2]', (('Zm', 5, 16), ('xs', 1, 14), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1D'],
    '{<Zt>.D},<Pg>,[<Xn|SP>,<Zm>.D,<mod>#3]', (('Zm', 5, 16), ('xs', 1, 14), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1B', 'ST1D', 'ST1H', 'ST1W'],
    '{<Zt>.D},<Pg>,[<Xn|SP>,<Zm>.D,<mod>]', (('Zm', 5, 16), ('xs', 1, 14), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1H'],
    '{<Zt>.D},<Pg>,[<Xn|SP>,<Zm>.D,LSL#1]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1W'],
    '{<Zt>.D},<Pg>,[<Xn|SP>,<Zm>.D,LSL#2]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1D'],
    '{<Zt>.D},<Pg>,[<Xn|SP>,<Zm>.D,LSL#3]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1B', 'ST1D', 'ST1H', 'ST1W'],
    '{<Zt>.D},<Pg>,[<Xn|SP>,<Zm>.D]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['STNT1D'],
    '{<Zt>.D},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1D'],
    '{<Zt>.D},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('size', 2, 21), ('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1B', 'ST1D', 'ST1H', 'ST1W'],
    '{<Zt>.D},<Pg>,[<Zn>.D{,#<imm>}]', (('imm5', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1H', 'LD1SH'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1SW', 'LD1W'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#2]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1D', 'LD1RQD', 'LDNT1D'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#3]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1SB'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1H', 'LD1SH', 'LDFF1H', 'LDFF1SH'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Zm>.D,<mod>#1]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1SW', 'LD1W', 'LDFF1SW', 'LDFF1W'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Zm>.D,<mod>#2]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1D', 'LDFF1D'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Zm>.D,<mod>#3]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1D', 'LD1H', 'LD1SB', 'LD1SH', 'LD1SW', 'LD1W', 'LDFF1B', 'LDFF1D', 'LDFF1H', 'LDFF1SB', 'LDFF1SH', 'LDFF1SW', 'LDFF1W'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Zm>.D,<mod>]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1H', 'LD1SH', 'LDFF1H', 'LDFF1SH'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Zm>.D,LSL#1]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1SW', 'LD1W', 'LDFF1SW', 'LDFF1W'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Zm>.D,LSL#2]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1D', 'LDFF1D'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Zm>.D,LSL#3]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1D', 'LD1H', 'LD1SB', 'LD1SH', 'LD1SW', 'LD1W', 'LDFF1B', 'LDFF1D', 'LDFF1H', 'LDFF1SB', 'LDFF1SH', 'LDFF1SW', 'LDFF1W'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>,<Zm>.D]', (('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1D', 'LD1H', 'LD1SB', 'LD1SH', 'LD1SW', 'LD1W', 'LDNF1B', 'LDNF1D', 'LDNF1H', 'LDNF1SB', 'LDNF1SH', 'LDNF1SW', 'LDNF1W', 'LDNT1D'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1RQD'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>{,#<imm>}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1RB', 'LD1RD', 'LD1RH', 'LD1RSB', 'LD1RSH', 'LD1RSW', 'LD1RW'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>{,#<imm>}]', (('imm6', 6, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDFF1H', 'LDFF1SH'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>{,<Xm>,LSL#1}]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDFF1SW', 'LDFF1W'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>{,<Xm>,LSL#2}]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDFF1D'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>{,<Xm>,LSL#3}]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDFF1B', 'LDFF1SB'],
    '{<Zt>.D},<Pg>/Z,[<Xn|SP>{,<Xm>}]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1D', 'LD1H', 'LD1SB', 'LD1SH', 'LD1SW', 'LD1W', 'LDFF1B', 'LDFF1D', 'LDFF1H', 'LDFF1SB', 'LDFF1SH', 'LDFF1SW', 'LDFF1W'],
    '{<Zt>.D},<Pg>/Z,[<Zn>.D{,#<imm>}]', (('imm5', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['STNT1H'],
    '{<Zt>.H},<Pg>,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['STNT1H'],
    '{<Zt>.H},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1H', 'LD1RQH', 'LDNT1H'],
    '{<Zt>.H},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1SB'],
    '{<Zt>.H},<Pg>/Z,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1H', 'LD1SB', 'LDNF1B', 'LDNF1H', 'LDNF1SB', 'LDNT1H'],
    '{<Zt>.H},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1RQH'],
    '{<Zt>.H},<Pg>/Z,[<Xn|SP>{,#<imm>}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1RB', 'LD1RH', 'LD1RSB'],
    '{<Zt>.H},<Pg>/Z,[<Xn|SP>{,#<imm>}]', (('imm6', 6, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDFF1H'],
    '{<Zt>.H},<Pg>/Z,[<Xn|SP>{,<Xm>,LSL#1}]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDFF1B', 'LDFF1SB'],
    '{<Zt>.H},<Pg>/Z,[<Xn|SP>{,<Xm>}]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['STNT1W'],
    '{<Zt>.S},<Pg>,[<Xn|SP>,<Xm>,LSL#2]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1H'],
    '{<Zt>.S},<Pg>,[<Xn|SP>,<Zm>.S,<mod>#1]', (('Zm', 5, 16), ('xs', 1, 14), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1W'],
    '{<Zt>.S},<Pg>,[<Xn|SP>,<Zm>.S,<mod>#2]', (('Zm', 5, 16), ('xs', 1, 14), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1B', 'ST1H', 'ST1W'],
    '{<Zt>.S},<Pg>,[<Xn|SP>,<Zm>.S,<mod>]', (('Zm', 5, 16), ('xs', 1, 14), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['STNT1W'],
    '{<Zt>.S},<Pg>,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['ST1B', 'ST1H', 'ST1W'],
    '{<Zt>.S},<Pg>,[<Zn>.S{,#<imm>}]', (('imm5', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1H', 'LD1SH'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#1]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1RQW', 'LD1W', 'LDNT1W'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>,<Xm>,LSL#2]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1SB'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>,<Xm>]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1H', 'LD1SH', 'LDFF1H', 'LDFF1SH'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>,<Zm>.S,<mod>#1]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1W', 'LDFF1W'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>,<Zm>.S,<mod>#2]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1H', 'LD1SB', 'LD1SH', 'LD1W', 'LDFF1B', 'LDFF1H', 'LDFF1SB', 'LDFF1SH', 'LDFF1W'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>,<Zm>.S,<mod>]', (('xs', 1, 22), ('Zm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1H', 'LD1SB', 'LD1SH', 'LD1W', 'LDNF1B', 'LDNF1H', 'LDNF1SB', 'LDNF1SH', 'LDNF1W', 'LDNT1W'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>{,#<imm>,MULVL}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1RQW'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>{,#<imm>}]', (('imm4', 4, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1RB', 'LD1RH', 'LD1RSB', 'LD1RSH', 'LD1RW'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>{,#<imm>}]', (('imm6', 6, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDFF1H', 'LDFF1SH'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>{,<Xm>,LSL#1}]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDFF1W'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>{,<Xm>,LSL#2}]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LDFF1B', 'LDFF1SB'],
    '{<Zt>.S},<Pg>/Z,[<Xn|SP>{,<Xm>}]', (('Rm', 5, 16), ('Pg', 3, 10), ('Rn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)

tlentry(['LD1B', 'LD1H', 'LD1SB', 'LD1SH', 'LD1W', 'LDFF1B', 'LDFF1H', 'LDFF1SB', 'LDFF1SH', 'LDFF1W'],
    '{<Zt>.S},<Pg>/Z,[<Zn>.S{,#<imm>}]', (('imm5', 5, 16), ('Pg', 3, 10), ('Zn', 5, 5), ('Zt', 5, 0)),
    matcher   = 'Unimp',
    processor = 'Unimp',
)
