use dynasmrt::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi};

// instructions that gas had issues with or weren't otherwise testable

#[test]
fn lui_64() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature i
        ; lui x5, 0x789A_B000
        ; lui x6, 0x7FFF_F000
        ; lui x9, -0x1234_5000
        ; lui x10, -0x8000_0000
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "B7B29A7837F3FF7FB7B4CBED37050080", "lui tests");
}

#[test]
fn lui_32() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv32
        ; .feature i
        ; lui x5, 0x789A_B000
        ; lui x6, 0x7FFF_F000
        ; lui x9, -0x1234_5000
        ; lui x10, -0x8000_0000
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "B7B29A7837F3FF7FB7B4CBED37050080", "lui tests");
}

#[test]
fn c_lui_64() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature ic
        ; c.lui x5, 0x1_3000
        ; c.lui x7, 0x1_F000
        ; c.lui x10, -0x1_2000
        ; c.lui x12, -0x2_0000
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "CD62FD6339750176", "c.lui tests");
}

#[test]
fn c_lui_32() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv32
        ; .feature ic
        ; c.lui x5, 0x1_3000
        ; c.lui x7, 0x1_F000
        ; c.lui x10, -0x1_2000
        ; c.lui x12, -0x2_0000
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "CD62FD6339750176", "c.lui tests");
}

#[test]
fn auipc_64() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature i
        ; auipc x1, 0x789A_B000
        ; auipc x2, 0x789A_B7FF
        ; auipc x3, 0x789A_B800
        ; auipc x4, 0x7FFF_F000
        ; auipc x5, 0x7FFF_F7FF
        ; auipc x6, -0x1234_5000
        ; auipc x7, -0x1234_5800
        ; auipc x8, -0x1234_5801
        ; auipc x9, -0x8000_0000
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "97B09A7817B19A7897C19A7817F2FF7F97F2FF7F17B3CBED97B3CBED17A4CBED97040080", "auipc tests");
}

#[test]
fn auipc_32() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv32
        ; .feature i
        ; auipc x1, 0x789A_B000
        ; auipc x2, 0x789A_B7FF
        ; auipc x3, 0x789A_B800
        ; auipc x4, 0x7FFF_F000
        ; auipc x5, 0x7FFF_F7FF
        ; auipc x6, -0x1234_5000
        ; auipc x7, -0x1234_5800
        ; auipc x8, -0x1234_5801
        ; auipc x9, -0x8000_0000
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "97B09A7817B19A7897C19A7817F2FF7F97F2FF7F17B3CBED97B3CBED17A4CBED97040080", "auipc tests");
}

#[test]
fn li_64() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature i
        ; li.32 x1, 0x1234_5678
        ; li.32 x2, 0x7FFF_FFFF
        ; li.32 x3, -0x1234_5678
        ; li.32 x4, -0x8000_0000
        ; li.43 x5, 0x345_6789_ABCD
        ; li.43 x6, 0x3FF_FFFF_FFFF
        ; li.43 x7, -0x345_6789_ABCD
        ; li.43 x8, -0x400_0000_0000
        ; li.54 x9, 0x12_3456_7890_ABCD
        ; li.54 x10, 0x1F_FFFF_FFFF_FFFF
        ; li.54 x11, -0x12_3456_7890_ABCD
        ; li.54 x12, -0x20_0000_0000_0000
        ; li  x13, 0x7890_ABCD_EF01_2345
        ; li  x14, 0x7FFF_FFFF_FFFF_FFFF
        ; li  x15, -0x7890_ABCD_EF01_2345
        ; li  x16, -0x8000_0000_0000_0000
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "B75034129B808067370100801B01F1FFB7B1CBED9B818198370200801B020200B7F2AC689B8252139392B2009382D23C370300801B03F3FF1313B3001303F37FB71353979B83A3EC9393B30093833343370400801B0404001314B40013040400B764D1489B84249E9394B400938454219394B4009384D43C370500801B05F5FF1315B5001305F57F1315B5001305F57FB7A52EB79B85D5619395B5009385A55E9395B50093853543370600801B0606001316B600130606001316B60013060600B7B690789B86D6BC9396B600938686779396B600938686049396A60093865634370700801B07F7FF1317B7001307F77F1317B7001307F77F1317A7001307F73FB7576F879B8727439397B700938777089397B7009387777B9397A7009387B70B370800801B0808001318B800130808001318B800130808001318A80013080800", "li tests");
}

#[test]
fn li_32() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv32
        ; .feature i
        ; li x1, 0x1234_5678
        ; li x2, 0x7FFF_FFFF
        ; li x3, -0x1234_5678
        ; li x4, -0x8000_0000
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "B750341293808067370100801301F1FFB7B1CBED938181983702008013020200", "li tests");
}

// gas thought that this one didn't exist for riscv64, but the datasheet disagrees
#[test]
fn ssamoswap_w_1284() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature zicfiss
        ; ssamoswap.w x0, x25, [X(23)]
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "2F, A0, 9B, 49", "ssamoswap.w x0, x25, [X(23)]");
}

