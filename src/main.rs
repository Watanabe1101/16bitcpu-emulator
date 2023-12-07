mod opecodes;
use opecodes::Op;

use std::io;
use std::io::Write;

fn decode(instruction: u16) -> Op{
    match instruction{
        x if x >> 12 == Op::AND as u16 => Op::AND,
        x if x >> 12 == Op::ADD as u16 => Op::ADD,
        x if x >> 12 == Op::LDA as u16 => Op::LDA,
        x if x >> 12 == Op::STA as u16 => Op::STA,
        x if x >> 12 == Op::BUN as u16 => Op::BUN,
        x if x >> 12 == Op::BSA as u16 => Op::BSA,
        x if x >> 12 == Op::IAND as u16 => Op::IAND,
        x if x >> 12 == Op::IADD as u16 => Op::IADD,
        x if x >> 12 == Op::ILDA as u16 => Op::ILDA,
        x if x >> 12 == Op::ISTA as u16 => Op::ISTA,
        x if x >> 12 == Op::IBUN as u16 => Op::IBUN,
        x if x >> 12 == Op::IBSA as u16 => Op::IBSA,
        x if x == Op::CLA as u16 => Op::CLA,
        x if x == Op::CLE as u16 => Op::CLE,
        x if x == Op::CMA as u16 => Op::CMA,
        x if x == Op::CME as u16 => Op::CME,
        x if x == Op::CIR as u16 => Op::CIR,
        x if x == Op::CIL as u16 => Op::CIL,
        x if x == Op::INC as u16 => Op::INC,
        x if x == Op::SPA as u16 => Op::SPA,
        x if x == Op::SNA as u16 => Op::SNA,
        x if x == Op::SZA as u16 => Op::SZA,
        x if x == Op::SZE as u16 => Op::SZE,
        x if x == Op::HLT as u16 => Op::HLT,
        x if x == Op::INP as u16 => Op::INP,
        x if x == Op::OUT as u16 => Op::OUT,
        x if x == Op::SKI as u16 => Op::SKI,
        x if x == Op::SKO as u16 => Op::SKO,
        x if x == Op::ION as u16 => Op::ION,
        x if x == Op::IOF as u16 => Op::IOF,
        _ => panic!("Invalid opcode"),
    }
}

fn main(){}