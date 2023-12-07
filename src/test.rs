#[cfg(test)]
mod tests{
    #[test]
    fn test_decode(){
        assert_eq!(decode(0b0000000000011000), Op::AND);
        assert_eq!(decode(0b0001000000000100), Op::ADD);
        assert_eq!(decode(0b0010000001000000), Op::LDA);
        assert_eq!(decode(0b0011000110000000), Op::STA);
        assert_eq!(decode(0b0100000000110000), Op::BUN);
        assert_eq!(decode(0b0101000000000001), Op::BSA);
        assert_eq!(decode(0b1000001001000001), Op::IAND);
        assert_eq!(decode(0b1001011001001100), Op::IADD);
        assert_eq!(decode(0b1010111111100000), Op::ILDA);
        assert_eq!(decode(0b1011000000011100), Op::ISTA);
        assert_eq!(decode(0b1100111111111111), Op::IBUN);
        assert_eq!(decode(0b1101000000000000), Op::IBSA);
        assert_eq!(decode(0b0111100000000000), Op::CLA);
        assert_eq!(decode(0b0111010000000000), Op::CLE);
        assert_eq!(decode(0b0111001000000000), Op::CMA);
        assert_eq!(decode(0b0111000100000000), Op::CME);
        assert_eq!(decode(0b0111000010000000), Op::CIR);
        assert_eq!(decode(0b0111000001000000), Op::CIL);
        assert_eq!(decode(0b0111000000100000), Op::INC);
        assert_eq!(decode(0b0111000000010000), Op::SPA);
        assert_eq!(decode(0b0111000000001000), Op::SNA);
        assert_eq!(decode(0b0111000000000100), Op::SZA);
        assert_eq!(decode(0b0111000000000010), Op::SZE);
        assert_eq!(decode(0b0111000000000001), Op::HLT);
        assert_eq!(decode(0b1111100000000000), Op::INP);
        assert_eq!(decode(0b1111010000000000), Op::OUT);
        assert_eq!(decode(0b1111001000000000), Op::SKI);
        assert_eq!(decode(0b1111000100000000), Op::SKO);
        assert_eq!(decode(0b1111000010000000), Op::ION);
        assert_eq!(decode(0b1111000001000000), Op::IOF);
    }
}