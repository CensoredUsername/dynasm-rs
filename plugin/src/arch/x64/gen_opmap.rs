Ops!(OPMAP;

"adc" = [
    b"Abib"       , [0x14              ], X;
    b"mbib"       , [0x80              ], 2, LOCK;
    b"mbrb"       , [0x10              ], X, LOCK | ENC_MR;
    b"rbib"       , [0x80              ], 2;
    b"rbrb"       , [0x10              ], X, ENC_MR;
    b"rbvb"       , [0x12              ], X;
    b"r*ib"       , [0x83              ], 2, AUTO_SIZE | LOCK| EXACT_SIZE;
    b"A*i*"       , [0x15              ], X, AUTO_SIZE;
    b"m*i*"       , [0x81              ], 2, AUTO_SIZE | LOCK;
    b"m*ib"       , [0x83              ], 2, AUTO_SIZE | LOCK;
    b"m*r*"       , [0x11              ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*i*"       , [0x81              ], 2, AUTO_SIZE | LOCK;
    b"r*r*"       , [0x11              ], X, AUTO_SIZE | ENC_MR;
    b"r*v*"       , [0x13              ], X, AUTO_SIZE;
]
"adcx" = [
    b"rqvq"       , [0x0F, 0x38, 0xF6  ], X, WITH_REXW | PREF_66;
]
"add" = [
    b"Abib"       , [0x04              ], X;
    b"mbib"       , [0x80              ], 0, LOCK;
    b"mbrb"       , [0x00              ], X, LOCK | ENC_MR;
    b"rbib"       , [0x80              ], 0;
    b"rbrb"       , [0x00              ], X, ENC_MR;
    b"rbvb"       , [0x02              ], X;
    b"r*ib"       , [0x83              ], 0, AUTO_SIZE | LOCK| EXACT_SIZE;
    b"A*i*"       , [0x05              ], X, AUTO_SIZE;
    b"m*i*"       , [0x81              ], 0, AUTO_SIZE | LOCK;
    b"m*ib"       , [0x83              ], 0, AUTO_SIZE | LOCK;
    b"m*r*"       , [0x01              ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*i*"       , [0x81              ], 0, AUTO_SIZE | LOCK;
    b"r*r*"       , [0x01              ], X, AUTO_SIZE | ENC_MR;
    b"r*v*"       , [0x03              ], X, AUTO_SIZE;
]
"addpd" = [
    b"yowo"       , [0x0F, 0x58        ], X, PREF_66 | SSE2;
]
"addps" = [
    b"yowo"       , [0x0F, 0x58        ], X, SSE;
]
"addsd" = [
    b"yoyo"       , [0x0F, 0x58        ], X, PREF_F2 | SSE2;
]
"addss" = [
    b"yoyo"       , [0x0F, 0x58        ], X, PREF_F3 | SSE;
]
"addsubpd" = [
    b"yowo"       , [0x0F, 0xD0        ], X, PREF_66 | SSE3;
]
"addsubps" = [
    b"yowo"       , [0x0F, 0xD0        ], X, PREF_F2 | SSE3;
]
"adox" = [
    b"rqvq"       , [0x0F, 0x38, 0xF6  ], X, WITH_REXW | PREF_F3;
]
"aesdec" = [
    b"yowo"       , [0x0F, 0x38, 0xDE  ], X, PREF_66 | SSE;
]
"aesdeclast" = [
    b"yowo"       , [0x0F, 0x38, 0xDF  ], X, PREF_66 | SSE;
]
"aesenc" = [
    b"yowo"       , [0x0F, 0x38, 0xDC  ], X, PREF_66 | SSE;
]
"aesenclast" = [
    b"yowo"       , [0x0F, 0x38, 0xDD  ], X, PREF_66 | SSE;
]
"aesimc" = [
    b"yowo"       , [0x0F, 0x38, 0xDB  ], X, PREF_66 | SSE;
]
"aeskeygenassist" = [
    b"yowoib"     , [0x0F, 0x3A, 0xDF  ], X, PREF_66 | SSE;
]
"and" = [
    b"Abib"       , [0x24              ], X;
    b"mbib"       , [0x80              ], 4, LOCK;
    b"mbrb"       , [0x20              ], X, LOCK | ENC_MR;
    b"rbib"       , [0x80              ], 4;
    b"rbrb"       , [0x20              ], X, ENC_MR;
    b"rbvb"       , [0x22              ], X;
    b"r*ib"       , [0x83              ], 4, AUTO_SIZE | LOCK| EXACT_SIZE;
    b"A*i*"       , [0x25              ], X, AUTO_SIZE;
    b"m*i*"       , [0x81              ], 4, AUTO_SIZE | LOCK;
    b"m*ib"       , [0x83              ], 4, AUTO_SIZE | LOCK;
    b"m*r*"       , [0x21              ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*i*"       , [0x81              ], 4, AUTO_SIZE | LOCK;
    b"r*r*"       , [0x21              ], X, AUTO_SIZE | ENC_MR;
    b"r*v*"       , [0x23              ], X, AUTO_SIZE;
]
"andnpd" = [
    b"yowo"       , [0x0F, 0x55        ], X, PREF_66 | SSE2;
]
"andnps" = [
    b"yowo"       , [0x0F, 0x55        ], X, SSE;
]
"andpd" = [
    b"yowo"       , [0x0F, 0x54        ], X, PREF_66 | SSE2;
]
"andps" = [
    b"yowo"       , [0x0F, 0x54        ], X, SSE;
]
"blendpd" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x0D  ], X, PREF_66 | SSE41;
]
"blendps" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x0C  ], X, PREF_66 | SSE41;
]
"blendvpd" = [
    b"yoyo"       , [0x0F, 0x38, 0x15  ], X, PREF_66 | SSE41;
]
"blendvps" = [
    b"yoyo"       , [0x0F, 0x38, 0x14  ], X, PREF_66 | SSE41;
]
"bndcl" = [
    b"bom!"       , [0x0F, 0x1A        ], X, PREF_F3 | MPX;
]
"bndcn" = [
    b"bom!"       , [0x0F, 0x1B        ], X, PREF_F2 | MPX;
]
"bndcu" = [
    b"bom!"       , [0x0F, 0x1A        ], X, PREF_F2 | MPX;
]
"bndldx" = [
    b"bom!"       , [0x0F, 0x1A        ], X, ENC_MIB | MPX;
    b"bom!rq"     , [0x0F, 0x1A        ], X, ENC_MIB | MPX;
]
"bndmk" = [
    b"bom!"       , [0x0F, 0x1B        ], X, ENC_MIB | PREF_F3 | MPX;
]
"bndmov" = [
    b"bobo"       , [0x0F, 0x1A        ], X, PREF_66 | MPX;
    b"bobo"       , [0x0F, 0x1B        ], X, ENC_MR | PREF_66 | MPX;
]
"bndstx" = [
    b"m!bo"       , [0x0F, 0x1B        ], X, ENC_MR | ENC_MIB | MPX;
    b"m!borq"     , [0x0F, 0x1B        ], X, ENC_MR | ENC_MIB | MPX;
    b"m!rqbo"     , [0x0F, 0x1B        ], X, ENC_MIB | MPX;
]
"bsf" = [
    b"r*v*"       , [0x0F, 0xBC        ], X, AUTO_SIZE;
]
"bsr" = [
    b"r*v*"       , [0x0F, 0xBD        ], X, AUTO_SIZE;
]
"bswap" = [
    b"r*"         , [0x0F, 0xC8        ], X, SHORT_ARG;
]
"bt" = [
    b"v*ib"       , [0x0F, 0xBA        ], 4, AUTO_SIZE;
    b"v*r*"       , [0x0F, 0xA3        ], X, AUTO_SIZE | ENC_MR;
]
"btc" = [
    b"r*ib"       , [0x0F, 0xBA        ], 7, AUTO_SIZE | LOCK| EXACT_SIZE;
    b"m*ib"       , [0x0F, 0xBA        ], 7, AUTO_SIZE | LOCK;
    b"m*r*"       , [0x0F, 0xBB        ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*r*"       , [0x0F, 0xBB        ], X, AUTO_SIZE | ENC_MR;
]
"btr" = [
    b"r*ib"       , [0x0F, 0xBA        ], 6, AUTO_SIZE | LOCK| EXACT_SIZE;
    b"m*ib"       , [0x0F, 0xBA        ], 6, AUTO_SIZE | LOCK;
    b"m*r*"       , [0x0F, 0xB3        ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*r*"       , [0x0F, 0xB3        ], X, AUTO_SIZE | ENC_MR;
]
"bts" = [
    b"r*ib"       , [0x0F, 0xBA        ], 5, AUTO_SIZE | LOCK| EXACT_SIZE;
    b"m*ib"       , [0x0F, 0xBA        ], 5, AUTO_SIZE | LOCK;
    b"m*r*"       , [0x0F, 0xAB        ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*r*"       , [0x0F, 0xAB        ], X, AUTO_SIZE | ENC_MR;
]
"cbw" = [
    b""           , [0x98              ], X, WORD_SIZE;
]
"cdq" = [
    b""           , [0x99              ], X;
]
"cdqe" = [
    b""           , [0x98              ], X, WITH_REXW;
]
"clac" = [
    b""           , [0x0F, 0x01, 0xCA  ], X;
]
"clc" = [
    b""           , [0xF8              ], X;
]
"cld" = [
    b""           , [0xFC              ], X;
]
"clflush" = [
    b"mb"         , [0x0F, 0xAE        ], 7, SSE2;
]
"clgi" = [
    b""           , [0x0F, 0x01, 0xDD  ], X, AMD | VMX;
]
"cli" = [
    b""           , [0xFA              ], X;
]
"clts" = [
    b""           , [0x0F, 0x06        ], X;
]
"clzero" = [
    b""           , [0x0F, 0x01, 0xFC  ], X, AMD;
]
"cmc" = [
    b""           , [0xF5              ], X;
]
"cmp" = [
    b"Abib"       , [0x3C              ], X;
    b"rbvb"       , [0x3A              ], X;
    b"vbib"       , [0x80              ], 7;
    b"vbrb"       , [0x38              ], X, ENC_MR;
    b"A*i*"       , [0x3D              ], X, AUTO_SIZE;
    b"r*v*"       , [0x3B              ], X, AUTO_SIZE;
    b"v*i*"       , [0x81              ], 7, AUTO_SIZE;
    b"v*ib"       , [0x83              ], 7, AUTO_SIZE;
    b"v*r*"       , [0x39              ], X, AUTO_SIZE | ENC_MR;
]
"cmpeqpd" = [
    b"yowo"       , [0x0F, 0xC2, 0x00  ], X, PREF_66 | SSE2;
]
"cmpeqps" = [
    b"yowo"       , [0x0F, 0xC2, 0x00  ], X, SSE;
]
"cmpeqsd" = [
    b"yoyo"       , [0x0F, 0xC2, 0x00  ], X, PREF_F2 | SSE2;
]
"cmpeqss" = [
    b"yoyo"       , [0x0F, 0xC2, 0x00  ], X, PREF_F3 | SSE;
]
"cmplepd" = [
    b"yowo"       , [0x0F, 0xC2, 0x02  ], X, PREF_66 | SSE2;
]
"cmpleps" = [
    b"yowo"       , [0x0F, 0xC2, 0x02  ], X, SSE;
]
"cmplesd" = [
    b"yoyo"       , [0x0F, 0xC2, 0x02  ], X, PREF_F2 | SSE2;
]
"cmpless" = [
    b"yoyo"       , [0x0F, 0xC2, 0x02  ], X, PREF_F3 | SSE;
]
"cmpltpd" = [
    b"yowo"       , [0x0F, 0xC2, 0x01  ], X, PREF_66 | SSE2;
]
"cmpltps" = [
    b"yowo"       , [0x0F, 0xC2, 0x01  ], X, SSE;
]
"cmpltsd" = [
    b"yoyo"       , [0x0F, 0xC2, 0x01  ], X, PREF_F2 | SSE2;
]
"cmpltss" = [
    b"yoyo"       , [0x0F, 0xC2, 0x01  ], X, PREF_F3 | SSE;
]
"cmpneqpd" = [
    b"yowo"       , [0x0F, 0xC2, 0x04  ], X, PREF_66 | SSE2;
]
"cmpneqps" = [
    b"yowo"       , [0x0F, 0xC2, 0x04  ], X, SSE;
]
"cmpneqsd" = [
    b"yoyo"       , [0x0F, 0xC2, 0x04  ], X, PREF_F2 | SSE2;
]
"cmpneqss" = [
    b"yoyo"       , [0x0F, 0xC2, 0x04  ], X, PREF_F3 | SSE;
]
"cmpnlepd" = [
    b"yowo"       , [0x0F, 0xC2, 0x06  ], X, PREF_66 | SSE2;
]
"cmpnleps" = [
    b"yowo"       , [0x0F, 0xC2, 0x06  ], X, SSE;
]
"cmpnlesd" = [
    b"yoyo"       , [0x0F, 0xC2, 0x06  ], X, PREF_F2 | SSE2;
]
"cmpnless" = [
    b"yoyo"       , [0x0F, 0xC2, 0x06  ], X, PREF_F3 | SSE;
]
"cmpnltpd" = [
    b"yowo"       , [0x0F, 0xC2, 0x05  ], X, PREF_66 | SSE2;
]
"cmpnltps" = [
    b"yowo"       , [0x0F, 0xC2, 0x05  ], X, SSE;
]
"cmpnltsd" = [
    b"yoyo"       , [0x0F, 0xC2, 0x05  ], X, PREF_F2 | SSE2;
]
"cmpnltss" = [
    b"yoyo"       , [0x0F, 0xC2, 0x05  ], X, PREF_F3 | SSE;
]
"cmpordpd" = [
    b"yowo"       , [0x0F, 0xC2, 0x07  ], X, PREF_66 | SSE2;
]
"cmpordps" = [
    b"yowo"       , [0x0F, 0xC2, 0x07  ], X, SSE;
]
"cmpordsd" = [
    b"yoyo"       , [0x0F, 0xC2, 0x07  ], X, PREF_F2 | SSE2;
]
"cmpordss" = [
    b"yoyo"       , [0x0F, 0xC2, 0x07  ], X, PREF_F3 | SSE;
]
"cmppd" = [
    b"yowoib"     , [0x0F, 0xC2        ], X, PREF_66 | SSE2;
]
"cmpps" = [
    b"yom!ib"     , [0x0F, 0xC2        ], X, SSE;
]
"cmpsb" = [
    b""           , [0xA6              ], X, REPE;
]
"cmpsd" = [
    b""           , [0xA7              ], X, REPE;
    b"yowoib"     , [0x0F, 0xC2        ], X, PREF_F2 | SSE2;
]
"cmpsq" = [
    b""           , [0xA7              ], X, REPE | WITH_REXW;
]
"cmpss" = [
    b"yom!ib"     , [0x0F, 0xC2        ], X, PREF_F3 | SSE;
]
"cmpsw" = [
    b""           , [0xA7              ], X, REPE | WORD_SIZE;
]
"cmpunordpd" = [
    b"yowo"       , [0x0F, 0xC2, 0x03  ], X, PREF_66 | SSE2;
]
"cmpunordps" = [
    b"yowo"       , [0x0F, 0xC2, 0x03  ], X, SSE;
]
"cmpunordsd" = [
    b"yoyo"       , [0x0F, 0xC2, 0x03  ], X, PREF_F2 | SSE2;
]
"cmpunordss" = [
    b"yoyo"       , [0x0F, 0xC2, 0x03  ], X, PREF_F3 | SSE;
]
"cmpxchg" = [
    b"mbrb"       , [0x0F, 0xB0        ], X, LOCK | ENC_MR;
    b"rbrb"       , [0x0F, 0xB0        ], X, ENC_MR;
    b"m*r*"       , [0x0F, 0xB1        ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*r*"       , [0x0F, 0xB1        ], X, AUTO_SIZE | ENC_MR;
]
"cmpxchg16b" = [
    b"mo"         , [0x0F, 0xC7        ], 1, LOCK | WITH_REXW;
]
"cmpxchg8b" = [
    b"mq"         , [0x0F, 0xC7        ], 1, LOCK;
]
"comisd" = [
    b"yoyo"       , [0x0F, 0x2F        ], X, PREF_66 | SSE2;
]
"comiss" = [
    b"yoyo"       , [0x0F, 0x2F        ], X, SSE;
]
"cpu_read" = [
    b""           , [0x0F, 0x3D        ], X, CYRIX;
]
"cpu_write" = [
    b""           , [0x0F, 0x3C        ], X, CYRIX;
]
"cpuid" = [
    b""           , [0x0F, 0xA2        ], X;
]
"cqo" = [
    b""           , [0x99              ], X, WITH_REXW;
]
"cvtdq2pd" = [
    b"yoyo"       , [0x0F, 0xE6        ], X, PREF_F3 | SSE2;
]
"cvtdq2ps" = [
    b"yowo"       , [0x0F, 0x5B        ], X, SSE2;
]
"cvtpd2dq" = [
    b"yowo"       , [0x0F, 0xE6        ], X, PREF_F2 | SSE2;
]
"cvtpd2pi" = [
    b"xqwo"       , [0x0F, 0x2D        ], X, PREF_66 | SSE2;
]
"cvtpd2ps" = [
    b"yowo"       , [0x0F, 0x5A        ], X, PREF_66 | SSE2;
]
"cvtpi2pd" = [
    b"youq"       , [0x0F, 0x2A        ], X, PREF_66 | SSE2;
]
"cvtpi2ps" = [
    b"youq"       , [0x0F, 0x2A        ], X, SSE | MMX;
]
"cvtps2dq" = [
    b"yowo"       , [0x0F, 0x5B        ], X, PREF_66 | SSE2;
]
"cvtps2pd" = [
    b"yoyo"       , [0x0F, 0x5A        ], X, SSE2;
]
"cvtps2pi" = [
    b"xqyo"       , [0x0F, 0x2D        ], X, SSE | MMX;
]
"cvtsd2si" = [
    b"r*mq"       , [0x0F, 0x2D        ], X, AUTO_REXW | PREF_F2 | SSE2;
    b"r*yo"       , [0x0F, 0x2D        ], X, AUTO_REXW | PREF_F2 | SSE2;
]
"cvtsd2ss" = [
    b"yoyo"       , [0x0F, 0x5A        ], X, PREF_F2 | SSE2;
]
"cvtsi2sd" = [
    b"yov*"       , [0x0F, 0x2A        ], X, WITH_REXW | PREF_F2 | SSE2;
]
"cvtsi2ss" = [
    b"yov*"       , [0x0F, 0x2A        ], X, WITH_REXW | PREF_F3 | SSE;
]
"cvtss2sd" = [
    b"yoyo"       , [0x0F, 0x5A        ], X, PREF_F3 | SSE2;
]
"cvtss2si" = [
    b"r*md"       , [0x0F, 0x2D        ], X, AUTO_REXW | PREF_F3 | SSE;
    b"r*yo"       , [0x0F, 0x2D        ], X, AUTO_REXW | PREF_F3 | SSE;
]
"cvttpd2dq" = [
    b"yowo"       , [0x0F, 0xE6        ], X, PREF_66 | SSE2;
]
"cvttpd2pi" = [
    b"xqwo"       , [0x0F, 0x2C        ], X, PREF_66 | SSE2;
]
"cvttps2dq" = [
    b"yowo"       , [0x0F, 0x5B        ], X, PREF_F3 | SSE2;
]
"cvttps2pi" = [
    b"xqyo"       , [0x0F, 0x2C        ], X, MMX | SSE;
]
"cvttsd2si" = [
    b"r*mq"       , [0x0F, 0x2C        ], X, AUTO_REXW | PREF_F2 | SSE2;
    b"r*yo"       , [0x0F, 0x2C        ], X, AUTO_REXW | PREF_F2 | SSE2;
]
"cvttss2si" = [
    b"r*md"       , [0x0F, 0x2C        ], X, AUTO_REXW | PREF_F3 | SSE;
    b"r*yo"       , [0x0F, 0x2C        ], X, AUTO_REXW | PREF_F3 | SSE;
]
"cwd" = [
    b""           , [0x99              ], X, WORD_SIZE;
]
"cwde" = [
    b""           , [0x98              ], X;
]
"dec" = [
    b"mb"         , [0xFE              ], 1, LOCK;
    b"rb"         , [0xFE              ], 1;
    b"m*"         , [0xFF              ], 1, AUTO_SIZE | LOCK;
    b"r*"         , [0xFF              ], 1, AUTO_SIZE | LOCK;
]
"div" = [
    b"vb"         , [0xF6              ], 6;
    b"v*"         , [0xF7              ], 6, AUTO_SIZE;
]
"divpd" = [
    b"yowo"       , [0x0F, 0x5E        ], X, PREF_66 | SSE2;
]
"divps" = [
    b"yowo"       , [0x0F, 0x5E        ], X, SSE;
]
"divsd" = [
    b"yoyo"       , [0x0F, 0x5E        ], X, PREF_F2 | SSE2;
]
"divss" = [
    b"yoyo"       , [0x0F, 0x5E        ], X, PREF_F3 | SSE;
]
"dmint" = [
    b""           , [0x0F, 0x39        ], X, CYRIX;
]
"dppd" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x41  ], X, PREF_66 | SSE41;
]
"dpps" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x40  ], X, PREF_66 | SSE41;
]
"emms" = [
    b""           , [0x0F, 0x77        ], X, MMX;
]
"enter" = [
    b"iwib"       , [0xC8              ], X;
]
"extractps" = [
    b"vdyoib"     , [0x0F, 0x3A, 0x17  ], X, WITH_REXW | ENC_MR | PREF_66 | SSE41;
]
"extrq" = [
    b"yoibib"     , [0x0F, 0x78        ], 0, PREF_66 | AMD | SSE4A;
    b"yoyo"       , [0x0F, 0x79        ], X, PREF_66 | AMD | SSE4A;
]
"f2xm1" = [
    b""           , [0xD9, 0xF0        ], X, FPU;
]
"fabs" = [
    b""           , [0xD9, 0xE1        ], X, FPU;
]
"fadd" = [
    b""           , [0xDE, 0xC1        ], X, FPU;
    b"Xpfp"       , [0xD8, 0xC0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xD8, 0xC0        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xC0        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xC0        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD8              ], 0, FPU;
    b"mq"         , [0xDC              ], 0, FPU;
]
"faddp" = [
    b""           , [0xDE, 0xC1        ], X, FPU;
    b"fp"         , [0xDE, 0xC0        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDE, 0xC0        ], X, SHORT_ARG | FPU;
]
"fbld" = [
    b"m!"         , [0xDF              ], 4, FPU;
]
"fbstp" = [
    b"m!"         , [0xDF              ], 6, FPU;
]
"fchs" = [
    b""           , [0xD9, 0xE0        ], X, FPU;
]
"fclex" = [
    b""           , [0x9B, 0xDB, 0xE2  ], X, FPU;
]
"fcmovb" = [
    b""           , [0xDA, 0xC1        ], X, FPU;
    b"Xpfp"       , [0xDA, 0xC0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDA, 0xC0        ], X, SHORT_ARG | FPU;
]
"fcmovbe" = [
    b""           , [0xDA, 0xD1        ], X, FPU;
    b"Xpfp"       , [0xDA, 0xD0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDA, 0xD0        ], X, SHORT_ARG | FPU;
]
"fcmove" = [
    b""           , [0xDA, 0xC9        ], X, FPU;
    b"Xpfp"       , [0xDA, 0xC8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDA, 0xC8        ], X, SHORT_ARG | FPU;
]
"fcmovnb" = [
    b""           , [0xDB, 0xC1        ], X, FPU;
    b"Xpfp"       , [0xDB, 0xC0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDB, 0xC0        ], X, SHORT_ARG | FPU;
]
"fcmovnbe" = [
    b""           , [0xDB, 0xD1        ], X, FPU;
    b"Xpfp"       , [0xDB, 0xD0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDB, 0xD0        ], X, SHORT_ARG | FPU;
]
"fcmovne" = [
    b""           , [0xDB, 0xC9        ], X, FPU;
    b"Xpfp"       , [0xDB, 0xC8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDB, 0xC8        ], X, SHORT_ARG | FPU;
]
"fcmovnu" = [
    b""           , [0xDB, 0xD9        ], X, FPU;
    b"Xpfp"       , [0xDB, 0xD8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDB, 0xD8        ], X, SHORT_ARG | FPU;
]
"fcmovu" = [
    b""           , [0xDA, 0xD9        ], X, FPU;
    b"Xpfp"       , [0xDA, 0xD8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDA, 0xD8        ], X, SHORT_ARG | FPU;
]
"fcom" = [
    b""           , [0xD8, 0xD1        ], X, FPU;
    b"Xpfp"       , [0xD8, 0xD0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xD8, 0xD0        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD8              ], 2, FPU;
    b"mq"         , [0xDC              ], 2, FPU;
]
"fcomi" = [
    b""           , [0xDB, 0xF1        ], X, FPU;
    b"Xpfp"       , [0xDB, 0xF0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDB, 0xF0        ], X, SHORT_ARG | FPU;
]
"fcomip" = [
    b""           , [0xDF, 0xF1        ], X, FPU;
    b"Xpfp"       , [0xDF, 0xF0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDF, 0xF0        ], X, SHORT_ARG | FPU;
]
"fcomp" = [
    b""           , [0xD8, 0xD9        ], X, FPU;
    b"Xpfp"       , [0xD8, 0xD8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xD8, 0xD8        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD8              ], 3, FPU;
    b"mq"         , [0xDC              ], 3, FPU;
]
"fcompp" = [
    b""           , [0xDE, 0xD9        ], X, FPU;
]
"fcos" = [
    b""           , [0xD9, 0xFF        ], X, FPU;
]
"fdecstp" = [
    b""           , [0xD9, 0xF6        ], X, FPU;
]
"fdisi" = [
    b""           , [0x9B, 0xDB, 0xE1  ], X, FPU;
]
"fdiv" = [
    b""           , [0xDE, 0xF9        ], X, FPU;
    b"Xpfp"       , [0xD8, 0xF0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xD8, 0xF0        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xF8        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xF8        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD8              ], 6, FPU;
    b"mq"         , [0xDC              ], 6, FPU;
]
"fdivp" = [
    b""           , [0xDE, 0xF9        ], X, FPU;
    b"fp"         , [0xDE, 0xF8        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDE, 0xF8        ], X, SHORT_ARG | FPU;
]
"fdivr" = [
    b""           , [0xDE, 0xF1        ], X, FPU;
    b"Xpfp"       , [0xD8, 0xF8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xD8, 0xF8        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xF0        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xF0        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD8              ], 7, FPU;
    b"mq"         , [0xDC              ], 7, FPU;
]
"fdivrp" = [
    b""           , [0xDE, 0xF1        ], X, FPU;
    b"fp"         , [0xDE, 0xF0        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDE, 0xF0        ], X, SHORT_ARG | FPU;
]
"femms" = [
    b""           , [0x0F, 0x0E        ], X, TDNOW;
]
"feni" = [
    b""           , [0x9B, 0xDB, 0xE0  ], X, FPU;
]
"ffree" = [
    b""           , [0xDD, 0xC1        ], X, FPU;
    b"fp"         , [0xDD, 0xC0        ], X, SHORT_ARG | FPU;
]
"fiadd" = [
    b"md"         , [0xDA              ], 0, FPU;
    b"mw"         , [0xDE              ], 0, FPU;
]
"ficom" = [
    b"md"         , [0xDA              ], 2, FPU;
    b"mw"         , [0xDE              ], 2, FPU;
]
"ficomp" = [
    b"md"         , [0xDA              ], 3, FPU;
    b"mw"         , [0xDE              ], 3, FPU;
]
"fidiv" = [
    b"md"         , [0xDA              ], 6, FPU;
    b"mw"         , [0xDE              ], 6, FPU;
]
"fidivr" = [
    b"md"         , [0xDA              ], 7, FPU;
    b"mw"         , [0xDE              ], 7, FPU;
]
"fild" = [
    b"md"         , [0xDB              ], 0, FPU;
    b"mq"         , [0xDF              ], 5, FPU;
    b"mw"         , [0xDF              ], 0, FPU;
]
"fimul" = [
    b"md"         , [0xDA              ], 1, FPU;
    b"mw"         , [0xDE              ], 1, FPU;
]
"fincstp" = [
    b""           , [0xD9, 0xF7        ], X, FPU;
]
"finit" = [
    b""           , [0x9B, 0xDB, 0xE3  ], X, FPU;
]
"fist" = [
    b"md"         , [0xDB              ], 2, FPU;
    b"mw"         , [0xDF              ], 2, FPU;
]
"fistp" = [
    b"md"         , [0xDB              ], 3, FPU;
    b"mq"         , [0xDF              ], 7, FPU;
    b"mw"         , [0xDF              ], 3, FPU;
]
"fisttp" = [
    b"md"         , [0xDB              ], 1, FPU;
    b"mq"         , [0xDD              ], 1, FPU;
    b"mw"         , [0xDF              ], 1, FPU;
]
"fisub" = [
    b"md"         , [0xDA              ], 4, FPU;
    b"mw"         , [0xDE              ], 4, FPU;
]
"fisubr" = [
    b"md"         , [0xDA              ], 5, FPU;
    b"mw"         , [0xDE              ], 5, FPU;
]
"fld" = [
    b""           , [0xD9, 0xC1        ], X, FPU;
    b"fp"         , [0xD9, 0xC0        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD9              ], 0, FPU;
    b"mp"         , [0xDB              ], 5, FPU;
    b"mq"         , [0xDD              ], 0, FPU;
]
"fld1" = [
    b""           , [0xD9, 0xE8        ], X, FPU;
]
"fldcw" = [
    b"mw"         , [0xD9              ], 5, FPU;
]
"fldenv" = [
    b"m!"         , [0xD9              ], 4, FPU;
]
"fldl2e" = [
    b""           , [0xD9, 0xEA        ], X, FPU;
]
"fldl2t" = [
    b""           , [0xD9, 0xE9        ], X, FPU;
]
"fldlg2" = [
    b""           , [0xD9, 0xEC        ], X, FPU;
]
"fldln2" = [
    b""           , [0xD9, 0xED        ], X, FPU;
]
"fldpi" = [
    b""           , [0xD9, 0xEB        ], X, FPU;
]
"fldz" = [
    b""           , [0xD9, 0xEE        ], X, FPU;
]
"fmul" = [
    b""           , [0xDE, 0xC9        ], X, FPU;
    b"Xpfp"       , [0xD8, 0xC8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xD8, 0xC8        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xC8        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xC8        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD8              ], 1, FPU;
    b"mq"         , [0xDC              ], 1, FPU;
]
"fmulp" = [
    b""           , [0xDE, 0xC9        ], X, FPU;
    b"fp"         , [0xDE, 0xC8        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDE, 0xC8        ], X, SHORT_ARG | FPU;
]
"fnclex" = [
    b""           , [0xDB, 0xE2        ], X, FPU;
]
"fndisi" = [
    b""           , [0xDB, 0xE1        ], X, FPU;
]
"fneni" = [
    b""           , [0xDB, 0xE0        ], X, FPU;
]
"fninit" = [
    b""           , [0xDB, 0xE3        ], X, FPU;
]
"fnop" = [
    b""           , [0xD9, 0xD0        ], X, FPU;
]
"fnsave" = [
    b"m!"         , [0xDD              ], 6, FPU;
]
"fnstcw" = [
    b"mw"         , [0xD9              ], 7, FPU;
]
"fnstenv" = [
    b"m!"         , [0xD9              ], 6, FPU;
]
"fnstsw" = [
    b"Aw"         , [0xDF, 0xE0        ], X, FPU;
    b"mw"         , [0xDD              ], 7, FPU;
]
"fpatan" = [
    b""           , [0xD9              ], X, PREF_F3 | FPU;
]
"fprem" = [
    b""           , [0xD9, 0xF8        ], X, FPU;
]
"fprem1" = [
    b""           , [0xD9, 0xF5        ], X, FPU;
]
"fptan" = [
    b""           , [0xD9              ], X, PREF_F2 | FPU;
]
"frndint" = [
    b""           , [0xD9, 0xFC        ], X, FPU;
]
"frstor" = [
    b"m!"         , [0xDD              ], 4, FPU;
]
"fsave" = [
    b"m!"         , [0x9B, 0xDD        ], 6, FPU;
]
"fscale" = [
    b""           , [0xD9, 0xFD        ], X, FPU;
]
"fsetpm" = [
    b""           , [0xDB, 0xE4        ], X, FPU;
]
"fsin" = [
    b""           , [0xD9, 0xFE        ], X, FPU;
]
"fsincos" = [
    b""           , [0xD9, 0xFB        ], X, FPU;
]
"fsqrt" = [
    b""           , [0xD9, 0xFA        ], X, FPU;
]
"fst" = [
    b""           , [0xDD, 0xD1        ], X, FPU;
    b"fp"         , [0xDD, 0xD0        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD9              ], 2, FPU;
    b"mq"         , [0xDD              ], 2, FPU;
]
"fstcw" = [
    b"mw"         , [0x9B, 0xD9        ], 7, FPU;
]
"fstenv" = [
    b"m!"         , [0x9B, 0xD9        ], 6, FPU;
]
"fstp" = [
    b""           , [0xDD, 0xD9        ], X, FPU;
    b"fp"         , [0xDD, 0xD8        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD9              ], 3, FPU;
    b"mp"         , [0xDB              ], 7, FPU;
    b"mq"         , [0xDD              ], 3, FPU;
]
"fstsw" = [
    b"Aw"         , [0x9B, 0xDF, 0xE0  ], X, FPU;
    b"mw"         , [0x9B, 0xDD        ], 7, FPU;
]
"fsub" = [
    b""           , [0xDE, 0xE9        ], X, FPU;
    b"Xpfp"       , [0xD8, 0xE0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xD8, 0xE0        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xE8        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xE8        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD8              ], 4, FPU;
    b"mq"         , [0xDC              ], 4, FPU;
]
"fsubp" = [
    b""           , [0xDE, 0xE9        ], X, FPU;
    b"fp"         , [0xDE, 0xE8        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDE, 0xE8        ], X, SHORT_ARG | FPU;
]
"fsubr" = [
    b""           , [0xDE, 0xE1        ], X, FPU;
    b"Xpfp"       , [0xD8, 0xE8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xD8, 0xE8        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xE0        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDC, 0xE0        ], X, SHORT_ARG | FPU;
    b"md"         , [0xD8              ], 5, FPU;
    b"mq"         , [0xDC              ], 5, FPU;
]
"fsubrp" = [
    b""           , [0xDE, 0xE1        ], X, FPU;
    b"fp"         , [0xDE, 0xE0        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xDE, 0xE0        ], X, SHORT_ARG | FPU;
]
"ftst" = [
    b""           , [0xD9, 0xE4        ], X, FPU;
]
"fucom" = [
    b""           , [0xDD, 0xE1        ], X, FPU;
    b"Xpfp"       , [0xDD, 0xE0        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDD, 0xE0        ], X, SHORT_ARG | FPU;
]
"fucomi" = [
    b""           , [0xDB, 0xE9        ], X, FPU;
    b"Xpfp"       , [0xDB, 0xE8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDB, 0xE8        ], X, SHORT_ARG | FPU;
]
"fucomip" = [
    b""           , [0xDF, 0xE9        ], X, FPU;
    b"Xpfp"       , [0xDF, 0xE8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDF, 0xE8        ], X, SHORT_ARG | FPU;
]
"fucomp" = [
    b""           , [0xDD, 0xE9        ], X, FPU;
    b"Xpfp"       , [0xDD, 0xE8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xDD, 0xE8        ], X, SHORT_ARG | FPU;
]
"fucompp" = [
    b""           , [0xDA, 0xE9        ], X, FPU;
]
"fwait" = [
    b""           , [0x9B              ], X;
]
"fxam" = [
    b""           , [0xD9, 0xE5        ], X, FPU;
]
"fxch" = [
    b""           , [0xD9, 0xC9        ], X, FPU;
    b"Xpfp"       , [0xD9, 0xC8        ], X, SHORT_ARG | FPU;
    b"fp"         , [0xD9, 0xC8        ], X, SHORT_ARG | FPU;
    b"fpXp"       , [0xD9, 0xC8        ], X, SHORT_ARG | FPU;
]
"fxrstor" = [
    b"m!"         , [0x0F, 0xAE        ], 1, FPU | SSE;
]
"fxrstor64" = [
    b"m!"         , [0x0F, 0xAE        ], 1, WITH_REXW | SSE | FPU;
]
"fxsave" = [
    b"m!"         , [0x0F, 0xAE        ], 0, SSE | FPU;
]
"fxsave64" = [
    b"m!"         , [0x0F, 0xAE        ], 0, WITH_REXW | SSE | FPU;
]
"fxtract" = [
    b""           , [0xD9, 0xF4        ], X, FPU;
]
"fyl2x" = [
    b""           , [0xD9, 0xF1        ], X, FPU;
]
"fyl2xp1" = [
    b""           , [0xD9, 0xF9        ], X, FPU;
]
"getsec" = [
    b""           , [0x0F, 0x37        ], X;
]
"haddpd" = [
    b"yowo"       , [0x0F, 0x7C        ], X, PREF_66 | SSE3;
]
"haddps" = [
    b"yowo"       , [0x0F, 0x7C        ], X, PREF_F2 | SSE3;
]
"hlt" = [
    b""           , [0xF4              ], X;
]
"hsubpd" = [
    b"yowo"       , [0x0F, 0x7D        ], X, PREF_66 | SSE3;
]
"hsubps" = [
    b"yowo"       , [0x0F, 0x7D        ], X, PREF_F2 | SSE3;
]
"icebp" = [
    b""           , [0xF1              ], X;
]
"idiv" = [
    b"vb"         , [0xF6              ], 7;
    b"v*"         , [0xF7              ], 7, AUTO_SIZE;
]
"inc" = [
    b"mb"         , [0xFE              ], 0, LOCK;
    b"rb"         , [0xFE              ], 0;
    b"m*"         , [0xFF              ], 0, AUTO_SIZE | LOCK;
    b"r*"         , [0xFF              ], 0, AUTO_SIZE | LOCK;
]
"insb" = [
    b""           , [0x6C              ], X;
]
"insd" = [
    b""           , [0x6D              ], X;
]
"insertps" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x21  ], X, PREF_66 | SSE41;
]
"insertq" = [
    b"yoyo"       , [0x0F, 0x79        ], X, PREF_F2 | AMD | SSE4A;
    b"yoyoibib"   , [0x0F, 0x78        ], X, PREF_F2 | SSE4A | AMD;
]
"insw" = [
    b""           , [0x6D              ], X, WORD_SIZE;
]
"int" = [
    b"ib"         , [0xCD              ], X;
]
"int01" = [
    b""           , [0xF1              ], X;
]
"int03" = [
    b""           , [0xCC              ], X;
]
"int1" = [
    b""           , [0xF1              ], X;
]
"int3" = [
    b""           , [0xCC              ], X;
]
"invd" = [
    b""           , [0x0F, 0x08        ], X;
]
"invept" = [
    b"rqmo"       , [0x0F, 0x38, 0x80  ], X, PREF_66 | VMX;
]
"invlpg" = [
    b"m!"         , [0x0F, 0x01        ], 7;
]
"invlpga" = [
    b""           , [0x0F, 0x01, 0xDF  ], X, AMD;
    b"AqBd"       , [0x0F, 0x01, 0xDF  ], X, AMD;
]
"invpcid" = [
    b"rqmo"       , [0x0F, 0x38, 0x82  ], X, PREF_66 | INVPCID;
]
"invvpid" = [
    b"rqmo"       , [0x0F, 0x38, 0x81  ], X, PREF_66 | VMX;
]
"iret" = [
    b""           , [0xCF              ], X;
]
"iretd" = [
    b""           , [0xCF              ], X;
]
"iretq" = [
    b""           , [0xCF              ], X, WITH_REXW;
]
"iretw" = [
    b""           , [0xCF              ], X, WORD_SIZE;
]
"jecxz" = [
    b"ob"         , [0xE3              ], X, PREF_67;
]
"jrcxz" = [
    b"ob"         , [0xE3              ], X;
]
"lahf" = [
    b""           , [0x9F              ], X;
]
"lar" = [
    b"r*mw"       , [0x0F, 0x02        ], X, AUTO_SIZE;
    b"r*r*"       , [0x0F, 0x02        ], X, AUTO_SIZE;
]
"lddqu" = [
    b"yomo"       , [0x0F, 0xF0        ], X, PREF_F2 | SSE3;
]
"ldmxcsr" = [
    b"md"         , [0x0F, 0xAE        ], 2, SSE;
]
"lea" = [
    b"r*m!"       , [0x8D              ], X, AUTO_SIZE;
]
"leave" = [
    b""           , [0xC9              ], X;
]
"lfence" = [
    b""           , [0x0F, 0xAE, 0xE8  ], X, AMD;
]
"lfs" = [
    b"r*m!"       , [0x0F, 0xB4        ], X, AUTO_SIZE;
]
"lgdt" = [
    b"m!"         , [0x0F, 0x01        ], 2;
]
"lgs" = [
    b"r*m!"       , [0x0F, 0xB5        ], X, AUTO_SIZE;
]
"lidt" = [
    b"m!"         , [0x0F, 0x01        ], 3;
]
"lldt" = [
    b"m!"         , [0x0F, 0x00        ], 2;
]
"lmsw" = [
    b"m!"         , [0x0F, 0x01        ], 6;
]
"lodsb" = [
    b""           , [0xAC              ], X;
]
"lodsd" = [
    b""           , [0xAD              ], X;
]
"lodsq" = [
    b""           , [0xAD              ], X, WITH_REXW;
]
"lodsw" = [
    b""           , [0xAD              ], X, WORD_SIZE;
]
"loop" = [
    b"ob"         , [0xE2              ], X;
]
"loope" = [
    b"ob"         , [0xE1              ], X;
]
"loopne" = [
    b"ob"         , [0xE0              ], X;
]
"loopnz" = [
    b"ob"         , [0xE0              ], X;
]
"loopz" = [
    b"ob"         , [0xE1              ], X;
]
"lsl" = [
    b"r*mw"       , [0x0F, 0x03        ], X, AUTO_SIZE;
    b"r*r*"       , [0x0F, 0x03        ], X, AUTO_SIZE;
]
"lss" = [
    b"r*m!"       , [0x0F, 0xB2        ], X, AUTO_SIZE;
]
"ltr" = [
    b"m!"         , [0x0F, 0x00        ], 3;
]
"lzcnt" = [
    b"r*v*"       , [0x0F, 0xBD        ], X, AUTO_SIZE | PREF_F3 | AMD;
]
"maskmovdqu" = [
    b"yoyo"       , [0x0F, 0xF7        ], X, PREF_66 | SSE2;
]
"maskmovq" = [
    b"xqxq"       , [0x0F, 0xF7        ], X, MMX;
]
"maxpd" = [
    b"yowo"       , [0x0F, 0x5F        ], X, PREF_66 | SSE2;
]
"maxps" = [
    b"yowo"       , [0x0F, 0x5F        ], X, SSE;
]
"maxsd" = [
    b"yoyo"       , [0x0F, 0x5F        ], X, PREF_F2 | SSE2;
]
"maxss" = [
    b"yoyo"       , [0x0F, 0x5F        ], X, PREF_F3 | SSE;
]
"mfence" = [
    b""           , [0x0F, 0xAE, 0xF0  ], X, AMD;
]
"minpd" = [
    b"yowo"       , [0x0F, 0x5D        ], X, PREF_66 | SSE2;
]
"minps" = [
    b"yowo"       , [0x0F, 0x5D        ], X, SSE;
]
"minsd" = [
    b"yoyo"       , [0x0F, 0x5D        ], X, PREF_F2 | SSE2;
]
"minss" = [
    b"yoyo"       , [0x0F, 0x5D        ], X, PREF_F3 | SSE;
]
"monitor" = [
    b""           , [0x0F, 0x01, 0xC8  ], X;
    b"AqBdCd"     , [0x0F, 0x01, 0xC8  ], X;
]
"monitorx" = [
    b""           , [0x0F, 0x01, 0xFA  ], X, AMD;
    b"A*BdCd"     , [0x0F, 0x01, 0xFA  ], X, AMD;
]
"montmul" = [
    b""           , [0x0F, 0xA6, 0xC0  ], X, PREF_F3 | CYRIX;
]
"movapd" = [
    b"moyo"       , [0x0F, 0x29        ], X, ENC_MR | PREF_66 | SSE2;
    b"yomo"       , [0x0F, 0x28        ], X, PREF_66 | SSE2;
    b"yoyo"       , [0x0F, 0x28        ], X, PREF_66 | SSE2;
    b"yoyo"       , [0x0F, 0x29        ], X, ENC_MR | PREF_66 | SSE2;
]
"movaps" = [
    b"woyo"       , [0x0F, 0x29        ], X, ENC_MR | SSE;
    b"yowo"       , [0x0F, 0x28        ], X, SSE;
]
"movbe" = [
    b"m*r*"       , [0x0F, 0x38, 0xF1  ], X, AUTO_SIZE | ENC_MR;
    b"r*m*"       , [0x0F, 0x38, 0xF0  ], X, AUTO_SIZE;
]
"movd" = [
    b"mdyo"       , [0x0F, 0x7E        ], X, ENC_MR | PREF_66 | SSE2;
    b"vdxq"       , [0x0F, 0x7E        ], X, ENC_MR | MMX;
    b"vdyo"       , [0x0F, 0x7E        ], X, ENC_MR | PREF_66 | SSE2;
    b"vqxq"       , [0x0F, 0x7E        ], X, WITH_REXW | ENC_MR | MMX;
    b"xqvd"       , [0x0F, 0x6E        ], X, MMX;
    b"xqvq"       , [0x0F, 0x6E        ], X, WITH_REXW | MMX;
    b"yomd"       , [0x0F, 0x6E        ], X, PREF_66 | SSE2;
    b"yovd"       , [0x0F, 0x6E        ], X, PREF_66 | SSE2;
]
"movddup" = [
    b"yoyo"       , [0x0F, 0x12        ], X, PREF_F2 | SSE3;
]
"movdq2q" = [
    b"xqyo"       , [0x0F, 0xD6        ], X, PREF_F2 | SSE2;
]
"movdqa" = [
    b"moyo"       , [0x0F, 0x7F        ], X, ENC_MR | PREF_66 | SSE2;
    b"yomo"       , [0x0F, 0x6F        ], X, PREF_66 | SSE2;
    b"yoyo"       , [0x0F, 0x6F        ], X, PREF_66 | SSE2;
    b"yoyo"       , [0x0F, 0x7F        ], X, ENC_MR | PREF_66 | SSE2;
]
"movdqu" = [
    b"moyo"       , [0x0F, 0x7F        ], X, ENC_MR | PREF_F3 | SSE2;
    b"yomo"       , [0x0F, 0x6F        ], X, PREF_F3 | SSE2;
    b"yoyo"       , [0x0F, 0x6F        ], X, PREF_F3 | SSE2;
    b"yoyo"       , [0x0F, 0x7F        ], X, ENC_MR | PREF_F3 | SSE2;
]
"movhlps" = [
    b"yoyo"       , [0x0F, 0x12        ], X, SSE;
]
"movhpd" = [
    b"m!yo"       , [0x0F, 0x17        ], X, ENC_MR | PREF_66 | SSE2;
    b"yom!"       , [0x0F, 0x16        ], X, PREF_66 | SSE2;
]
"movhps" = [
    b"mqyo"       , [0x0F, 0x17        ], X, ENC_MR | SSE;
    b"yomq"       , [0x0F, 0x16        ], X, SSE;
]
"movlhps" = [
    b"yoyo"       , [0x0F, 0x16        ], X, SSE;
]
"movlpd" = [
    b"mqyo"       , [0x0F, 0x13        ], X, ENC_MR | PREF_66 | SSE2;
    b"yomq"       , [0x0F, 0x12        ], X, PREF_66 | SSE2;
]
"movlps" = [
    b"mqyo"       , [0x0F, 0x13        ], X, ENC_MR | SSE;
    b"yomq"       , [0x0F, 0x12        ], X, SSE;
]
"movmskpd" = [
    b"r*yo"       , [0x0F, 0x50        ], X, AUTO_REXW | PREF_66 | SSE2;
]
"movmskps" = [
    b"r*yo"       , [0x0F, 0x50        ], X, AUTO_REXW | SSE;
]
"movntdq" = [
    b"moyo"       , [0x0F, 0xE7        ], X, ENC_MR | PREF_66 | SSE2;
]
"movntdqa" = [
    b"yomo"       , [0x0F, 0x38, 0x2A  ], X, PREF_66 | SSE41;
]
"movnti" = [
    b"m*r*"       , [0x0F, 0xC3        ], X, AUTO_REXW | ENC_MR;
]
"movntpd" = [
    b"moyo"       , [0x0F, 0x2B        ], X, ENC_MR | PREF_66 | SSE2;
]
"movntps" = [
    b"moyo"       , [0x0F, 0x2B        ], X, ENC_MR | SSE;
]
"movntq" = [
    b"mqxq"       , [0x0F, 0xE7        ], X, ENC_MR | MMX;
]
"movntsd" = [
    b"mqyo"       , [0x0F, 0x2B        ], X, ENC_MR | PREF_F2 | SSE4A | AMD;
]
"movntss" = [
    b"mdyo"       , [0x0F, 0x2B        ], X, ENC_MR | PREF_F3 | AMD | SSE4A;
]
"movq" = [
    b"mqyo"       , [0x0F, 0xD6        ], X, ENC_MR | PREF_66 | SSE2;
    b"uqxq"       , [0x0F, 0x7F        ], X, ENC_MR | MMX;
    b"vqxq"       , [0x0F, 0x7E        ], X, WITH_REXW | ENC_MR | MMX;
    b"vqyo"       , [0x0F, 0x7E        ], X, WITH_REXW | ENC_MR | PREF_66 | SSE2;
    b"xquq"       , [0x0F, 0x6F        ], X, MMX;
    b"xqvq"       , [0x0F, 0x6E        ], X, WITH_REXW | MMX;
    b"yomq"       , [0x0F, 0x7E        ], X, PREF_F3 | SSE2;
    b"yovq"       , [0x0F, 0x6E        ], X, WITH_REXW | PREF_66 | SSE2;
    b"yoyo"       , [0x0F, 0x7E        ], X, PREF_F3 | SSE2;
    b"yoyo"       , [0x0F, 0xD6        ], X, ENC_MR | PREF_66 | SSE2;
]
"movq2dq" = [
    b"yoxq"       , [0x0F, 0xD6        ], X, PREF_F3 | SSE2;
]
"movsb" = [
    b""           , [0xA4              ], X;
]
"movsd" = [
    b""           , [0xA5              ], X;
    b"yoyo"       , [0x0F, 0x10        ], X, PREF_F2 | SSE2;
    b"yoyo"       , [0x0F, 0x11        ], X, ENC_MR | PREF_F2 | SSE2;
]
"movshdup" = [
    b"yoyo"       , [0x0F, 0x16        ], X, PREF_F3 | SSE3;
]
"movsldup" = [
    b"yoyo"       , [0x0F, 0x12        ], X, PREF_F3 | SSE3;
]
"movsq" = [
    b""           , [0xA5              ], X, WITH_REXW;
]
"movss" = [
    b"mdyo"       , [0x0F, 0x11        ], X, ENC_MR | PREF_F3 | SSE;
    b"yoyo"       , [0x0F, 0x10        ], X, PREF_F3 | SSE;
]
"movsw" = [
    b""           , [0xA5              ], X, WORD_SIZE;
]
"movsx" = [
    b"rqvd"       , [0x63              ], X, WITH_REXW;
    b"rwmb"       , [0x0F, 0xBE        ], X, WORD_SIZE;
    b"r*vb"       , [0x0F, 0xBE        ], X, AUTO_SIZE;
    b"r*vw"       , [0x0F, 0xBF        ], X, AUTO_REXW | EXACT_SIZE;
]
"movsxd" = [
    b"rqvd"       , [0x63              ], X, WITH_REXW;
]
"movupd" = [
    b"moyo"       , [0x0F, 0x11        ], X, ENC_MR | PREF_66 | SSE2;
    b"yomo"       , [0x0F, 0x10        ], X, PREF_66 | SSE2;
    b"yoyo"       , [0x0F, 0x10        ], X, PREF_66 | SSE2;
    b"yoyo"       , [0x0F, 0x11        ], X, ENC_MR | PREF_66 | SSE2;
]
"movups" = [
    b"woyo"       , [0x0F, 0x11        ], X, ENC_MR | SSE;
    b"yowo"       , [0x0F, 0x10        ], X, SSE;
]
"movzx" = [
    b"rwmb"       , [0x0F, 0xB6        ], X, WORD_SIZE;
    b"r*vb"       , [0x0F, 0xB6        ], X, AUTO_SIZE;
    b"r*vw"       , [0x0F, 0xB7        ], X, AUTO_REXW | EXACT_SIZE;
]
"mpsadbw" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x42  ], X, PREF_66 | SSE41;
]
"mul" = [
    b"vb"         , [0xF6              ], 4;
    b"v*"         , [0xF7              ], 4, AUTO_SIZE;
]
"mulpd" = [
    b"yowo"       , [0x0F, 0x59        ], X, PREF_66 | SSE2;
]
"mulps" = [
    b"yowo"       , [0x0F, 0x59        ], X, SSE;
]
"mulsd" = [
    b"yoyo"       , [0x0F, 0x59        ], X, PREF_F2 | SSE2;
]
"mulss" = [
    b"yoyo"       , [0x0F, 0x59        ], X, PREF_F3 | SSE;
]
"mwait" = [
    b""           , [0x0F, 0x01, 0xC9  ], X;
    b"AdBd"       , [0x0F, 0x01, 0xC9  ], X;
]
"mwaitx" = [
    b""           , [0x0F, 0x01, 0xFB  ], X, AMD;
    b"AdBd"       , [0x0F, 0x01, 0xFB  ], X, AMD;
]
"neg" = [
    b"mb"         , [0xF6              ], 3, LOCK;
    b"rb"         , [0xF6              ], 3;
    b"m*"         , [0xF7              ], 3, AUTO_SIZE | LOCK;
    b"r*"         , [0xF7              ], 3, AUTO_SIZE | LOCK;
]
"nop" = [
    b""           , [0x90              ], X;
    b"v*"         , [0x0F, 0x1F        ], 0, AUTO_SIZE;
]
"not" = [
    b"mb"         , [0xF6              ], 2, LOCK;
    b"rb"         , [0xF6              ], 2;
    b"m*"         , [0xF7              ], 2, AUTO_SIZE | LOCK;
    b"r*"         , [0xF7              ], 2, AUTO_SIZE | LOCK;
]
"or" = [
    b"Abib"       , [0x0C              ], X;
    b"mbib"       , [0x80              ], 1, LOCK;
    b"mbrb"       , [0x08              ], X, LOCK | ENC_MR;
    b"rbib"       , [0x80              ], 1;
    b"rbrb"       , [0x08              ], X, ENC_MR;
    b"rbvb"       , [0x0A              ], X;
    b"r*ib"       , [0x83              ], 1, AUTO_SIZE | LOCK| EXACT_SIZE;
    b"A*i*"       , [0x0D              ], X, AUTO_SIZE;
    b"m*i*"       , [0x81              ], 1, AUTO_SIZE | LOCK;
    b"m*ib"       , [0x83              ], 1, AUTO_SIZE | LOCK;
    b"m*r*"       , [0x09              ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*i*"       , [0x81              ], 1, AUTO_SIZE | LOCK;
    b"r*r*"       , [0x09              ], X, AUTO_SIZE | ENC_MR;
    b"r*v*"       , [0x0B              ], X, AUTO_SIZE;
]
"orpd" = [
    b"yowo"       , [0x0F, 0x56        ], X, PREF_66 | SSE2;
]
"orps" = [
    b"yowo"       , [0x0F, 0x56        ], X, SSE;
]
"outsb" = [
    b""           , [0x6E              ], X;
]
"outsd" = [
    b""           , [0x6F              ], X;
]
"outsw" = [
    b""           , [0x6F              ], X, WORD_SIZE;
]
"pabsb" = [
    b"xquq"       , [0x0F, 0x38, 0x1C  ], X, SSSE3 | MMX;
    b"yoyo"       , [0x0F, 0x38, 0x1C  ], X, PREF_66 | SSSE3;
]
"pabsd" = [
    b"xquq"       , [0x0F, 0x38, 0x1E  ], X, SSSE3 | MMX;
    b"yoyo"       , [0x0F, 0x38, 0x1E  ], X, PREF_66 | SSSE3;
]
"pabsw" = [
    b"xquq"       , [0x0F, 0x38, 0x1D  ], X, SSSE3 | MMX;
    b"yoyo"       , [0x0F, 0x38, 0x1D  ], X, PREF_66 | SSSE3;
]
"packssdw" = [
    b"xquq"       , [0x0F, 0x6B        ], X, MMX;
    b"yowo"       , [0x0F, 0x6B        ], X, PREF_66 | SSE2;
]
"packsswb" = [
    b"xquq"       , [0x0F, 0x63        ], X, MMX;
    b"yowo"       , [0x0F, 0x63        ], X, PREF_66 | SSE2;
]
"packusdw" = [
    b"yoyo"       , [0x0F, 0x38, 0x2B  ], X, PREF_66 | SSE41;
]
"packuswb" = [
    b"xquq"       , [0x0F, 0x67        ], X, MMX;
    b"yowo"       , [0x0F, 0x67        ], X, PREF_66 | SSE2;
]
"paddb" = [
    b"xquq"       , [0x0F, 0xFC        ], X, MMX;
    b"yowo"       , [0x0F, 0xFC        ], X, PREF_66 | SSE2;
]
"paddd" = [
    b"xquq"       , [0x0F, 0xFE        ], X, MMX;
    b"yowo"       , [0x0F, 0xFE        ], X, PREF_66 | SSE2;
]
"paddq" = [
    b"xquq"       , [0x0F, 0xD4        ], X, MMX;
    b"yowo"       , [0x0F, 0xD4        ], X, PREF_66 | SSE2;
]
"paddsb" = [
    b"xquq"       , [0x0F, 0xEC        ], X, MMX;
    b"yowo"       , [0x0F, 0xEC        ], X, PREF_66 | SSE2;
]
"paddsiw" = [
    b"xquq"       , [0x0F, 0x51        ], X, CYRIX | MMX;
]
"paddsw" = [
    b"xquq"       , [0x0F, 0xED        ], X, MMX;
    b"yowo"       , [0x0F, 0xED        ], X, PREF_66 | SSE2;
]
"paddusb" = [
    b"xquq"       , [0x0F, 0xDC        ], X, MMX;
    b"yowo"       , [0x0F, 0xDC        ], X, PREF_66 | SSE2;
]
"paddusw" = [
    b"xquq"       , [0x0F, 0xDD        ], X, MMX;
    b"yowo"       , [0x0F, 0xDD        ], X, PREF_66 | SSE2;
]
"paddw" = [
    b"xquq"       , [0x0F, 0xFD        ], X, MMX;
    b"yowo"       , [0x0F, 0xFD        ], X, PREF_66 | SSE2;
]
"palignr" = [
    b"xquqib"     , [0x0F, 0x3A, 0x0F  ], X, MMX | SSSE3;
    b"yoyoib"     , [0x0F, 0x3A, 0x0F  ], X, PREF_66 | SSSE3;
]
"pand" = [
    b"xquq"       , [0x0F, 0xDB        ], X, MMX;
    b"yowo"       , [0x0F, 0xDB        ], X, PREF_66 | SSE2;
]
"pandn" = [
    b"xquq"       , [0x0F, 0xDF        ], X, MMX;
    b"yowo"       , [0x0F, 0xDF        ], X, PREF_66 | SSE2;
]
"pause" = [
    b""           , [0x90              ], X, PREF_F3;
]
"paveb" = [
    b"xquq"       , [0x0F, 0x50        ], X, CYRIX | MMX;
]
"pavgb" = [
    b"xquq"       , [0x0F, 0xE0        ], X, MMX;
    b"yowo"       , [0x0F, 0xE0        ], X, PREF_66 | SSE2;
]
"pavgusb" = [
    b"xquq"       , [0x0F, 0x0F, 0xBF  ], X, TDNOW;
]
"pavgw" = [
    b"xquq"       , [0x0F, 0xE3        ], X, MMX;
    b"yowo"       , [0x0F, 0xE3        ], X, PREF_66 | SSE2;
]
"pblendvb" = [
    b"yoyo"       , [0x0F, 0x38, 0x10  ], X, PREF_66 | SSE41;
]
"pblendw" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x0E  ], X, PREF_66 | SSE41;
]
"pclmulhqhqdq" = [
    b"yowo"       , [0x0F, 0x3A, 0x44, 0x11], X, PREF_66 | SSE;
]
"pclmulhqlqdq" = [
    b"yowo"       , [0x0F, 0x3A, 0x44, 0x01], X, PREF_66 | SSE;
]
"pclmullqhqdq" = [
    b"yowo"       , [0x0F, 0x3A, 0x44, 0x10], X, PREF_66 | SSE;
]
"pclmullqlqdq" = [
    b"yowo"       , [0x0F, 0x3A, 0x44, 0x00], X, PREF_66 | SSE;
]
"pclmulqdq" = [
    b"yowoib"     , [0x0F, 0x3A, 0x44  ], X, PREF_66 | SSE;
]
"pcmpeqb" = [
    b"xquq"       , [0x0F, 0x74        ], X, MMX;
    b"yowo"       , [0x0F, 0x74        ], X, PREF_66 | SSE2;
]
"pcmpeqd" = [
    b"xquq"       , [0x0F, 0x76        ], X, MMX;
    b"yowo"       , [0x0F, 0x76        ], X, PREF_66 | SSE2;
]
"pcmpeqq" = [
    b"yoyo"       , [0x0F, 0x38, 0x29  ], X, PREF_66 | SSE41;
]
"pcmpeqw" = [
    b"xquq"       , [0x0F, 0x75        ], X, MMX;
    b"yowo"       , [0x0F, 0x75        ], X, PREF_66 | SSE2;
]
"pcmpestri" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x61  ], X, PREF_66 | SSE42;
]
"pcmpestrm" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x60  ], X, PREF_66 | SSE42;
]
"pcmpgtb" = [
    b"xquq"       , [0x0F, 0x64        ], X, MMX;
    b"yowo"       , [0x0F, 0x64        ], X, PREF_66 | SSE2;
]
"pcmpgtd" = [
    b"xquq"       , [0x0F              ], X, PREF_66 | MMX;
    b"yowo"       , [0x0F, 0x66        ], X, PREF_66 | SSE2;
]
"pcmpgtq" = [
    b"yoyo"       , [0x0F, 0x38, 0x37  ], X, PREF_66 | SSE42;
]
"pcmpgtw" = [
    b"xquq"       , [0x0F, 0x65        ], X, MMX;
    b"yowo"       , [0x0F, 0x65        ], X, PREF_66 | SSE2;
]
"pcmpistri" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x63  ], X, PREF_66 | SSE42;
]
"pcmpistrm" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x62  ], X, PREF_66 | SSE42;
]
"pdistib" = [
    b"xqm!"       , [0x0F, 0x54        ], X, MMX | CYRIX;
]
"pextrb" = [
    b"mbyoib"     , [0x0F, 0x3A, 0x14  ], X, WITH_REXW | ENC_MR | PREF_66 | SSE41;
    b"rdyoib"     , [0x0F, 0x3A, 0x14  ], X, WITH_REXW | ENC_MR | PREF_66 | SSE41;
    b"rqyoib"     , [0x0F, 0x3A, 0x14  ], X, WITH_REXW | ENC_MR | PREF_66 | SSE41;
]
"pextrd" = [
    b"vdyoib"     , [0x0F, 0x3A, 0x16  ], X, ENC_MR | PREF_66 | SSE41;
]
"pextrq" = [
    b"vqyoib"     , [0x0F, 0x3A, 0x16  ], X, WITH_REXW | ENC_MR | PREF_66 | SSE41;
]
"pextrw" = [
    b"mwyoib"     , [0x0F, 0x3A, 0x15  ], X, WITH_REXW | ENC_MR | PREF_66 | SSE41;
    b"rdxqib"     , [0x0F, 0xC5        ], X, MMX;
    b"rdyoib"     , [0x0F, 0x3A, 0x15  ], X, WITH_REXW | ENC_MR | PREF_66 | SSE41;
    b"rdyoib"     , [0x0F, 0xC5        ], X, PREF_66 | SSE2;
    b"rqyoib"     , [0x0F, 0x3A, 0x15  ], X, WITH_REXW | ENC_MR | PREF_66 | SSE41;
]
"pf2id" = [
    b"xquq"       , [0x0F, 0x0F, 0x1D  ], X, TDNOW;
]
"pf2iw" = [
    b"xquq"       , [0x0F, 0x0F, 0x1C  ], X, TDNOW;
]
"pfacc" = [
    b"xquq"       , [0x0F, 0x0F, 0xAE  ], X, TDNOW;
]
"pfadd" = [
    b"xquq"       , [0x0F, 0x0F, 0x9E  ], X, TDNOW;
]
"pfcmpeq" = [
    b"xquq"       , [0x0F, 0x0F, 0xB0  ], X, TDNOW;
]
"pfcmpge" = [
    b"xquq"       , [0x0F, 0x0F, 0x90  ], X, TDNOW;
]
"pfcmpgt" = [
    b"xquq"       , [0x0F, 0x0F, 0xA0  ], X, TDNOW;
]
"pfmax" = [
    b"xquq"       , [0x0F, 0x0F, 0xA4  ], X, TDNOW;
]
"pfmin" = [
    b"xquq"       , [0x0F, 0x0F, 0x94  ], X, TDNOW;
]
"pfmul" = [
    b"xquq"       , [0x0F, 0x0F, 0xB4  ], X, TDNOW;
]
"pfnacc" = [
    b"xquq"       , [0x0F, 0x0F, 0x8A  ], X, TDNOW;
]
"pfpnacc" = [
    b"xquq"       , [0x0F, 0x0F, 0x8E  ], X, TDNOW;
]
"pfrcp" = [
    b"xquq"       , [0x0F, 0x0F, 0x96  ], X, TDNOW;
]
"pfrcpit1" = [
    b"xquq"       , [0x0F, 0x0F, 0xA6  ], X, TDNOW;
]
"pfrcpit2" = [
    b"xquq"       , [0x0F, 0x0F, 0xB6  ], X, TDNOW;
]
"pfrcpv" = [
    b"xquq"       , [0x0F, 0x0F, 0x86  ], X, TDNOW | CYRIX;
]
"pfrsqit1" = [
    b"xquq"       , [0x0F, 0x0F, 0xA7  ], X, TDNOW;
]
"pfrsqrt" = [
    b"xquq"       , [0x0F, 0x0F, 0x97  ], X, TDNOW;
]
"pfrsqrtv" = [
    b"xquq"       , [0x0F, 0x0F, 0x87  ], X, CYRIX | TDNOW;
]
"pfsub" = [
    b"xquq"       , [0x0F, 0x0F, 0x9A  ], X, TDNOW;
]
"pfsubr" = [
    b"xquq"       , [0x0F, 0x0F, 0xAA  ], X, TDNOW;
]
"phaddd" = [
    b"xquq"       , [0x0F, 0x38, 0x02  ], X, SSSE3 | MMX;
    b"yoyo"       , [0x0F, 0x38, 0x02  ], X, PREF_66 | SSSE3;
]
"phaddsw" = [
    b"xquq"       , [0x0F, 0x38, 0x03  ], X, MMX | SSSE3;
    b"yoyo"       , [0x0F, 0x38, 0x03  ], X, PREF_66 | SSSE3;
]
"phaddw" = [
    b"xquq"       , [0x0F, 0x38, 0x01  ], X, SSSE3 | MMX;
    b"yoyo"       , [0x0F, 0x38, 0x01  ], X, PREF_66 | SSSE3;
]
"phminposuw" = [
    b"yoyo"       , [0x0F, 0x38, 0x41  ], X, PREF_66 | SSE41;
]
"phsubd" = [
    b"xquq"       , [0x0F, 0x38, 0x06  ], X, SSSE3 | MMX;
    b"yoyo"       , [0x0F, 0x38, 0x06  ], X, PREF_66 | SSSE3;
]
"phsubsw" = [
    b"xquq"       , [0x0F, 0x38, 0x07  ], X, MMX | SSSE3;
    b"yoyo"       , [0x0F, 0x38, 0x07  ], X, PREF_66 | SSSE3;
]
"phsubw" = [
    b"xquq"       , [0x0F, 0x38, 0x05  ], X, SSSE3 | MMX;
    b"yoyo"       , [0x0F, 0x38, 0x05  ], X, PREF_66 | SSSE3;
]
"pi2fd" = [
    b"xquq"       , [0x0F, 0x0F, 0x0D  ], X, TDNOW;
]
"pi2fw" = [
    b"xquq"       , [0x0F, 0x0F, 0x0C  ], X, TDNOW;
]
"pinsrb" = [
    b"yom!ib"     , [0x0F, 0x3A, 0x20  ], X, PREF_66 | SSE41;
    b"yordib"     , [0x0F, 0x3A, 0x20  ], X, PREF_66 | SSE41;
    b"yovbib"     , [0x0F, 0x3A, 0x20  ], X, PREF_66 | SSE41;
]
"pinsrd" = [
    b"yom!ib"     , [0x0F, 0x3A, 0x22  ], X, PREF_66 | SSE41;
]
"pinsrq" = [
    b"yom!ib"     , [0x0F, 0x3A, 0x22  ], X, WITH_REXW | PREF_66 | SSE41;
]
"pinsrw" = [
    b"xqm!ib"     , [0x0F, 0xC4        ], X, MMX;
    b"xqrdib"     , [0x0F, 0xC4        ], X, MMX;
    b"xqvwib"     , [0x0F, 0xC4        ], X, MMX;
    b"yom!ib"     , [0x0F, 0xC4        ], X, PREF_66 | SSE2;
    b"yomwib"     , [0x0F, 0xC4        ], X, PREF_66 | SSE2;
    b"yordib"     , [0x0F, 0xC4        ], X, PREF_66 | SSE2;
    b"yorwib"     , [0x0F, 0xC4        ], X, PREF_66 | SSE2;
]
"pmachriw" = [
    b"xqm!"       , [0x0F, 0x5E        ], X, MMX | CYRIX;
]
"pmaddubsw" = [
    b"xquq"       , [0x0F, 0x38, 0x04  ], X, MMX | SSSE3;
    b"yoyo"       , [0x0F, 0x38, 0x04  ], X, PREF_66 | SSSE3;
]
"pmaddwd" = [
    b"xquq"       , [0x0F, 0xF5        ], X, MMX;
    b"yowo"       , [0x0F, 0xF5        ], X, PREF_66 | SSE2;
]
"pmagw" = [
    b"xquq"       , [0x0F, 0x52        ], X, MMX | CYRIX;
]
"pmaxsb" = [
    b"yoyo"       , [0x0F, 0x38, 0x3C  ], X, PREF_66 | SSE41;
]
"pmaxsd" = [
    b"yoyo"       , [0x0F, 0x38, 0x3D  ], X, PREF_66 | SSE41;
]
"pmaxsw" = [
    b"xquq"       , [0x0F, 0xEE        ], X, MMX;
    b"yowo"       , [0x0F, 0xEE        ], X, PREF_66 | SSE2;
]
"pmaxub" = [
    b"xquq"       , [0x0F, 0xDE        ], X, MMX;
    b"yowo"       , [0x0F, 0xDE        ], X, PREF_66 | SSE2;
]
"pmaxud" = [
    b"yoyo"       , [0x0F, 0x38, 0x3F  ], X, PREF_66 | SSE41;
]
"pmaxuw" = [
    b"yoyo"       , [0x0F, 0x38, 0x3E  ], X, PREF_66 | SSE41;
]
"pminsb" = [
    b"yoyo"       , [0x0F, 0x38, 0x38  ], X, PREF_66 | SSE41;
]
"pminsd" = [
    b"yoyo"       , [0x0F, 0x38, 0x39  ], X, PREF_66 | SSE41;
]
"pminsw" = [
    b"xquq"       , [0x0F, 0xEA        ], X, MMX;
    b"yowo"       , [0x0F, 0xEA        ], X, PREF_66 | SSE2;
]
"pminub" = [
    b"xquq"       , [0x0F, 0xDA        ], X, MMX;
    b"yowo"       , [0x0F, 0xDA        ], X, PREF_66 | SSE2;
]
"pminud" = [
    b"yoyo"       , [0x0F, 0x38, 0x3B  ], X, PREF_66 | SSE41;
]
"pminuw" = [
    b"yoyo"       , [0x0F, 0x38, 0x3A  ], X, PREF_66 | SSE41;
]
"pmovmskb" = [
    b"rdxq"       , [0x0F, 0xD7        ], X, MMX;
    b"rdyo"       , [0x0F, 0xD7        ], X, PREF_66 | SSE2;
]
"pmovsxbd" = [
    b"yoyo"       , [0x0F, 0x38, 0x21  ], X, PREF_66 | SSE41;
]
"pmovsxbq" = [
    b"yoyo"       , [0x0F, 0x38, 0x22  ], X, PREF_66 | SSE41;
]
"pmovsxbw" = [
    b"yoyo"       , [0x0F, 0x38, 0x20  ], X, PREF_66 | SSE41;
]
"pmovsxdq" = [
    b"yoyo"       , [0x0F, 0x38, 0x25  ], X, PREF_66 | SSE41;
]
"pmovsxwd" = [
    b"yoyo"       , [0x0F, 0x38, 0x23  ], X, PREF_66 | SSE41;
]
"pmovsxwq" = [
    b"yoyo"       , [0x0F, 0x38, 0x24  ], X, PREF_66 | SSE41;
]
"pmovzxbd" = [
    b"yoyo"       , [0x0F, 0x38, 0x31  ], X, PREF_66 | SSE41;
]
"pmovzxbq" = [
    b"yoyo"       , [0x0F, 0x38, 0x32  ], X, PREF_66 | SSE41;
]
"pmovzxbw" = [
    b"yoyo"       , [0x0F, 0x38, 0x30  ], X, PREF_66 | SSE41;
]
"pmovzxdq" = [
    b"yoyo"       , [0x0F, 0x38, 0x35  ], X, PREF_66 | SSE41;
]
"pmovzxwd" = [
    b"yoyo"       , [0x0F, 0x38, 0x33  ], X, PREF_66 | SSE41;
]
"pmovzxwq" = [
    b"yoyo"       , [0x0F, 0x38, 0x34  ], X, PREF_66 | SSE41;
]
"pmuldq" = [
    b"yoyo"       , [0x0F, 0x38, 0x28  ], X, PREF_66 | SSE41;
]
"pmulhriw" = [
    b"xquq"       , [0x0F, 0x5D        ], X, MMX | CYRIX;
]
"pmulhrsw" = [
    b"xquq"       , [0x0F, 0x38, 0x0B  ], X, MMX | SSSE3;
    b"yoyo"       , [0x0F, 0x38, 0x0B  ], X, PREF_66 | SSSE3;
]
"pmulhrwa" = [
    b"xquq"       , [0x0F, 0x0F, 0xB7  ], X, TDNOW;
]
"pmulhrwc" = [
    b"xquq"       , [0x0F, 0x59        ], X, MMX | CYRIX;
]
"pmulhuw" = [
    b"xquq"       , [0x0F, 0xE4        ], X, MMX;
    b"yowo"       , [0x0F, 0xE4        ], X, PREF_66 | SSE2;
]
"pmulhw" = [
    b"xquq"       , [0x0F, 0xE5        ], X, MMX;
    b"yowo"       , [0x0F, 0xE5        ], X, PREF_66 | SSE2;
]
"pmulld" = [
    b"yoyo"       , [0x0F, 0x38, 0x40  ], X, PREF_66 | SSE41;
]
"pmullw" = [
    b"xquq"       , [0x0F, 0xD5        ], X, MMX;
    b"yowo"       , [0x0F, 0xD5        ], X, PREF_66 | SSE2;
]
"pmuludq" = [
    b"xquq"       , [0x0F, 0xF4        ], X, SSE2;
    b"yowo"       , [0x0F, 0xF4        ], X, PREF_66 | SSE2;
]
"pmvgezb" = [
    b"xqmq"       , [0x0F, 0x5C        ], X, MMX | CYRIX;
]
"pmvlzb" = [
    b"xqmq"       , [0x0F, 0x5B        ], X, CYRIX | MMX;
]
"pmvnzb" = [
    b"xqmq"       , [0x0F, 0x5A        ], X, MMX | CYRIX;
]
"pmvzb" = [
    b"xqmq"       , [0x0F, 0x58        ], X, MMX | CYRIX;
]
"pop" = [
    b"Uw"         , [0x0F, 0xA1        ], X;
    b"Vw"         , [0x0F, 0xA9        ], X;
    b"r*"         , [0x58              ], X, AUTO_NO32 | SHORT_ARG;
    b"v*"         , [0x8F              ], 0, AUTO_NO32;
]
"popcnt" = [
    b"r*v*"       , [0x0F, 0xB8        ], X, AUTO_SIZE | PREF_F3;
]
"popf" = [
    b""           , [0x9D              ], X;
]
"popfq" = [
    b""           , [0x9D              ], X;
]
"popfw" = [
    b""           , [0x9D              ], X, WORD_SIZE;
]
"por" = [
    b"xquq"       , [0x0F, 0xEB        ], X, MMX;
    b"yowo"       , [0x0F, 0xEB        ], X, PREF_66 | SSE2;
]
"prefetch" = [
    b"mq"         , [0x0F, 0x0D        ], 0, TDNOW;
]
"prefetchnta" = [
    b"mb"         , [0x0F, 0x18        ], 0;
]
"prefetcht0" = [
    b"mb"         , [0x0F, 0x18        ], 1;
]
"prefetcht1" = [
    b"mb"         , [0x0F, 0x18        ], 2;
]
"prefetcht2" = [
    b"mb"         , [0x0F, 0x18        ], 3;
]
"prefetchw" = [
    b"mq"         , [0x0F, 0x0D        ], 1, TDNOW;
]
"prefetchwt1" = [
    b"mb"         , [0x0F, 0x0D        ], 2, PREFETCHWT1;
]
"psadbw" = [
    b"xquq"       , [0x0F, 0xF6        ], X, MMX;
    b"yowo"       , [0x0F, 0xF6        ], X, PREF_66 | SSE2;
]
"pshufb" = [
    b"xquq"       , [0x0F, 0x38, 0x00  ], X, SSSE3 | MMX;
    b"yoyo"       , [0x0F, 0x38, 0x00  ], X, PREF_66 | SSSE3;
]
"pshufd" = [
    b"yoyoib"     , [0x0F, 0x70        ], X, PREF_66 | SSE2;
]
"pshufhw" = [
    b"yoyoib"     , [0x0F, 0x70        ], X, PREF_F3 | SSE2;
]
"pshuflw" = [
    b"yoyoib"     , [0x0F, 0x70        ], X, PREF_F2 | SSE2;
]
"pshufw" = [
    b"xquqib"     , [0x0F, 0x70        ], X, MMX;
]
"psignb" = [
    b"xquq"       , [0x0F, 0x38, 0x08  ], X, MMX | SSSE3;
    b"yoyo"       , [0x0F, 0x38, 0x08  ], X, PREF_66 | SSSE3;
]
"psignd" = [
    b"xquq"       , [0x0F, 0x38, 0x0A  ], X, MMX | SSSE3;
    b"yoyo"       , [0x0F, 0x38, 0x0A  ], X, PREF_66 | SSSE3;
]
"psignw" = [
    b"xquq"       , [0x0F, 0x38, 0x09  ], X, SSSE3 | MMX;
    b"yoyo"       , [0x0F, 0x38, 0x09  ], X, PREF_66 | SSSE3;
]
"pslld" = [
    b"xqib"       , [0x0F, 0x72        ], 6, MMX;
    b"xquq"       , [0x0F              ], X, PREF_F2 | MMX;
    b"yoib"       , [0x0F, 0x72        ], 6, PREF_66 | SSE2;
    b"yowo"       , [0x0F, 0xF2        ], X, PREF_66 | SSE2;
]
"pslldq" = [
    b"yoib"       , [0x0F, 0x73        ], 7, PREF_66 | SSE2;
]
"psllq" = [
    b"xqib"       , [0x0F, 0x73        ], 6, MMX;
    b"xquq"       , [0x0F              ], X, PREF_F3 | MMX;
    b"yoib"       , [0x0F, 0x73        ], 6, PREF_66 | SSE2;
    b"yowo"       , [0x0F, 0xF3        ], X, PREF_66 | SSE2;
]
"psllw" = [
    b"xqib"       , [0x0F, 0x71        ], 6, MMX;
    b"xquq"       , [0x0F, 0xF1        ], X, MMX;
    b"yoib"       , [0x0F, 0x71        ], 6, PREF_66 | SSE2;
    b"yowo"       , [0x0F, 0xF1        ], X, PREF_66 | SSE2;
]
"psrad" = [
    b"xqib"       , [0x0F, 0x72        ], 4, MMX;
    b"xquq"       , [0x0F, 0xE2        ], X, MMX;
    b"yoib"       , [0x0F, 0x72        ], 4, PREF_66 | SSE2;
    b"yowo"       , [0x0F, 0xE2        ], X, PREF_66 | SSE2;
]
"psraw" = [
    b"xqib"       , [0x0F, 0x71        ], 4, MMX;
    b"xquq"       , [0x0F, 0xE1        ], X, MMX;
    b"yoib"       , [0x0F, 0x71        ], 4, PREF_66 | SSE2;
    b"yowo"       , [0x0F, 0xE1        ], X, PREF_66 | SSE2;
]
"psrld" = [
    b"xqib"       , [0x0F, 0x72        ], 2, MMX;
    b"xquq"       , [0x0F, 0xD2        ], X, MMX;
    b"yoib"       , [0x0F, 0x72        ], 2, PREF_66 | SSE2;
    b"yowo"       , [0x0F, 0xD2        ], X, PREF_66 | SSE2;
]
"psrldq" = [
    b"yoib"       , [0x0F, 0x73        ], 3, PREF_66 | SSE2;
]
"psrlq" = [
    b"xqib"       , [0x0F, 0x73        ], 2, MMX;
    b"xquq"       , [0x0F, 0xD3        ], X, MMX;
    b"yoib"       , [0x0F, 0x73        ], 2, PREF_66 | SSE2;
    b"yowo"       , [0x0F, 0xD3        ], X, PREF_66 | SSE2;
]
"psrlw" = [
    b"xqib"       , [0x0F, 0x71        ], 2, MMX;
    b"xquq"       , [0x0F, 0xD1        ], X, MMX;
    b"yoib"       , [0x0F, 0x71        ], 2, PREF_66 | SSE2;
    b"yowo"       , [0x0F, 0xD1        ], X, PREF_66 | SSE2;
]
"psubb" = [
    b"xquq"       , [0x0F, 0xF8        ], X, MMX;
    b"yowo"       , [0x0F, 0xF8        ], X, PREF_66 | SSE2;
]
"psubd" = [
    b"xquq"       , [0x0F, 0xFA        ], X, MMX;
    b"yowo"       , [0x0F, 0xFA        ], X, PREF_66 | SSE2;
]
"psubq" = [
    b"xquq"       , [0x0F, 0xFB        ], X, SSE2;
    b"yowo"       , [0x0F, 0xFB        ], X, PREF_66 | SSE2;
]
"psubsb" = [
    b"xquq"       , [0x0F, 0xE8        ], X, MMX;
    b"yowo"       , [0x0F, 0xE8        ], X, PREF_66 | SSE2;
]
"psubsiw" = [
    b"xquq"       , [0x0F, 0x55        ], X, CYRIX | MMX;
]
"psubsw" = [
    b"xquq"       , [0x0F, 0xE9        ], X, MMX;
    b"yowo"       , [0x0F, 0xE9        ], X, PREF_66 | SSE2;
]
"psubusb" = [
    b"xquq"       , [0x0F, 0xD8        ], X, MMX;
    b"yowo"       , [0x0F, 0xD8        ], X, PREF_66 | SSE2;
]
"psubusw" = [
    b"xquq"       , [0x0F, 0xD9        ], X, MMX;
    b"yowo"       , [0x0F, 0xD9        ], X, PREF_66 | SSE2;
]
"psubw" = [
    b"xquq"       , [0x0F, 0xF9        ], X, MMX;
    b"yowo"       , [0x0F, 0xF9        ], X, PREF_66 | SSE2;
]
"pswapd" = [
    b"xquq"       , [0x0F, 0x0F, 0xBB  ], X, TDNOW;
]
"ptest" = [
    b"yoyo"       , [0x0F, 0x38, 0x17  ], X, PREF_66 | SSE41;
]
"punpckhbw" = [
    b"xquq"       , [0x0F, 0x68        ], X, MMX;
    b"yowo"       , [0x0F, 0x68        ], X, PREF_66 | SSE2;
]
"punpckhdq" = [
    b"xquq"       , [0x0F, 0x6A        ], X, MMX;
    b"yowo"       , [0x0F, 0x6A        ], X, PREF_66 | SSE2;
]
"punpckhqdq" = [
    b"yowo"       , [0x0F, 0x6D        ], X, PREF_66 | SSE2;
]
"punpckhwd" = [
    b"xquq"       , [0x0F, 0x69        ], X, MMX;
    b"yowo"       , [0x0F, 0x69        ], X, PREF_66 | SSE2;
]
"punpcklbw" = [
    b"xquq"       , [0x0F, 0x60        ], X, MMX;
    b"yowo"       , [0x0F, 0x60        ], X, PREF_66 | SSE2;
]
"punpckldq" = [
    b"xquq"       , [0x0F, 0x62        ], X, MMX;
    b"yowo"       , [0x0F, 0x62        ], X, PREF_66 | SSE2;
]
"punpcklqdq" = [
    b"yowo"       , [0x0F, 0x6C        ], X, PREF_66 | SSE2;
]
"punpcklwd" = [
    b"xquq"       , [0x0F, 0x61        ], X, MMX;
    b"yowo"       , [0x0F, 0x61        ], X, PREF_66 | SSE2;
]
"push" = [
    b"Uw"         , [0x0F, 0xA0        ], X;
    b"Vw"         , [0x0F, 0xA8        ], X;
    b"ib"         , [0x6A              ], X;
    b"i*"         , [0x68              ], X, AUTO_NO32;
    b"r*"         , [0x50              ], X, AUTO_NO32 | SHORT_ARG;
    b"v*"         , [0xFF              ], 6, AUTO_NO32;
]
"pushf" = [
    b""           , [0x9C              ], X;
]
"pushfq" = [
    b""           , [0x9C              ], X;
]
"pushfw" = [
    b""           , [0x9C              ], X, WORD_SIZE;
]
"pxor" = [
    b"xquq"       , [0x0F, 0xEF        ], X, MMX;
    b"yowo"       , [0x0F, 0xEF        ], X, PREF_66 | SSE2;
]
"rcl" = [
    b"vbBb"       , [0xD2              ], 2;
    b"vbib"       , [0xC0              ], 2;
    b"v*Bb"       , [0xD3              ], 2, AUTO_SIZE;
    b"v*ib"       , [0xC1              ], 2, AUTO_SIZE;
]
"rcpps" = [
    b"yowo"       , [0x0F, 0x53        ], X, SSE;
]
"rcpss" = [
    b"yoyo"       , [0x0F, 0x53        ], X, PREF_F3 | SSE;
]
"rcr" = [
    b"vbBb"       , [0xD2              ], 3;
    b"vbib"       , [0xC0              ], 3;
    b"v*Bb"       , [0xD3              ], 3, AUTO_SIZE;
    b"v*ib"       , [0xC1              ], 3, AUTO_SIZE;
]
"rdfsbase" = [
    b"r*"         , [0x0F, 0xAE        ], 0, AUTO_REXW | PREF_F3;
]
"rdgsbase" = [
    b"r*"         , [0x0F, 0xAE        ], 1, AUTO_REXW | PREF_F3;
]
"rdm" = [
    b""           , [0x0F, 0x3A        ], X, CYRIX;
]
"rdmsr" = [
    b""           , [0x0F, 0x32        ], X;
]
"rdpid" = [
    b"rq"         , [0x0F, 0xC7        ], 7, PREF_F3;
]
"rdpkru" = [
    b""           , [0x0F, 0x01, 0xEE  ], X;
]
"rdpmc" = [
    b""           , [0x0F, 0x33        ], X;
]
"rdrand" = [
    b"rq"         , [0x0F, 0xC7        ], 6, WITH_REXW;
]
"rdseed" = [
    b"rq"         , [0x0F, 0xC7        ], 7, WITH_REXW;
]
"rdshr" = [
    b"vd"         , [0x0F, 0x36        ], 0, CYRIX;
]
"rdtsc" = [
    b""           , [0x0F, 0x31        ], X;
]
"rdtscp" = [
    b""           , [0x0F, 0x01, 0xF9  ], X;
]
"ret" = [
    b""           , [0xC3              ], X;
    b"iw"         , [0xC2              ], X;
]
"retf" = [
    b""           , [0xCB              ], X;
    b"iw"         , [0xCA              ], X;
]
"retn" = [
    b""           , [0xC3              ], X;
    b"iw"         , [0xC2              ], X;
]
"rol" = [
    b"vbBb"       , [0xD2              ], 0;
    b"vbib"       , [0xC0              ], 0;
    b"v*Bb"       , [0xD3              ], 0, AUTO_SIZE;
    b"v*ib"       , [0xC1              ], 0, AUTO_SIZE;
]
"ror" = [
    b"vbBb"       , [0xD2              ], 1;
    b"vbib"       , [0xC0              ], 1;
    b"v*Bb"       , [0xD3              ], 1, AUTO_SIZE;
    b"v*ib"       , [0xC1              ], 1, AUTO_SIZE;
]
"roundpd" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x09  ], X, PREF_66 | SSE41;
]
"roundps" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x08  ], X, PREF_66 | SSE41;
]
"roundsd" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x0B  ], X, PREF_66 | SSE41;
]
"roundss" = [
    b"yoyoib"     , [0x0F, 0x3A, 0x0A  ], X, PREF_66 | SSE41;
]
"rsdc" = [
    b"swmp"       , [0x0F, 0x79        ], X, CYRIX;
]
"rsldt" = [
    b"mp"         , [0x0F, 0x7B        ], 0, CYRIX;
]
"rsm" = [
    b""           , [0x0F, 0xAA        ], X;
]
"rsqrtps" = [
    b"yowo"       , [0x0F, 0x52        ], X, SSE;
]
"rsqrtss" = [
    b"yoyo"       , [0x0F, 0x52        ], X, PREF_F3 | SSE;
]
"rsts" = [
    b"mp"         , [0x0F, 0x7D        ], 0, CYRIX;
]
"sahf" = [
    b""           , [0x9E              ], X;
]
"sal" = [
    b"vbBb"       , [0xD2              ], 4;
    b"vbib"       , [0xC0              ], 4;
    b"v*Bb"       , [0xD3              ], 4, AUTO_SIZE;
    b"v*ib"       , [0xC1              ], 4, AUTO_SIZE;
]
"sar" = [
    b"vbBb"       , [0xD2              ], 7;
    b"vbib"       , [0xC0              ], 7;
    b"v*Bb"       , [0xD3              ], 7, AUTO_SIZE;
    b"v*ib"       , [0xC1              ], 7, AUTO_SIZE;
]
"sbb" = [
    b"Abib"       , [0x1C              ], X;
    b"mbib"       , [0x80              ], 3, LOCK;
    b"mbrb"       , [0x18              ], X, LOCK | ENC_MR;
    b"rbib"       , [0x80              ], 3;
    b"rbrb"       , [0x18              ], X, ENC_MR;
    b"rbvb"       , [0x1A              ], X;
    b"r*ib"       , [0x83              ], 3, AUTO_SIZE | LOCK| EXACT_SIZE;
    b"A*i*"       , [0x1D              ], X, AUTO_SIZE;
    b"m*i*"       , [0x81              ], 3, AUTO_SIZE | LOCK;
    b"m*ib"       , [0x83              ], 3, AUTO_SIZE | LOCK;
    b"m*r*"       , [0x19              ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*i*"       , [0x81              ], 3, AUTO_SIZE | LOCK;
    b"r*r*"       , [0x19              ], X, AUTO_SIZE | ENC_MR;
    b"r*v*"       , [0x1B              ], X, AUTO_SIZE;
]
"scasb" = [
    b""           , [0xAE              ], X, REPE;
]
"scasd" = [
    b""           , [0xAF              ], X, REPE;
]
"scasq" = [
    b""           , [0xAF              ], X, REPE | WITH_REXW;
]
"scasw" = [
    b""           , [0xAF              ], X, REPE | WORD_SIZE;
]
"sfence" = [
    b""           , [0x0F, 0xAE, 0xF8  ], X, AMD;
]
"sgdt" = [
    b"m!"         , [0x0F, 0x01        ], 0;
]
"sha1msg1" = [
    b"yowo"       , [0x0F, 0x38, 0xC9  ], X, SHA;
]
"sha1msg2" = [
    b"yowo"       , [0x0F, 0x38, 0xCA  ], X, SHA;
]
"sha1nexte" = [
    b"yowo"       , [0x0F, 0x38, 0xC8  ], X, SHA;
]
"sha1rnds4" = [
    b"yowoib"     , [0x0F, 0x3A, 0xCC  ], X, SHA;
]
"sha256msg1" = [
    b"yowo"       , [0x0F, 0x38, 0xCC  ], X, SHA;
]
"sha256msg2" = [
    b"yowo"       , [0x0F, 0x38, 0xCD  ], X, SHA;
]
"sha256rnds2" = [
    b"yowo"       , [0x0F, 0x38, 0xCB  ], X, SHA;
]
"shl" = [
    b"vbBb"       , [0xD2              ], 4;
    b"vbib"       , [0xC0              ], 4;
    b"v*Bb"       , [0xD3              ], 4, AUTO_SIZE;
    b"v*ib"       , [0xC1              ], 4, AUTO_SIZE;
]
"shld" = [
    b"v*r*Bb"     , [0x0F, 0xA5        ], X, AUTO_SIZE | ENC_MR;
    b"v*r*ib"     , [0x0F, 0xA4        ], X, AUTO_SIZE | ENC_MR;
]
"shr" = [
    b"vbBb"       , [0xD2              ], 5;
    b"vbib"       , [0xC0              ], 5;
    b"v*Bb"       , [0xD3              ], 5, AUTO_SIZE;
    b"v*ib"       , [0xC1              ], 5, AUTO_SIZE;
]
"shrd" = [
    b"v*r*Bb"     , [0x0F, 0xAD        ], X, AUTO_SIZE | ENC_MR;
    b"v*r*ib"     , [0x0F, 0xAC        ], X, AUTO_SIZE | ENC_MR;
]
"shufpd" = [
    b"yoyoib"     , [0x0F, 0xC6        ], X, PREF_66 | SSE2;
]
"shufps" = [
    b"yowoib"     , [0x0F, 0xC6        ], X, SSE;
]
"sidt" = [
    b"m!"         , [0x0F, 0x01        ], 1;
]
"skinit" = [
    b""           , [0x0F, 0x01, 0xDE  ], X;
]
"sldt" = [
    b"m!"         , [0x0F, 0x00        ], 0;
    b"r*"         , [0x0F, 0x00        ], 0, AUTO_SIZE;
]
"smint" = [
    b""           , [0x0F, 0x38        ], X, CYRIX;
]
"smsw" = [
    b"m!"         , [0x0F, 0x01        ], 4;
    b"r*"         , [0x0F, 0x01        ], 4, AUTO_SIZE;
]
"sqrtpd" = [
    b"yowo"       , [0x0F, 0x51        ], X, PREF_66 | SSE2;
]
"sqrtps" = [
    b"yowo"       , [0x0F, 0x51        ], X, SSE;
]
"sqrtsd" = [
    b"yoyo"       , [0x0F, 0x51        ], X, PREF_F2 | SSE2;
]
"sqrtss" = [
    b"yoyo"       , [0x0F, 0x51        ], X, PREF_F3 | SSE;
]
"stac" = [
    b""           , [0x0F, 0x01, 0xCB  ], X;
]
"stc" = [
    b""           , [0xF9              ], X;
]
"std" = [
    b""           , [0xFD              ], X;
]
"stgi" = [
    b""           , [0x0F, 0x01, 0xDC  ], X, AMD | VMX;
]
"sti" = [
    b""           , [0xFB              ], X;
]
"stmxcsr" = [
    b"md"         , [0x0F, 0xAE        ], 3, SSE;
]
"stosb" = [
    b""           , [0xAA              ], X;
]
"stosd" = [
    b""           , [0xAB              ], X;
]
"stosq" = [
    b""           , [0xAB              ], X, WITH_REXW;
]
"stosw" = [
    b""           , [0xAB              ], X, WORD_SIZE;
]
"str" = [
    b"m!"         , [0x0F, 0x00        ], 1;
    b"r*"         , [0x0F, 0x00        ], 1, AUTO_SIZE;
]
"sub" = [
    b"Abib"       , [0x2C              ], X;
    b"mbib"       , [0x80              ], 5, LOCK;
    b"mbrb"       , [0x28              ], X, LOCK | ENC_MR;
    b"rbib"       , [0x80              ], 5;
    b"rbrb"       , [0x28              ], X, ENC_MR;
    b"rbvb"       , [0x2A              ], X;
    b"r*ib"       , [0x83              ], 5, AUTO_SIZE | LOCK| EXACT_SIZE;
    b"A*i*"       , [0x2D              ], X, AUTO_SIZE;
    b"m*i*"       , [0x81              ], 5, AUTO_SIZE | LOCK;
    b"m*ib"       , [0x83              ], 5, AUTO_SIZE | LOCK;
    b"m*r*"       , [0x29              ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*i*"       , [0x81              ], 5, AUTO_SIZE | LOCK;
    b"r*r*"       , [0x29              ], X, AUTO_SIZE | ENC_MR;
    b"r*v*"       , [0x2B              ], X, AUTO_SIZE;
]
"subpd" = [
    b"yowo"       , [0x0F, 0x5C        ], X, PREF_66 | SSE2;
]
"subps" = [
    b"yowo"       , [0x0F, 0x5C        ], X, SSE;
]
"subsd" = [
    b"yoyo"       , [0x0F, 0x5C        ], X, PREF_F2 | SSE2;
]
"subss" = [
    b"yoyo"       , [0x0F, 0x5C        ], X, PREF_F3 | SSE;
]
"svdc" = [
    b"mpsw"       , [0x0F, 0x78        ], X, ENC_MR | CYRIX;
]
"svldt" = [
    b"mp"         , [0x0F, 0x7A        ], 0, CYRIX;
]
"svts" = [
    b"mp"         , [0x0F, 0x7C        ], 0, CYRIX;
]
"swapgs" = [
    b""           , [0x0F, 0x01, 0xF8  ], X;
]
"syscall" = [
    b""           , [0x0F, 0x05        ], X, AMD;
]
"sysenter" = [
    b""           , [0x0F, 0x34        ], X;
]
"sysexit" = [
    b""           , [0x0F, 0x35        ], X;
]
"sysret" = [
    b""           , [0x0F, 0x07        ], X, AMD;
]
"test" = [
    b"Abib"       , [0xA8              ], X;
    b"rbmb"       , [0x84              ], X;
    b"vbib"       , [0xF6              ], 0;
    b"vbrb"       , [0x84              ], X, ENC_MR;
    b"A*i*"       , [0xA9              ], X, AUTO_SIZE;
    b"r*m*"       , [0x85              ], X, AUTO_SIZE;
    b"v*i*"       , [0xF7              ], 0, AUTO_SIZE;
    b"v*r*"       , [0x85              ], X, AUTO_SIZE | ENC_MR;
]
"tzcnt" = [
    b"r*v*"       , [0x0F, 0xBC        ], X, AUTO_SIZE | PREF_F3 | BMI1;
]
"ucomisd" = [
    b"yoyo"       , [0x0F, 0x2E        ], X, PREF_66 | SSE2;
]
"ucomiss" = [
    b"yoyo"       , [0x0F, 0x2E        ], X, SSE;
]
"ud2" = [
    b""           , [0x0F, 0x0B        ], X;
]
"ud2a" = [
    b""           , [0x0F, 0x0B        ], X;
]
"unpckhpd" = [
    b"yowo"       , [0x0F, 0x15        ], X, PREF_66 | SSE2;
]
"unpckhps" = [
    b"yowo"       , [0x0F, 0x15        ], X, SSE;
]
"unpcklpd" = [
    b"yowo"       , [0x0F, 0x14        ], X, PREF_66 | SSE2;
]
"unpcklps" = [
    b"yowo"       , [0x0F, 0x14        ], X, SSE;
]
"verr" = [
    b"m!"         , [0x0F, 0x00        ], 4;
]
"verw" = [
    b"m!"         , [0x0F, 0x00        ], 5;
]
"vmcall" = [
    b""           , [0x0F, 0x01, 0xC1  ], X, VMX;
]
"vmclear" = [
    b"m!"         , [0x0F, 0xC7        ], 6, PREF_66 | VMX;
]
"vmfunc" = [
    b""           , [0x0F, 0x01, 0xD4  ], X, VMX;
]
"vmlaunch" = [
    b""           , [0x0F, 0x01, 0xC2  ], X, VMX;
]
"vmload" = [
    b""           , [0x0F, 0x01, 0xDA  ], X, AMD | VMX;
]
"vmmcall" = [
    b""           , [0x0F, 0x01, 0xD9  ], X, VMX | AMD;
]
"vmptrld" = [
    b"m!"         , [0x0F, 0xC7        ], 6, VMX;
]
"vmptrst" = [
    b"m!"         , [0x0F, 0xC7        ], 7, VMX;
]
"vmread" = [
    b"vqrq"       , [0x0F, 0x78        ], X, ENC_MR | VMX;
]
"vmresume" = [
    b""           , [0x0F, 0x01, 0xC3  ], X, VMX;
]
"vmrun" = [
    b""           , [0x0F, 0x01, 0xD8  ], X, VMX | AMD;
]
"vmsave" = [
    b""           , [0x0F, 0x01, 0xDB  ], X, AMD | VMX;
]
"vmwrite" = [
    b"rqvq"       , [0x0F, 0x79        ], X, VMX;
]
"vmxoff" = [
    b""           , [0x0F, 0x01, 0xC4  ], X, VMX;
]
"vmxon" = [
    b"m!"         , [0x0F, 0xC7        ], 6, PREF_F3 | VMX;
]
"wbinvd" = [
    b""           , [0x0F, 0x09        ], X;
]
"wrfsbase" = [
    b"r*"         , [0x0F, 0xAE        ], 2, AUTO_REXW | PREF_F3;
]
"wrgsbase" = [
    b"r*"         , [0x0F, 0xAE        ], 3, AUTO_REXW | PREF_F3;
]
"wrmsr" = [
    b""           , [0x0F, 0x30        ], X;
]
"wrpkru" = [
    b""           , [0x0F, 0x01, 0xEF  ], X;
]
"wrshr" = [
    b"vd"         , [0x0F, 0x37        ], 0, CYRIX;
]
"xabort" = [
    b"ib"         , [0xC6, 0xF8        ], X, RTM;
]
"xadd" = [
    b"mbrb"       , [0x0F, 0xC0        ], X, LOCK | ENC_MR;
    b"rbrb"       , [0x0F, 0xC0        ], X, ENC_MR;
    b"m*r*"       , [0x0F, 0xC1        ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*r*"       , [0x0F, 0xC1        ], X, AUTO_SIZE | ENC_MR;
]
"xbegin" = [
    b"od"         , [0xC7, 0xF8        ], X, RTM;
]
"xchg" = [
    b"mbrb"       , [0x86              ], X, LOCK | ENC_MR;
    b"rbmb"       , [0x86              ], X, LOCK;
    b"rbrb"       , [0x86              ], X, ENC_MR;
    b"rbrb"       , [0x86              ], X;
    b"A*r*"       , [0x90              ], X, AUTO_SIZE | SHORT_ARG;
    b"m*r*"       , [0x87              ], X, AUTO_SIZE | ENC_MR;
    b"r*A*"       , [0x90              ], X, AUTO_SIZE | SHORT_ARG;
    b"r*m*"       , [0x87              ], X, AUTO_SIZE;
    b"r*r*"       , [0x87              ], X, AUTO_SIZE | ENC_MR;
    b"r*r*"       , [0x87              ], X, AUTO_SIZE;
]
"xcryptcbc" = [
    b""           , [0x0F, 0xA7, 0xD0  ], X, PREF_F3 | CYRIX;
]
"xcryptcfb" = [
    b""           , [0x0F, 0xA7, 0xE0  ], X, PREF_F3 | CYRIX;
]
"xcryptctr" = [
    b""           , [0x0F, 0xA7, 0xD8  ], X, PREF_F3 | CYRIX;
]
"xcryptecb" = [
    b""           , [0x0F, 0xA7, 0xC8  ], X, PREF_F3 | CYRIX;
]
"xcryptofb" = [
    b""           , [0x0F, 0xA7, 0xE8  ], X, PREF_F3 | CYRIX;
]
"xend" = [
    b""           , [0x0F, 0x01, 0xD5  ], X, RTM;
]
"xgetbv" = [
    b""           , [0x0F, 0x01, 0xD0  ], X;
]
"xlat" = [
    b""           , [0xD7              ], X;
]
"xlatb" = [
    b""           , [0xD7              ], X;
]
"xor" = [
    b"Abib"       , [0x34              ], X;
    b"mbib"       , [0x80              ], 6, LOCK;
    b"mbrb"       , [0x30              ], X, LOCK | ENC_MR;
    b"rbib"       , [0x80              ], 6;
    b"rbrb"       , [0x30              ], X, ENC_MR;
    b"rbvb"       , [0x32              ], X;
    b"r*ib"       , [0x83              ], 6, AUTO_SIZE | LOCK| EXACT_SIZE;
    b"A*i*"       , [0x35              ], X, AUTO_SIZE;
    b"m*i*"       , [0x81              ], 6, AUTO_SIZE | LOCK;
    b"m*ib"       , [0x83              ], 6, AUTO_SIZE | LOCK;
    b"m*r*"       , [0x31              ], X, AUTO_SIZE | LOCK | ENC_MR;
    b"r*i*"       , [0x81              ], 6, AUTO_SIZE | LOCK;
    b"r*r*"       , [0x31              ], X, AUTO_SIZE | ENC_MR;
    b"r*v*"       , [0x33              ], X, AUTO_SIZE;
]
"xorpd" = [
    b"yowo"       , [0x0F, 0x57        ], X, PREF_66 | SSE2;
]
"xorps" = [
    b"yowo"       , [0x0F, 0x57        ], X, SSE;
]
"xrstor" = [
    b"m!"         , [0x0F, 0xAE        ], 5;
]
"xrstor64" = [
    b"m!"         , [0x0F, 0xAE        ], 5, WITH_REXW;
]
"xrstors64" = [
    b"m!"         , [0x0F, 0xC7        ], 3, WITH_REXW;
]
"xsave" = [
    b"m!"         , [0x0F, 0xAE        ], 4;
]
"xsave64" = [
    b"m!"         , [0x0F, 0xAE        ], 4, WITH_REXW;
]
"xsavec64" = [
    b"m!"         , [0x0F, 0xC7        ], 4, WITH_REXW;
]
"xsaveopt64" = [
    b"m!"         , [0x0F, 0xAE        ], 6, WITH_REXW;
]
"xsaves64" = [
    b"m!"         , [0x0F, 0xC7        ], 5, WITH_REXW;
]
"xsetbv" = [
    b""           , [0x0F, 0x01, 0xD1  ], X;
]
"xsha1" = [
    b""           , [0x0F, 0xA6, 0xC8  ], X, PREF_F3 | CYRIX;
]
"xsha256" = [
    b""           , [0x0F, 0xA6, 0xD0  ], X, PREF_F3 | CYRIX;
]
"xstore" = [
    b""           , [0x0F, 0xA7, 0xC0  ], X, CYRIX;
]
"xtest" = [
    b""           , [0x0F, 0x01, 0xD6  ], X, RTM;
]
);
