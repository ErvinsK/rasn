use rasn::prelude::*;

/*
World-Schema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
    S ::= CHOICE {
        A ),
        B INTEGER OPTIONAL,
        ...
    }
END
*/

#[doc = "A"]
#[derive(Default, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2"))]
pub struct A (pub FixedOctetString<2>);

#[doc = "A"]
#[derive(Default, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("4"))]
pub struct B (pub FixedOctetString<4>);

#[doc = "S"]
#[derive(Default, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags, identifier = "S")]
#[non_exhaustive]
pub struct S {
    #[rasn(identifier = "A")]
    pub a: A,
    #[rasn(identifier = "B")]
    pub b: Option<Integer>,
}

#[test]
fn test_aper_alignment_sequence_a() {
    let value: u16 = 0xffffu16;
    let mut bits = FixedBitString::<16usize>::default();
    for i in 0..16 {
        if (value & (1 << (15 - i))) != 0 {
            bits.set(i, true);
        }
    }
    let original = S {
        a: A(bits),
        b: Some(42.into()),
    };
    let encoded = rasn::aper::encode(&original).expect("encode");
    println!("encoded: {:02X?}", encoded);
    let decoded: S = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}


