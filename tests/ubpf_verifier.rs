// Converted from the tests for uBPF <https://github.com/iovisor/ubpf>
// Copyright 2015 Big Switch Networks, Inc
// Copyright 2016 6WIND S.A. <quentin.monnet@6wind.com>
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license <http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// The tests contained in this file are extracted from the unit tests of uBPF software. Each test
// in this file has a name in the form `test_verifier_<name>`, and corresponds to the
// (human-readable) code in `ubpf/tree/master/tests/<name>`, available at
// <https://github.com/iovisor/ubpf/tree/master/tests> (hyphen had to be replaced with underscores
// as Rust will not accept them in function names). It is strongly advised to refer to the uBPF
// version to understand what these program do.
//
// Each program was assembled from the uBPF version with the assembler provided by uBPF itself, and
// available at <https://github.com/iovisor/ubpf/tree/master/ubpf>.
// The very few modifications that have been realized should be indicated.

// These are unit tests for the eBPF “verifier”.

#![allow(clippy::deprecated_cfg_attr)]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate solana_rbpf;

use solana_rbpf::assembler::assemble;
use solana_rbpf::ebpf;
use solana_rbpf::EbpfVmNoData;

#[test]
#[should_panic(expected = "[Verifier] Error: division by 0 (insn #1)")]
fn test_verifier_err_div_by_zero_imm() {
    let prog = assemble("
        mov32 r0, 1
        div32 r0, 0
        exit").unwrap();
    let mut vm = EbpfVmNoData::new(Some(&prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: unsupported argument for LE/BE (insn #0)")]
fn test_verifier_err_endian_size() {
    let prog = &[
        0xdc, 0x01, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
        0xb7, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    let mut vm = EbpfVmNoData::new(Some(prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: incomplete LD_DW instruction (insn #0)")]
fn test_verifier_err_incomplete_lddw() { // Note: ubpf has test-err-incomplete-lddw2, which is the same
    let prog = &[
        0x18, 0x00, 0x00, 0x00, 0x88, 0x77, 0x66, 0x55,
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    let mut vm = EbpfVmNoData::new(Some(prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: infinite loop")]
fn test_verifier_err_infinite_loop() {
    let prog = assemble("
        ja -1
        exit").unwrap();
    let mut vm = EbpfVmNoData::new(Some(&prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: invalid destination register (insn #0)")]
fn test_verifier_err_invalid_reg_dst() {
    let prog = assemble("
        mov r11, 1
        exit").unwrap();
    let mut vm = EbpfVmNoData::new(Some(&prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: invalid source register (insn #0)")]
fn test_verifier_err_invalid_reg_src() {
    let prog = assemble("
        mov r0, r11
        exit").unwrap();
    let mut vm = EbpfVmNoData::new(Some(&prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: jump to middle of LD_DW at #2 (insn #0)")]
fn test_verifier_err_jmp_lddw() {
    let prog = assemble("
        ja +1
        lddw r0, 0x1122334455667788
        exit").unwrap();
    let mut vm = EbpfVmNoData::new(Some(&prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: jump out of code to #3 (insn #0)")]
fn test_verifier_err_jmp_out() {
    let prog = assemble("
        ja +2
        exit").unwrap();
    let mut vm = EbpfVmNoData::new(Some(&prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: program does not end with “EXIT” instruction")]
fn test_verifier_err_no_exit() {
    let prog = assemble("
        mov32 r0, 0").unwrap();
    let mut vm = EbpfVmNoData::new(Some(&prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: eBPF program length limited to 65536, here 65537")]
fn test_verifier_err_too_many_instructions() {
    let mut prog = (0..(65536 * ebpf::INSN_SIZE)).map( |x| match x % 8 {
            0 => 0xb7,
            1 => 0x01,
            _ => 0
    }).collect::<Vec<u8>>();
    prog.append(&mut vec![ 0x95, 0, 0, 0, 0, 0, 0, 0 ]);

    let mut vm = EbpfVmNoData::new(Some(&prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: unknown eBPF opcode 0x6 (insn #0)")]
fn test_verifier_err_unknown_opcode() {
    let prog = &[
        0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    let mut vm = EbpfVmNoData::new(Some(prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}

#[test]
#[should_panic(expected = "[Verifier] Error: cannot write into register r10 (insn #0)")]
fn test_verifier_err_write_r10() {
    let prog = assemble("
        mov r10, 1
        exit").unwrap();
    let mut vm = EbpfVmNoData::new(Some(&prog)).unwrap();
    vm.execute_program(&[], &[]).unwrap();
}