#[test]
fn ssamoswap_w_1285() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature zicfiss
        ; ssamoswap.w X(20), X(15), [x10]
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join(", ");
    assert_eq!(hex, "2F, 2A, F5, 48", "ssamoswap.w X(20), X(15), [x10]");
}

// pc relative address building
#[test]
fn pc_rel_address_building_64() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::riscv::RiscvRelocation>::new(0x4000_0000_0000_0000);
    dynasm!(ops
        ; .arch riscv64
        ; .feature i
        ; startlabel:

        ; auipc x1, >testlabel
        ; addi x1, x1, >testlabel + 4
        ; addi x1, x1, >testlabel + 8

        ; la x2, <startlabel
        ; la x3, 0x1234_5678
        ; la x4, 0x7FFF_F7FF
        ; la x5, -0x1234_5678
        ; la x6, -0x8000_0000

        ; testlabel:
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "97000000938040039380400317010000130141FF975134129381816717F2FF7F1302F27F97B2CBED938282981703008013030300", "pc relative address building");
}

#[test]
fn pc_rel_address_building_32() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::riscv::RiscvRelocation>::new(0x4000_0000);
    dynasm!(ops
        ; .arch riscv32
        ; .feature i
        ; startlabel:

        ; auipc x1, >testlabel
        ; addi x1, x1, >testlabel + 4

        ; la x2, <startlabel
        ; la x3, 0x1234_5678
        ; la x4, 0x7FFF_F7FF
        ; la x5, -0x1234_5678
        ; la x6, -0x8000_0000

        ; testlabel:
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "970000009380000317010000130181FF975134129381816717F2FF7F1302F27F97B2CBED938282981703008013030300", "pc relative address building");
}

// 32-bit call/jump/tail
#[test]
fn pc_rel_big_jumps_64() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::riscv::RiscvRelocation>::new(0x4000_0000_0000_0000);
    dynasm!(ops
        ; .arch riscv64
        ; .feature i
        ; startlabel:

        ; call >testlabel
        ; call x2, <startlabel
        ; tail >testlabel
        ; jump <startlabel, x2

        ; call 0x1234_5678
        ; call 0x7FFF_F7FF
        ; call -0x1234_5678
        ; call -0x8000_0000

        ; tail 0x1234_5678
        ; tail 0x7FFF_F7FF
        ; tail -0x1234_5678
        ; tail -0x8000_0000

        ; jump 0x1234_5678, x3
        ; jump 0x7FFF_F7FF, x4
        ; jump -0x1234_5678, x5
        ; jump -0x8000_0000, x6

        ; testlabel:
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "97000000E780000817010000670181FF170300006700030717010000670081FE97503412E780806797F0FF7FE780F07F97B0CBEDE780809897000080E7800000175334126700836717F3FF7F6700F37F17B3CBED670083981703008067000300975134126780816717F2FF7F6700F27F97B2CBED678082981703008067000300", "big jumps");
}

#[test]
fn pc_rel_big_jumps_32() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::riscv::RiscvRelocation>::new(0x4000_0000);
    dynasm!(ops
        ; .arch riscv32
        ; .feature i
        ; startlabel:

        ; call >testlabel
        ; call x2, <startlabel
        ; tail >testlabel
        ; jump <startlabel, x2

        ; call 0x1234_5678
        ; call 0x7FFF_F7FF
        ; call -0x1234_5678
        ; call -0x8000_0000

        ; tail 0x1234_5678
        ; tail 0x7FFF_F7FF
        ; tail -0x1234_5678
        ; tail -0x8000_0000

        ; jump 0x1234_5678, x3
        ; jump 0x7FFF_F7FF, x4
        ; jump -0x1234_5678, x5
        ; jump -0x8000_0000, x6

        ; testlabel:
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "97000000E780000817010000670181FF170300006700030717010000670081FE97503412E780806797F0FF7FE780F07F97B0CBEDE780809897000080E7800000175334126700836717F3FF7F6700F37F17B3CBED670083981703008067000300975134126780816717F2FF7F6700F27F97B2CBED678082981703008067000300", "big jumps");
}

// pc relative loads
#[test]
fn pc_rel_load_64() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::riscv::RiscvRelocation>::new(0x4000_0000_0000_0000);
    dynasm!(ops
        ; .arch riscv64
        ; .feature ifdqzfh
        ; data:
        ; .u8 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11
        ; .u8 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99

        // pc-relative loads
        ; lb x1, <data
        ; lbu x2, <data
        ; lh x3, <data
        ; lhu x4, <data
        ; lw x5, <data
        ; lwu x6, <data
        ; ld x7, <data
        ; flh f8, <data, x12
        ; flw f9, <data, x13
        ; fld f10, <data, x14
        ; flq f11, <data, x15

        // split pc-relative loads
        ; auipc x16, <data
        ; lb x1, [x16, <data + 4]
        ; lbu x2, [x16, <data + 8]
        ; lh x3, [x16, <data + 12]
        ; lhu x4, [x16, <data + 16]
        ; lw x5, [x16, <data + 20]
        ; lwu x6, [x16, <data + 24]
        ; ld x7, [x16, <data + 28]
        ; flh f8, [x16, <data + 32]
        ; flw f9, [x16, <data + 36]
        ; fld f10, [x16, <data + 40]
        ; flq f11, [x16, <data + 44]

        ; testlabel:
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "AABBCCDDEEFF0011223344556677889997000000838000FF17010000034181FE97010000839101FE17020000035282FD9702000083A202FD17030000036383FC9703000083B303FC17060000071486FB9706000087A406FB17070000073587FA9707000087C507FA17080000830088F9034188F9831188F9035288F9832288F9036388F9833388F9071488F9872488F9073588F9874588F9", "pc-relative loads");
}

