
tlentry(['ERETAA', 'ERETAB', 'RETAA', 'RETAB'],
    '', (),
    matcher   = '',
    processor = '',
)

tlentry(['UDF'],
    '#<imm>', (('imm16', 16, 0),),
    matcher   = 'Imm',
    processor = 'Ubits(0, 16)',
)

tlentry(['PRFM'],
    '(<prfop>|#<imm5>),<label>', (('imm19', 19, 5), ('Rt', 5, 0)),
    matcher   = 'Imm, Offset',
    processor = 'Ubits(0, 5), Offset(BCOND)',
)

tlentry(['PRFM'],
    '(<prfop>|#<imm5>),[<Xn|SP>,(<Wm>|<Xm>){,<extend>{<amount>}}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'Imm, RefIndex',
    processor = 'Ubits(0, 5), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 3])',
)

tlentry(['PRFUM'],
    '(<prfop>|#<imm5>),[<Xn|SP>{,#<simm>}]', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'Imm, RefOffset',
    processor = 'Ubits(0, 5), R(5), Sbits(12, 9)',
)

tlentry(['B'],
    '.<cond><label>', (('imm19', 19, 5), ('cond', 4, 0)),
    matcher   = 'Dot, Cond, Offset',
    processor = 'Cond(0), Offset(BCOND)',
)

tlentry(['TBNZ', 'TBZ'],
    '<R><t>,#<imm>,<label>', (('b5', 1, 31), ('b40', 5, 19), ('imm14', 14, 5), ('Rt', 5, 0)),
    matcher   = 'W, Imm, Offset',
    processor = 'R(0), Ubits(19, 5), Offset(TBZ)',
    matchers  =['X, Imm, Offset'], # immediate gets encoded into b5:b40
    processors=['R(0), BUbits(6), Uslice(19, 5, 0), Uslice(31, 1, 5), A, Offset(TBZ)'],
)

tlentry(['MOV'],
    '<Wd>,#<imm>', (('hw', 2, 21), ('imm16', 16, 5), ('Rd', 5, 0)),
    matcher   = 'W, Imm',
    processor = 'R(0), Special(5, WIDE_IMMEDIATE_W)', # wide immediate
    matchers  =['Dot, Lit("inverted"), W, Imm'],
    processors=['R(0), Special(5, INVERTED_WIDE_IMMEDIATE_W)'], # inverted wide immediate
    names = ["MOV (wide immediate)", "MOV (inverted wide immediate)"]
)

tlentry(['MOVK', 'MOVN', 'MOVZ'],
    '<Wd>,#<imm>{,LSL#<shift>}', (('hw', 2, 21), ('imm16', 16, 5), ('Rd', 5, 0)),
    matcher   = 'W, Imm, End, LitMod(LSL)',
    processor = 'R(0), Ubits(5, 16), Ulist(21, &[0, 16])',
)

tlentry(['BFC'],
    '<Wd>,#<lsb>,#<width>', (('immr', 6, 16), ('imms', 6, 10), ('Rd', 5, 0)),
    matcher   = 'W, Imm, Imm',
    processor = 'R(0), Unegmod(16, 5), BUsum(5), Urange(10, 1, 32)', # immr = -lsb % 32, imms = width - 1
)

tlentry(['MOV', 'NGC', 'NGCS'],
    '<Wd>,<Wm>', (('Rm', 5, 16), ('Rd', 5, 0)),
    matcher   = 'W, W',
    processor = 'R(0), R(16)',
    priority = 1
)

tlentry(['MVN', 'NEG', 'NEGS'],
    '<Wd>,<Wm>{,<shift>#<amount>}', (('shift', 2, 22), ('Rm', 5, 16), ('imm6', 6, 10), ('Rd', 5, 0)),
    matcher   = 'W, W, End, Mod(SHIFTS)',
    processor = 'R(0), R(16), Rotates(22), Ubits(10, 5)',
    priority = 1
)

tlentry(['CLS', 'CLZ', 'RBIT', 'REV', 'REV16', 'SXTB', 'SXTH', 'UXTB', 'UXTH'],
    '<Wd>,<Wn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W',
    processor = 'R(0), R(5)',
)

tlentry(['ANDS'],
    '<Wd>,<Wn>,#<imm>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, Imm',
    processor = 'R(0), R(5), Special(10, LOGICAL_IMMEDIATE_W)',
)

tlentry(['BFM', 'SBFM', 'UBFM'],
    '<Wd>,<Wn>,#<immr>,#<imms>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, Imm, Imm',
    processor = 'R(0), R(5), Ubits(16, 5), Ubits(10, 5)',
)

tlentry(['BFI', 'SBFIZ', 'UBFIZ'],
    '<Wd>,<Wn>,#<lsb>,#<width>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, Imm, Imm',
    processor = 'R(0), R(5), Unegmod(16, 5), BUsum(5), Urange(10, 1, 32)', # immr = -lsb % 32, imms = width - 1
)

tlentry(['BFXIL', 'SBFX', 'UBFX'],
    '<Wd>,<Wn>,#<lsb>,#<width>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, Imm, Imm',
    processor = 'R(0), R(5), Ubits(16, 5), BUsum(5), Usumdec(10, 5)', # immr = lsb, imms = lsb + width - 1 
)

