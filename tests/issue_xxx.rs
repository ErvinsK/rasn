use bitvec::prelude::*;
use rasn::prelude::*;

/*
World-Schema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
    Variable_Bit_String ::= BIT STRING (SIZE(3..23), ...)
END
*/

#[doc = "BIT STRING (SIZE(0..23), ...)"]
#[derive(Default, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("0..=23", extensible))]
pub struct BITSTR(pub BitString);

#[test]
fn test_aper_alignment_variable_bit_string() {
    // Test value within base range
    let bv: BitVec::<u8, Msb0> = [true;3].into_iter().collect();
    println!("bv len: {:?}", bv);
    println!("BITSTR Constaints: {:?}", BITSTR::CONSTRAINTS);
    let original = BITSTR(bv);
    let encoded = rasn::aper::encode(&original).expect("encode");
    println!("encoded: {:02X?}", encoded);
    let decoded: BITSTR = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded); 
    // Test extended value
    let bv: BitVec::<u8, Msb0> = [true;25].into_iter().collect();
    let original = BITSTR(bv);
    let encoded = rasn::aper::encode(&original).expect("encode");
    println!("encoded: {:02X?}", encoded);
    let decoded: BITSTR = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

/*
World-Schema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
    Variable_Bit_String ::= BIT STRING (SIZE(3..23), ...)
END
*/

#[doc = "BIT STRING (SIZE(1..32))"]
#[derive(Default, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1..=32"))]
pub struct BITSTR2(pub BitString);

#[test]
fn test_aper_alignment_variable_bit_string2() {
    // Test value within base range
    let bv: BitVec::<u8, Msb0> = [true;1].into_iter().collect();
    let original = BITSTR2(bv);
    let encoded = rasn::aper::encode(&original).expect("encode");
    println!("encoded: {:02X?}", encoded);
    let decoded: BITSTR2 = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded); 
    // Test extended value
    let bv: BitVec::<u8, Msb0> = [true;32].into_iter().collect();
    let original = BITSTR2(bv);
    let encoded = rasn::aper::encode(&original).expect("encode");
    let decoded: BITSTR2 = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

