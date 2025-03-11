use dynasmrt::dynasm;
use dynasmrt::DynasmApi;

// confirms that static and dynamic encodings for immediates result in the same data
// (regular tests confirmthis for registers)

// black box function that blocks constant evaluation
fn i<I>(i: I) -> I { i }

// chunk comparison function
fn are_chunks_equal(data: &[u8], chunksize: usize) -> bool {
    assert!(data.len() >= chunksize * 2);
    assert!(((data.len() / chunksize) * chunksize) == data.len());

    let mut iter = data.chunks_exact(chunksize);

    let first = iter.next().expect("data was empty");

    for c in iter {
        if c != first {
            return false;
        }
    }
    true
}

#[test]
fn register_lists_1() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature GCQZcmt_Zcmp_Zacas_Zfa
        ; cm.push {ra, s0 - s6}, -80
        ; cm.push {ra, s0 - s6}, i(-80)
        ; cm.push {ra; 7}, -80
        ; cm.push {ra; 7}, i(-80)
        ; cm.push {ra; i(7)}, -80
        ; cm.push {ra; i(7)}, i(-80)
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 4), "register_lists");
}

#[test]
fn register_lists_2() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature GCQZcmt_Zcmp_Zacas_Zfa
        ; cm.push {ra, s0 - s11}, -112
        ; cm.push {ra, s0 - s11}, i(-112)
        ; cm.push {ra; 12}, -112
        ; cm.push {ra; 12}, i(-112)
        ; cm.push {ra; i(12)}, -112
        ; cm.push {ra; i(12)}, i(-112)
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 2), "register_lists");
}

#[test]
fn csrs() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature GCQZicsr
        ; csrc cycle, x3
        ; csrc 0xC00, x3
        ; csrc i(0xC00), x3
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 4), "csrs");
}

#[test]
fn uimm() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature GCZcmop_Zimop
        ; mop.r.27 x3, x4
        ; mop.r.N 27, x3, x4
        ; mop.r.N i(27), x3, x4
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 4), "uimm");
}

#[test]
fn simm() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature G
        ; andi x1, x2, -0x45C
        ; andi x1, x2, i(-0x45C)
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 4), "uimm");
}

#[test]
fn uimm_odd() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature GCZcmop_Zimop
        ; c.mop.11
        ; c.mop.N 11
        ; c.mop.N i(11)
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 2), "uimm_odd");
}

#[test]
fn uimm_no0() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature GC
        ; c.addi4spn x8, x2, 0x3C4
        ; c.addi4spn x8, x2, i(0x3C4)
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 2), "uimm_no0");
}

#[test]
fn simm_no0() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature GC
        ; c.addi16sp x2, 0x1C0
        ; c.addi16sp x2, i(0x1C0)
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 2), "simm_no0");
}

#[test]
fn bigimm() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature G
        ; li x23, 0x1234_5678_9ABC_DEF0
        ; li x23, i(0x1234_5678_9ABC_DEF0)
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 32), "bigimm");
}

#[test]
fn uimm_range() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature GZcmt
        ; cm.jalt 177
        ; cm.jalt i(177)
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 2), "uimm_range");
}

#[test]
fn offsets_range() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch riscv64
        ; .feature GZcmt
        ; la x5, 0x12345678
        ; la x5, i(0x12345678)
    );
    let buf = ops.finalize();
    assert!(are_chunks_equal(&buf, 8), "offsets_range");
}