#[test]
fn pc_rel_load_32() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::riscv::RiscvRelocation>::new(0x4000_0000_0000_0000);
    dynasm!(ops
        ; .arch riscv32
        ; .feature ifdqzfh
        ; data:
        ; .u8 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11
        ; .u8 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99

        // pc-relative loads
        ; lb x1, <data
        ; lbu x2, <data
        ; lh x3, <data
        ; lhu x4, <data
        ; lw x5, <data
        ; flh f8, <data, x12
        ; flw f9, <data, x13
        ; fld f10, <data, x14
        ; flq f11, <data, x15

        // split pc-relative loads
        ; auipc x16, <data
        ; lb x1, [x16, <data + 4]
        ; lbu x2, [x16, <data + 8]
        ; lh x3, [x16, <data + 12]
        ; lhu x4, [x16, <data + 16]
        ; lw x5, [x16, <data + 20]
        ; flh f8, [x16, <data + 24]
        ; flw f9, [x16, <data + 28]
        ; fld f10, [x16, <data + 32]
        ; flq f11, [x16, <data + 36]

        ; testlabel:
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "AABBCCDDEEFF0011223344556677889997000000838000FF17010000034181FE97010000839101FE17020000035282FD9702000083A202FD17060000071486FC9706000087A406FC17070000073587FB9707000087C507FB17080000830088FA034188FA831188FA035288FA832288FA071488FA872488FA073588FA874588FA", "pc-relative loads");
}

// pc relative stores
#[test]
fn pc_rel_store_64() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::riscv::RiscvRelocation>::new(0x4000_0000_0000_0000);
    dynasm!(ops
        ; .arch riscv64
        ; .feature ifdqzfh
        ; data:
        ; .u8 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11
        ; .u8 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99

        // pc-relative stores
        ; sb x1, <data, x1
        ; sh x2, <data, x2
        ; sw x3, <data, x3
        ; sd x4, <data, x4
        ; flh f5, <data, x5
        ; fsw f6, <data, x6
        ; fsd f7, <data, x7
        ; fsq f8, <data, x8

        // split pc-relative stores
        ; auipc x16, <data
        ; sb x1, [x16, <data + 4]
        ; sh x3, [x16, <data + 8]
        ; sw x5, [x16, <data + 12]
        ; sd x6, [x16, <data + 16]
        ; fsh f8, [x16, <data + 20]
        ; fsw f9, [x16, <data + 24]
        ; fsd f10, [x16, <data + 28]
        ; fsq f11, [x16, <data + 32]

        ; testlabel:
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "AABBCCDDEEFF0011223344556677889997000000238810FE17010000231421FE9701000023A031FE17020000233C42FC97020000879202FD17030000272463FC9703000027B073FC17040000274C84FA17080000230818FA231838FA232858FA233868FA271888FA272898FA2738A8FA2748B8FA", "pc-relative stores");
}

#[test]
fn pc_rel_store_32() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::riscv::RiscvRelocation>::new(0x4000_0000_0000_0000);
    dynasm!(ops
        ; .arch riscv32
        ; .feature ifdqzfh
        ; data:
        ; .u8 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11
        ; .u8 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99

        // pc-relative stores
        ; sb x1, <data, x1
        ; sh x2, <data, x2
        ; sw x3, <data, x3
        ; flh f5, <data, x5
        ; fsw f6, <data, x6
        ; fsd f7, <data, x7
        ; fsq f8, <data, x8

        // split pc-relative stores
        ; auipc x16, <data
        ; sb x1, [x16, <data + 4]
        ; sh x3, [x16, <data + 8]
        ; sw x5, [x16, <data + 12]
        ; fsh f8, [x16, <data + 16]
        ; fsw f9, [x16, <data + 20]
        ; fsd f10, [x16, <data + 24]
        ; fsq f11, [x16, <data + 28]

        ; testlabel:
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex = hex.join("");
    assert_eq!(hex, "AABBCCDDEEFF0011223344556677889997000000238810FE17010000231421FE9701000023A031FE97020000879282FD17030000272863FC9703000027B473FC17040000274084FC17080000230C18FA231C38FA232C58FA271C88FA272C98FA273CA8FA274CB8FA", "pc-relative stores");
}

