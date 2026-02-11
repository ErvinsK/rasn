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
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(choice, automatic_tags, identifier = "Choice")]
#[non_exhaustive]
pub enum SChoice {
    #[rasn(identifier = "A")]
    First(B),
    #[rasn(identifier = "B")]
    Second(B),
    #[rasn(extension_addition, identifier = "C")]
    Third(B),
}

#[test]
fn test_aper_alignment_choice() {
    let octets = B(FixedOctetString::from([0xff, 0xff, 0xff, 0xff]));
    let encoded = rasn::aper::encode(&octets).expect("encode");
    println!("encoded: {:02X?}", encoded);
    let decoded: B = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(octets, decoded);
    let original = SChoice::First(B(FixedOctetString::from([0xff, 0xff, 0xff, 0xff])));
    let encoded = rasn::aper::encode(&original).expect("encode");
    println!("encoded: {:02X?}", encoded);
    let decoded: SChoice = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}