tlentry(['ASR', 'LSR'],
    '<Wd>,<Wn>,#<shift>', (('immr', 6, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, Imm',
    processor = 'R(0), R(5), Ubits(16, 5)',
)

tlentry(['LSL'],
    '<Wd>,<Wn>,#<shift>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, Imm',
    processor = 'R(0), R(5), Unegmod(16, 5), C, Usub(10, 5, 31)', # immr = -shift % 32, imms = 31 - shift
)

tlentry(['ADC', 'ADCS', 'ASR', 'ASRV', 'CRC32B', 'CRC32CB', 'CRC32CH', 'CRC32CW', 'CRC32H', 'CRC32W', 'LSL', 'LSLV', 'LSR', 'LSRV', 'MNEG', 'MUL', 'ROR', 'RORV', 'SBC', 'SBCS', 'SDIV', 'UDIV'],
    '<Wd>,<Wn>,<Wm>', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, W',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['EXTR'],
    '<Wd>,<Wn>,<Wm>,#<lsb>', (('Rm', 5, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, W, Imm',
    processor = 'R(0), R(5), R(16), Ubits(10, 5)',
)

tlentry(['MADD', 'MSUB'],
    '<Wd>,<Wn>,<Wm>,<Wa>', (('Rm', 5, 16), ('Ra', 5, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, W, W',
    processor = 'R(0), R(5), R(16), R(10)',
)

tlentry(['CSEL', 'CSINC', 'CSINV', 'CSNEG'],
    '<Wd>,<Wn>,<Wm>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, W, Cond',
    processor = 'R(0), R(5), R(16), Cond(12)',
)

tlentry(['ADD', 'ADDS', 'SUB', 'SUBS'],
    '<Wd>,<Wn>,<Wm>{,<shift>#<amount>}', (('shift', 2, 22), ('Rm', 5, 16), ('imm6', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, W, End, Mod(SHIFTS)',
    processor = 'R(0), R(5), R(16), Rotates(22), Ubits(10, 5)',
    priority = 1
)

tlentry(['AND', 'ANDS', 'BIC', 'BICS', 'EON', 'EOR', 'ORN', 'ORR'],
    '<Wd>,<Wn>,<Wm>{,<shift>#<amount>}', (('shift', 2, 22), ('Rm', 5, 16), ('imm6', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, W, End, Mod(ROTATES)',
    processor = 'R(0), R(5), R(16), Rotates(22), Ubits(10, 5)',
)

tlentry(['CRC32CX', 'CRC32X'],
    '<Wd>,<Wn>,<Xm>', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, X',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['CINC', 'CINV', 'CNEG'],
    '<Wd>,<Wn>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, Cond',
    processor = 'R(0), R(5), C, R(16), CondInv(12)',
)

tlentry(['ADDS', 'SUBS'],
    '<Wd>,<Wn|WSP>,#<imm>{,<shift>}', (('sh', 1, 22), ('imm12', 12, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, WSP, Imm, End, LitMod(LSL)',
    processor = 'R(0), R(5), Ubits(10, 12), Ulist(22, &[0, 12])',
)

tlentry(['ADDS', 'SUBS'],
    '<Wd>,<Wn|WSP>,<Wm>{,<extend>{#<amount>}}', (('Rm', 5, 16), ('option', 3, 13), ('imm3', 3, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, WSP, W, End, Mod(EXTENDS)',
    processor = 'R(0), R(5), R(16), ExtendsW(13), Urange(10, 0, 4)',
)

tlentry(['ROR'],
    '<Wd>,<Ws>,#<shift>', (('Rm', 5, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'W, W, Imm',
    processor = 'R(0), R(5), C, R(16), Ubits(10, 5)',
)

tlentry(['CSET', 'CSETM'],
    '<Wd>,<cond>', (('cond', 4, 12), ('Rd', 5, 0)),
    matcher   = 'W, Cond',
    processor = 'R(0), CondInv(12)',
)

tlentry(['MOV'],
    '<Wd|WSP>,#<imm>', (('immr', 6, 16), ('imms', 6, 10), ('Rd', 5, 0)),
    matcher   = 'Dot, Lit("logical"), WSP, Imm',
    processor = 'R(0), Special(10, LOGICAL_IMMEDIATE_W)',
)

tlentry(['AND', 'EOR', 'ORR'],
    '<Wd|WSP>,<Wn>,#<imm>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'WSP, W, Imm',
    processor = 'R(0), R(5), Special(10, LOGICAL_IMMEDIATE_W)',
)

tlentry(['MOV'],
    '<Wd|WSP>,<Wn|WSP>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'WSP, WSP',
    processor = 'R(0), R(5)',
)

tlentry(['ADD', 'SUB'],
    '<Wd|WSP>,<Wn|WSP>,#<imm>{,<shift>}', (('sh', 1, 22), ('imm12', 12, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'WSP, WSP, Imm, End, LitMod(LSL)',
    processor = 'R(0), R(5), Ubits(10, 12), Ulist(22, &[0, 12])',
)

tlentry(['ADD', 'SUB'],
    '<Wd|WSP>,<Wn|WSP>,<Wm>{,<extend>{#<amount>}}', (('Rm', 5, 16), ('option', 3, 13), ('imm3', 3, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'WSP, WSP, W, End, Mod(EXTENDS)',
    processor = 'R(0), R(5), R(16), ExtendsW(13), Urange(10, 0, 4)',
)

tlentry(['SETF16', 'SETF8'],
    '<Wn>', (('Rn', 5, 5),),
    matcher   = 'W',
    processor = 'R(5)',
)

tlentry(['TST'],
    '<Wn>,#<imm>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5)),
    matcher   = 'W, Imm',
    processor = 'R(5), Special(10, LOGICAL_IMMEDIATE_W)',
)

tlentry(['CCMN', 'CCMP'],
    '<Wn>,#<imm>,#<nzcv>,<cond>', (('imm5', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('nzcv', 4, 0)),
    matcher   = 'W, Imm, Imm, Cond',
    processor = 'R(5), Ubits(16, 5), Ubits(0, 4), Cond(12)',
)

tlentry(['CCMN', 'CCMP'],
    '<Wn>,<Wm>,#<nzcv>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('nzcv', 4, 0)),
    matcher   = 'W, W, Imm, Cond',
    processor = 'R(5), R(16), Ubits(0, 4), Cond(12)',
)

tlentry(['CMN', 'CMP'],
    '<Wn>,<Wm>{,<shift>#<amount>}', (('shift', 2, 22), ('Rm', 5, 16), ('imm6', 6, 10), ('Rn', 5, 5)),
    matcher   = 'W, W, End, Mod(SHIFTS)',
    processor = 'R(5), R(16), Rotates(22), Ubits(10, 5)',
    priority = 1
)

tlentry(['TST'],
    '<Wn>,<Wm>{,<shift>#<amount>}', (('shift', 2, 22), ('Rm', 5, 16), ('imm6', 6, 10), ('Rn', 5, 5)),
    matcher   = 'W, W, End, Mod(ROTATES)',
    processor = 'R(5), R(16), Rotates(22), Ubits(10, 5)',
)

tlentry(['CMN', 'CMP'],
    '<Wn|WSP>,#<imm>{,<shift>}', (('sh', 1, 22), ('imm12', 12, 10), ('Rn', 5, 5)),
    matcher   = 'WSP, Imm, End, LitMod(LSL)',
    processor = 'R(5), Ubits(10, 12), Ulist(22, &[0, 12])',
)

tlentry(['CMN', 'CMP'],
    '<Wn|WSP>,<Wm>{,<extend>{#<amount>}}', (('Rm', 5, 16), ('option', 3, 13), ('imm3', 3, 10), ('Rn', 5, 5)),
    matcher   = 'WSP, W, End, Mod(EXTENDS)',
    processor = 'R(5), R(16), ExtendsW(13), Urange(10, 0, 4)',
)

tlentry(['CASP', 'CASPA', 'CASPAL', 'CASPL'],
    '<Ws>,<W(s+1)>,<Wt>,<W(t+1)>,[<Xn|SP>{,#0}]', (('Rs', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, W, W, W, RefBase',
    processor = 'REven(16), RNext, REven(0), RNext, R(5)',
)

tlentry(['STLXP', 'STXP'],
    '<Ws>,<Wt1>,<Wt2>,[<Xn|SP>{,#0}]', (('Rs', 5, 16), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, W, W, RefBase',
    processor = 'R(16), R(0), R(10), R(5)',
)

tlentry(['LDADD', 'LDADDA', 'LDADDAB', 'LDADDAH', 'LDADDAL', 'LDADDALB', 'LDADDALH', 'LDADDB', 'LDADDH', 'LDADDL', 'LDADDLB', 'LDADDLH', 'LDCLR', 'LDCLRA', 'LDCLRAB', 'LDCLRAH', 'LDCLRAL', 'LDCLRALB', 'LDCLRALH', 'LDCLRB', 'LDCLRH', 'LDCLRL', 'LDCLRLB', 'LDCLRLH', 'LDEOR', 'LDEORA', 'LDEORAB', 'LDEORAH', 'LDEORAL', 'LDEORALB', 'LDEORALH', 'LDEORB', 'LDEORH', 'LDEORL', 'LDEORLB', 'LDEORLH', 'LDSET', 'LDSETA', 'LDSETAB', 'LDSETAH', 'LDSETAL', 'LDSETALB', 'LDSETALH', 'LDSETB', 'LDSETH', 'LDSETL', 'LDSETLB', 'LDSETLH', 'LDSMAX', 'LDSMAXA', 'LDSMAXAB', 'LDSMAXAH', 'LDSMAXAL', 'LDSMAXALB', 'LDSMAXALH', 'LDSMAXB', 'LDSMAXH', 'LDSMAXL', 'LDSMAXLB', 'LDSMAXLH', 'LDSMIN', 'LDSMINA', 'LDSMINAB', 'LDSMINAH', 'LDSMINAL', 'LDSMINALB', 'LDSMINALH', 'LDSMINB', 'LDSMINH', 'LDSMINL', 'LDSMINLB', 'LDSMINLH', 'LDUMAX', 'LDUMAXA', 'LDUMAXAB', 'LDUMAXAH', 'LDUMAXAL', 'LDUMAXALB', 'LDUMAXALH', 'LDUMAXB', 'LDUMAXH', 'LDUMAXL', 'LDUMAXLB', 'LDUMAXLH', 'LDUMIN', 'LDUMINA', 'LDUMINAB', 'LDUMINAH', 'LDUMINAL', 'LDUMINALB', 'LDUMINALH', 'LDUMINB', 'LDUMINH', 'LDUMINL', 'LDUMINLB', 'LDUMINLH', 'SWP', 'SWPA', 'SWPAB', 'SWPAH', 'SWPAL', 'SWPALB', 'SWPALH', 'SWPB', 'SWPH', 'SWPL', 'SWPLB', 'SWPLH'],
    '<Ws>,<Wt>,[<Xn|SP>]', (('Rs', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, W, RefBase',
    processor = 'R(16), R(0), R(5)',
)

tlentry(['CAS', 'CASA', 'CASAB', 'CASAH', 'CASAL', 'CASALB', 'CASALH', 'CASB', 'CASH', 'CASL', 'CASLB', 'CASLH', 'STLXR', 'STLXRB', 'STLXRH', 'STXR', 'STXRB', 'STXRH'],
    '<Ws>,<Wt>,[<Xn|SP>{,#0}]', (('Rs', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, W, RefBase',
    processor = 'R(16), R(0), R(5)',
)

tlentry(['STLXP', 'STXP'],
    '<Ws>,<Xt1>,<Xt2>,[<Xn|SP>{,#0}]', (('Rs', 5, 16), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, X, X, RefBase',
    processor = 'R(16), R(0), R(10), R(5)',
)

tlentry(['STLXR', 'STXR'],
    '<Ws>,<Xt>,[<Xn|SP>{,#0}]', (('Rs', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, X, RefBase',
    processor = 'R(16), R(0), R(5)',
)

tlentry(['STADD', 'STADDB', 'STADDH', 'STADDL', 'STADDLB', 'STADDLH', 'STCLR', 'STCLRB', 'STCLRH', 'STCLRL', 'STCLRLB', 'STCLRLH', 'STEOR', 'STEORB', 'STEORH', 'STEORL', 'STEORLB', 'STEORLH', 'STSET', 'STSETB', 'STSETH', 'STSETL', 'STSETLB', 'STSETLH', 'STSMAX', 'STSMAXB', 'STSMAXH', 'STSMAXL', 'STSMAXLB', 'STSMAXLH', 'STSMIN', 'STSMINB', 'STSMINH', 'STSMINL', 'STSMINLB', 'STSMINLH', 'STUMAX', 'STUMAXB', 'STUMAXH', 'STUMAXL', 'STUMAXLB', 'STUMAXLH', 'STUMIN', 'STUMINB', 'STUMINH', 'STUMINL', 'STUMINLB', 'STUMINLH'],
    '<Ws>,[<Xn|SP>]', (('Rs', 5, 16), ('Rn', 5, 5)),
    matcher   = 'W, RefBase',
    processor = 'R(16), R(5)',
)

tlentry(['LDP', 'STP'],
    '<Wt1>,<Wt2>,[<Xn|SP>,#<imm>]!', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, W, RefPre',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 2)',
)

tlentry(['LDP', 'STP'],
    '<Wt1>,<Wt2>,[<Xn|SP>],#<imm>', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, W, RefBase, Imm',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 2)',
)

tlentry(['LDAXP', 'LDXP'],
    '<Wt1>,<Wt2>,[<Xn|SP>{,#0}]', (('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, W, RefBase',
    processor = 'R(0), R(10), R(5)',
)

tlentry(['LDNP', 'LDP', 'STNP', 'STP'],
    '<Wt1>,<Wt2>,[<Xn|SP>{,#<imm>}]', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, W, RefOffset',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 2)',
)

tlentry(['CBNZ', 'CBZ', 'LDR'],
    '<Wt>,<label>', (('imm19', 19, 5), ('Rt', 5, 0)),
    matcher   = 'W, Offset',
    processor = 'R(0), Offset(BCOND)',
)

tlentry(['LDR', 'STR'],
    '<Wt>,[<Xn|SP>,#<simm>]!', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefPre',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDRH', 'LDRSH', 'STRH'],
    '<Wt>,[<Xn|SP>,#<simm>]!', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefPre',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDRB', 'LDRSB', 'STRB'],
    '<Wt>,[<Xn|SP>,#<simm>]!', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefPre',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDRB', 'LDRSB', 'STRB'],
    '<Wt>,[<Xn|SP>,(<Wm>|<Xm>),<extend>{<amount>}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 0])',
)

tlentry(['LDR', 'STR'],
    '<Wt>,[<Xn|SP>,(<Wm>|<Xm>){,<extend>{<amount>}}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 2])',
)

tlentry(['LDRH', 'LDRSH', 'STRH'],
    '<Wt>,[<Xn|SP>,(<Wm>|<Xm>){,<extend>{<amount>}}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 1])',
)

tlentry(['LDRB', 'LDRSB', 'STRB'],
    '<Wt>,[<Xn|SP>,<Xm>{,LSL<amount>}]', (('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    forget = True
)

tlentry(['LDR', 'LDRH', 'LDRB', 'LDRSB', 'LDRSH', 'LDRSW', 'STR', 'STRH', 'STRB'],
    '<Wt>,[<Xn|SP>],#<simm>', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefBase, Imm',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDAPR', 'LDAPRB', 'LDAPRH', 'LDAR', 'LDARB', 'LDARH', 'LDAXR', 'LDAXRB', 'LDAXRH', 'LDLAR', 'LDLARB', 'LDLARH', 'LDXR', 'LDXRB', 'LDXRH', 'STLLR', 'STLLRB', 'STLLRH', 'STLR', 'STLRB', 'STLRH'],
    '<Wt>,[<Xn|SP>{,#0}]', (('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefBase',
    processor = 'R(0), R(5)',
)

tlentry(['LDR', 'STR'],
    '<Wt>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefOffset',
    processor = 'R(0), R(5), Uscaled(10, 12, 2)',
)

tlentry(['LDRH', 'LDRSH', 'STRH'],
    '<Wt>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefOffset',
    processor = 'R(0), R(5), Uscaled(10, 12, 1)',
)

tlentry(['LDRB', 'LDRSB', 'STRB'],
    '<Wt>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefOffset',
    processor = 'R(0), R(5), Ubits(10, 12)',
)

tlentry(['LDAPUR', 'LDAPURB', 'LDAPURH', 'LDAPURSB', 'LDAPURSH', 'LDTR', 'LDTRB', 'LDTRH', 'LDTRSB', 'LDTRSH', 'LDUR', 'LDURB', 'LDURH', 'LDURSB', 'LDURSH', 'STLUR', 'STLURB', 'STLURH', 'STTR', 'STTRB', 'STTRH', 'STUR', 'STURB', 'STURH'],
    '<Wt>,[<Xn|SP>{,#<simm>}]', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'W, RefOffset',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['AUTDZA', 'AUTDZB', 'AUTIZA', 'AUTIZB', 'PACDZA', 'PACDZB', 'PACIZA', 'PACIZB', 'XPACD', 'XPACI'],
    '<Xd>', (('Rd', 5, 0),),
    matcher   = 'X',
    processor = 'R(0)',
)

tlentry(['MOV'],
    '<Xd>,#<imm>', (('hw', 2, 21), ('imm16', 16, 5), ('Rd', 5, 0)),
    matcher   = 'X, Imm',
    processor = 'R(0), Special(5, WIDE_IMMEDIATE_X)', # wide immediate
    matchers  =['Dot, Lit("inverted"), X, Imm'],
    processors=['R(0), Special(5, INVERTED_WIDE_IMMEDIATE_X)'], # inverted wide immediate
    names = ["MOV (wide immediate)", "MOV (inverted wide immediate)"]
)

tlentry(['MOVK', 'MOVN', 'MOVZ'],
    '<Xd>,#<imm>{,LSL#<shift>}', (('hw', 2, 21), ('imm16', 16, 5), ('Rd', 5, 0)),
    matcher   = 'X, Imm, End, LitMod(LSL)',
    processor = 'R(0), Ubits(5, 16), Ulist(21, &[0, 16, 32, 48])',
)

tlentry(['BFC'],
    '<Xd>,#<lsb>,#<width>', (('immr', 6, 16), ('imms', 6, 10), ('Rd', 5, 0)),
    matcher   = 'X, Imm, Imm',
    processor = 'R(0), Unegmod(16, 6), BUsum(6), Urange(10, 1, 64)', # immr = -lsb % 64, imms = width - 1
)

tlentry(['SXTB', 'SXTH', 'SXTW'],
    '<Xd>,<Wn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, W',
    processor = 'R(0), R(5)',
)

tlentry(['SMNEGL', 'SMULL', 'UMNEGL', 'UMULL'],
    '<Xd>,<Wn>,<Wm>', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, W, W',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['SMADDL', 'SMSUBL', 'UMADDL', 'UMSUBL'],
    '<Xd>,<Wn>,<Wm>,<Xa>', (('Rm', 5, 16), ('Ra', 5, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, W, W, X',
    processor = 'R(0), R(5), R(16), R(10)',
)

tlentry(['MOV', 'NGC', 'NGCS'],
    '<Xd>,<Xm>', (('Rm', 5, 16), ('Rd', 5, 0)),
    matcher   = 'X, X',
    processor = 'R(0), R(16)',
    priority = 1
)

tlentry(['MVN', 'NEG', 'NEGS'],
    '<Xd>,<Xm>{,<shift>#<amount>}', (('shift', 2, 22), ('Rm', 5, 16), ('imm6', 6, 10), ('Rd', 5, 0)),
    matcher   = 'X, X, End, Mod(SHIFTS)',
    processor = 'R(0), R(16), Rotates(22), Ubits(10, 6)',
    priority = 1
)

tlentry(['CLS', 'CLZ', 'RBIT', 'REV', 'REV16', 'REV32', 'REV64'],
    '<Xd>,<Xn>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X',
    processor = 'R(0), R(5)',
)

tlentry(['ANDS'],
    '<Xd>,<Xn>,#<imm>', (('N', 1, 22), ('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, Imm',
    processor = 'R(0), R(5), Special(10, LOGICAL_IMMEDIATE_X)',
)

tlentry(['BFM', 'SBFM', 'UBFM'],
    '<Xd>,<Xn>,#<immr>,#<imms>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, Imm, Imm',
    processor = 'R(0), R(5), Ubits(16, 6), BUsum(6), Ubits(10, 6)',
)

tlentry(['BFI', 'SBFIZ', 'UBFIZ'],
    '<Xd>,<Xn>,#<lsb>,#<width>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, Imm, Imm',
    processor = 'R(0), R(5), Unegmod(16, 6), BUsum(6), Urange(10, 1, 64)',  # immr = -lsb % 64, imms = width - 1
)

tlentry(['BFXIL', 'SBFX', 'UBFX'],
    '<Xd>,<Xn>,#<lsb>,#<width>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, Imm, Imm',
    processor = 'R(0), R(5), Ubits(16, 6), BUsum(6), Usumdec(10, 6)',  # immr = lsb, imms = lsb + width - 1 
)

tlentry(['ASR', 'LSR'],
    '<Xd>,<Xn>,#<shift>', (('immr', 6, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, Imm',
    processor = 'R(0), R(5), Ubits(16, 6)',
)

tlentry(['LSL'],
    '<Xd>,<Xn>,#<shift>', (('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, Imm',
    processor = 'R(0), R(5), Unegmod(16, 6), C, Usub(10, 6, 63)', # immr = -shift % 64, imms = 63 - shift
)

tlentry(['ADC', 'ADCS', 'ASR', 'ASRV', 'LSL', 'LSLV', 'LSR', 'LSRV', 'MNEG', 'MUL', 'ROR', 'RORV', 'SBC', 'SBCS', 'SDIV', 'SMULH', 'UDIV', 'UMULH'],
    '<Xd>,<Xn>,<Xm>', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, X',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['EXTR'],
    '<Xd>,<Xn>,<Xm>,#<lsb>', (('Rm', 5, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, X, Imm',
    processor = 'R(0), R(5), R(16), Ubits(10, 6)',
)

tlentry(['MADD', 'MSUB'],
    '<Xd>,<Xn>,<Xm>,<Xa>', (('Rm', 5, 16), ('Ra', 5, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, X, X',
    processor = 'R(0), R(5), R(16), R(10)',
)

tlentry(['CSEL', 'CSINC', 'CSINV', 'CSNEG'],
    '<Xd>,<Xn>,<Xm>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, X, Cond',
    processor = 'R(0), R(5), R(16), Cond(12)',
)

tlentry(['ADD', 'ADDS', 'SUB', 'SUBS'],
    '<Xd>,<Xn>,<Xm>{,<shift>#<amount>}', (('shift', 2, 22), ('Rm', 5, 16), ('imm6', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, X, End, Mod(SHIFTS)',
    processor = 'R(0), R(5), R(16), Rotates(22), Ubits(10, 6)',
    priority = 1
)

tlentry(['AND', 'ANDS', 'BIC', 'BICS', 'EON', 'EOR', 'ORN', 'ORR'],
    '<Xd>,<Xn>,<Xm>{,<shift>#<amount>}', (('shift', 2, 22), ('Rm', 5, 16), ('imm6', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, X, End, Mod(ROTATES)',
    processor = 'R(0), R(5), R(16), Rotates(22), Ubits(10, 6)',
)

tlentry(['PACGA'],
    '<Xd>,<Xn>,<Xm|SP>', (('Rm', 5, 16), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, XSP',
    processor = 'R(0), R(5), R(16)',
)

tlentry(['CINC', 'CINV', 'CNEG'],
    '<Xd>,<Xn>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, Cond',
    processor = 'R(0), R(5), C, R(16), CondInv(12)',
)

tlentry(['AUTDA', 'AUTDB', 'AUTIA', 'AUTIB', 'PACDA', 'PACDB', 'PACIA', 'PACIB'],
    '<Xd>,<Xn|SP>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, XSP',
    processor = 'R(0), R(5)',
)

tlentry(['ADDS', 'SUBS'],
    '<Xd>,<Xn|SP>,#<imm>{,<shift>}', (('sh', 1, 22), ('imm12', 12, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, XSP, Imm, End, LitMod(LSL)',
    processor = 'R(0), R(5), Ubits(10, 12), Ulist(22, &[0, 12])',
)

tlentry(['ADDS', 'SUBS'],
    '<Xd>,<Xn|SP>,<R><m>{,<extend>{#<amount>}}', (('Rm', 5, 16), ('option', 3, 13), ('imm3', 3, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, XSP, W, End, Mod(EXTENDS_W)',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)',
    matchers  =['X, XSP, X, End, Mod(EXTENDS_X)'],
    processors=['R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)'],
)

tlentry(['ROR'],
    '<Xd>,<Xs>,#<shift>', (('Rm', 5, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'X, X, Imm',
    processor = 'R(0), R(5), C, R(16), Ubits(10, 6)',
)

tlentry(['CSET', 'CSETM'],
    '<Xd>,<cond>', (('cond', 4, 12), ('Rd', 5, 0)),
    matcher   = 'X, Cond',
    processor = 'R(0), CondInv(12)',
)

tlentry(['ADR'],
    '<Xd>,<label>', (('immlo', 2, 29), ('immhi', 19, 5), ('Rd', 5, 0)),
    matcher   = 'X, Offset',
    processor = 'R(0), Offset(ADR)', # offset is unscaled, encoded in hi:lo
)

tlentry(['ADRP'],
    '<Xd>,<label>', (('immlo', 2, 29), ('immhi', 19, 5), ('Rd', 5, 0)),
    matcher   = 'X, Offset',
    processor = 'R(0), Offset(ADRP)', # offset is scaled to 12 bits, encoded in hi:lo
)

tlentry(['MOV'],
    '<Xd|SP>,#<imm>', (('N', 1, 22), ('immr', 6, 16), ('imms', 6, 10), ('Rd', 5, 0)),
    matcher   = 'Dot, Lit("logical"), XSP, Imm',
    processor = 'R(0), Special(10, LOGICAL_IMMEDIATE_X)',
)

tlentry(['AND', 'EOR', 'ORR'],
    '<Xd|SP>,<Xn>,#<imm>', (('N', 1, 22), ('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'XSP, X, Imm',
    processor = 'R(0), R(5), Special(10, LOGICAL_IMMEDIATE_X)',
)

tlentry(['MOV'],
    '<Xd|SP>,<Xn|SP>', (('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'XSP, XSP',
    processor = 'R(0), R(5)',
)

tlentry(['ADD', 'SUB'],
    '<Xd|SP>,<Xn|SP>,#<imm>{,<shift>}', (('sh', 1, 22), ('imm12', 12, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'XSP, XSP, Imm, End, LitMod(LSL)',
    processor = 'R(0), R(5), Ubits(10, 12), Ulist(22, &[0, 12])',
)

tlentry(['ADD', 'SUB'],
    '<Xd|SP>,<Xn|SP>,<R><m>{,<extend>{#<amount>}}', (('Rm', 5, 16), ('option', 3, 13), ('imm3', 3, 10), ('Rn', 5, 5), ('Rd', 5, 0)),
    matcher   = 'XSP, XSP, W, End, Mod(EXTENDS_W)',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)',
    matchers  =['XSP, XSP, X, End, Mod(EXTENDS_X)'],
    processors=['R(0), R(5), R(16), ExtendsX(13), Urange(10, 0, 4)'],
)

tlentry(['BLR', 'BLRAAZ', 'BLRABZ', 'BR', 'BRAAZ', 'BRABZ'],
    '<Xn>', (('Rn', 5, 5),),
    matcher   = 'X',
    processor = 'R(5)',
)

tlentry(['TST'],
    '<Xn>,#<imm>', (('N', 1, 22), ('immr', 6, 16), ('imms', 6, 10), ('Rn', 5, 5)),
    matcher   = 'X, Imm',
    processor = 'R(5), Special(10, LOGICAL_IMMEDIATE_X)',
)

tlentry(['CCMN', 'CCMP'],
    '<Xn>,#<imm>,#<nzcv>,<cond>', (('imm5', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('nzcv', 4, 0)),
    matcher   = 'X, Imm, Imm, Cond',
    processor = 'R(5), Ubits(16, 5), Ubits(0, 4), Cond(12)',
)

tlentry(['RMIF'],
    '<Xn>,#<shift>,#<mask>', (('imm6', 6, 15), ('Rn', 5, 5), ('mask', 4, 0)),
    matcher   = 'X, Imm, Imm',
    processor = 'R(5), Ubits(15, 6), Ubits(0, 4)',
)

tlentry(['CCMN', 'CCMP'],
    '<Xn>,<Xm>,#<nzcv>,<cond>', (('Rm', 5, 16), ('cond', 4, 12), ('Rn', 5, 5), ('nzcv', 4, 0)),
    matcher   = 'X, X, Imm, Cond',
    processor = 'R(5), R(16), Ubits(0, 4), Cond(12)',
)

tlentry(['CMN', 'CMP'],
    '<Xn>,<Xm>{,<shift>#<amount>}', (('shift', 2, 22), ('Rm', 5, 16), ('imm6', 6, 10), ('Rn', 5, 5)),
    matcher   = 'X, X, End, Mod(SHIFTS)',
    processor = 'R(5), R(16), Rotates(22), Ubits(10, 6)',
    priority = 1
)

tlentry(['TST'],
    '<Xn>,<Xm>{,<shift>#<amount>}', (('shift', 2, 22), ('Rm', 5, 16), ('imm6', 6, 10), ('Rn', 5, 5)),
    matcher   = 'X, X, End, Mod(ROTATES)',
    processor = 'R(5), R(16), Rotates(22), Ubits(10, 6)',
)

tlentry(['BLRAA', 'BLRAB', 'BRAA', 'BRAB'],
    '<Xn>,<Xm|SP>', (('Rn', 5, 5), ('Rm', 5, 0)),
    matcher   = 'X, XSP',
    processor = 'R(5), R(0)',
)

tlentry(['CMN', 'CMP'],
    '<Xn|SP>,#<imm>{,<shift>}', (('sh', 1, 22), ('imm12', 12, 10), ('Rn', 5, 5)),
    matcher   = 'XSP, Imm, End, LitMod(LSL)',
    processor = 'R(5), Ubits(10, 12), Ulist(22, &[0, 12])',
)

tlentry(['CMN', 'CMP'],
    '<Xn|SP>,<R><m>{,<extend>{#<amount>}}', (('Rm', 5, 16), ('option', 3, 13), ('imm3', 3, 10), ('Rn', 5, 5)),
    matcher   = 'XSP, W, Mod(EXTENDS_W)',
    processor = 'R(5), R(16), ExtendsX(13), Urange(10, 0, 4)',
    matchers  =['XSP, X, End, Mod(EXTENDS_X)'],
    processors=['R(5), R(16), ExtendsX(13), Urange(10, 0, 4)'],
)

tlentry(['CASP', 'CASPA', 'CASPAL', 'CASPL'],
    '<Xs>,<X(s+1)>,<Xt>,<X(t+1)>,[<Xn|SP>{,#0}]', (('Rs', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, X, X, X, RefBase',
    processor = 'REven(16), RNext, REven(0), RNext, R(5)', # register-next-to-register constraint
)

tlentry(['LDADD', 'LDADDA', 'LDADDAL', 'LDADDL', 'LDCLR', 'LDCLRA', 'LDCLRAL', 'LDCLRL', 'LDEOR', 'LDEORA', 'LDEORAL', 'LDEORL', 'LDSET', 'LDSETA', 'LDSETAL', 'LDSETL', 'LDSMAX', 'LDSMAXA', 'LDSMAXAL', 'LDSMAXL', 'LDSMIN', 'LDSMINA', 'LDSMINAL', 'LDSMINL', 'LDUMAX', 'LDUMAXA', 'LDUMAXAL', 'LDUMAXL', 'LDUMIN', 'LDUMINA', 'LDUMINAL', 'LDUMINL', 'SWP', 'SWPA', 'SWPAL', 'SWPL'],
    '<Xs>,<Xt>,[<Xn|SP>]', (('Rs', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, X, RefBase',
    processor = 'R(16), R(0), R(5)',
)

tlentry(['CAS', 'CASA', 'CASAL', 'CASL'],
    '<Xs>,<Xt>,[<Xn|SP>{,#0}]', (('Rs', 5, 16), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, X, RefBase',
    processor = 'R(16), R(0), R(5)',
)

tlentry(['STADD', 'STADDL', 'STCLR', 'STCLRL', 'STEOR', 'STEORL', 'STSET', 'STSETL', 'STSMAX', 'STSMAXL', 'STSMIN', 'STSMINL', 'STUMAX', 'STUMAXL', 'STUMIN', 'STUMINL'],
    '<Xs>,[<Xn|SP>]', (('Rs', 5, 16), ('Rn', 5, 5)),
    matcher   = 'X, RefBase',
    processor = 'R(16), R(5)',
)

tlentry(['LDP', 'STP'],
    '<Xt1>,<Xt2>,[<Xn|SP>,#<imm>]!', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, X, RefPre',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 3)',
)

tlentry(['LDPSW'],
    '<Xt1>,<Xt2>,[<Xn|SP>,#<imm>]!', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, X, RefPre',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 2)',
)

tlentry(['LDP', 'STP'],
    '<Xt1>,<Xt2>,[<Xn|SP>],#<imm>', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, X, RefBase, Imm',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 3)',
)

tlentry(['LDPSW'],
    '<Xt1>,<Xt2>,[<Xn|SP>],#<imm>', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, X, RefBase, Imm',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 2)',
)

tlentry(['LDAXP', 'LDXP'],
    '<Xt1>,<Xt2>,[<Xn|SP>{,#0}]', (('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, X, RefBase',
    processor = 'R(0), R(10), R(5)',
)

tlentry(['LDNP', 'LDP', 'STNP', 'STP'],
    '<Xt1>,<Xt2>,[<Xn|SP>{,#<imm>}]', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, X, RefOffset',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 3)',
)

tlentry(['LDPSW'],
    '<Xt1>,<Xt2>,[<Xn|SP>{,#<imm>}]', (('imm7', 7, 15), ('Rt2', 5, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, X, RefOffset',
    processor = 'R(0), R(10), R(5), Sscaled(15, 7, 2)',
)

tlentry(['CBNZ', 'CBZ', 'LDR', 'LDRSW'],
    '<Xt>,<label>', (('imm19', 19, 5), ('Rt', 5, 0)),
    matcher   = 'X, Offset',
    processor = 'R(0), Offset(BCOND)',
)

tlentry(['LDR', 'LDRSB', 'LDRSH', 'LDRSW', 'STR'],
    '<Xt>,[<Xn|SP>,#<simm>]!', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefPre',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDRSB'],
    '<Xt>,[<Xn|SP>,(<Wm>|<Xm>),<extend>{<amount>}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 0])',
)

tlentry(['LDRSH'],
    '<Xt>,[<Xn|SP>,(<Wm>|<Xm>){,<extend>{<amount>}}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 1])',
)

tlentry(['LDRSW'],
    '<Xt>,[<Xn|SP>,(<Wm>|<Xm>){,<extend>{<amount>}}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 2])',
)

tlentry(['LDR', 'STR'],
    '<Xt>,[<Xn|SP>,(<Wm>|<Xm>){,<extend>{<amount>}}]', (('Rm', 5, 16), ('option', 3, 13), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefIndex',
    processor = 'R(0), R(5), R(16), ExtendsX(13), Ulist(12, &[0, 3])',
)

tlentry(['LDRSB'],
    '<Xt>,[<Xn|SP>,<Xm>{,LSL<amount>}]', (('Rm', 5, 16), ('S', 1, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    forget = True
)

tlentry(['LDR', 'LDRSB', 'LDRSH', 'LDRSW', 'STR'],
    '<Xt>,[<Xn|SP>],#<simm>', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefBase, Imm',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDAPR', 'LDAR', 'LDAXR', 'LDLAR', 'LDXR', 'STLLR', 'STLR'],
    '<Xt>,[<Xn|SP>{,#0}]', (('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefBase',
    processor = 'R(0), R(5)',
)

tlentry(['LDR', 'STR'],
    '<Xt>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefOffset',
    processor = 'R(0), R(5), Uscaled(10, 12, 3)',
)

tlentry(['LDRSW'],
    '<Xt>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefOffset',
    processor = 'R(0), R(5), Uscaled(10, 12, 2)',
)

tlentry(['LDRSH'],
    '<Xt>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefOffset',
    processor = 'R(0), R(5), Uscaled(10, 12, 1)',
)

tlentry(['LDRSB'],
    '<Xt>,[<Xn|SP>{,#<pimm>}]', (('imm12', 12, 10), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefOffset',
    processor = 'R(0), R(5), Ubits(10, 12)',
)

tlentry(['LDRAA', 'LDRAB'],
    '<Xt>,[<Xn|SP>{,#<simm>}]', (('S', 1, 22), ('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefOffset',
    processor = 'R(0), R(5), BSscaled(10, 3), Sslice(12, 9, 3), Sslice(22, 1, 12), A', # immediate is sliced S:imm9
)

tlentry(['LDAPUR', 'LDAPURSB', 'LDAPURSH', 'LDAPURSW', 'LDTR', 'LDTRSB', 'LDTRSH', 'LDTRSW', 'LDUR', 'LDURSB', 'LDURSH', 'LDURSW', 'STLUR', 'STTR', 'STUR'],
    '<Xt>,[<Xn|SP>{,#<simm>}]', (('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefOffset',
    processor = 'R(0), R(5), Sbits(12, 9)',
)

tlentry(['LDRAA', 'LDRAB'],
    '<Xt>,[<Xn|SP>{,#<simm>}]!', (('S', 1, 22), ('imm9', 9, 12), ('Rn', 5, 5), ('Rt', 5, 0)),
    matcher   = 'X, RefPre',
    processor = 'R(0), R(5), BSscaled(10, 3), Sslice(12, 9, 3), Sslice(22, 1, 12), A', # immediate is sliced S:imm9
)

tlentry(['B', 'BL'],
    '<label>', (('imm26', 26, 0),),
    matcher   = 'Offset',
    processor = 'Offset(B)',
)

tlentry(['RET'],
    '{<Xn>}', (('Rn', 5, 5),),
    matcher   = 'X',
    processor = 'R(5)', # weird default
    matchers  =[''],
    processors=['Static(5, 0b11110)'],
)
