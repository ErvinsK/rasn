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
    let original: Vec<SChoice> = vec![
        SChoice::First(B(FixedOctetString::from([0xff, 0xff, 0xff, 0xff]))),
        SChoice::Second(B(FixedOctetString::from([0xff, 0xff, 0xff, 0xff]))),
        SChoice::Third(B(FixedOctetString::from([0xff, 0xff, 0xff, 0xff]))),
    ];
    for choice in original {
        let encoded = rasn::aper::encode(&choice).expect("encode");
        println!("encoded: {:02X?}", encoded);
        let decoded: SChoice = rasn::aper::decode(&encoded).expect("decode");
        assert_eq!(choice, decoded);
    }
}
#[doc = "S"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(automatic_tags, identifier = "S")]
#[non_exhaustive]
pub struct S {
    #[rasn(identifier = "A")]
    pub a: SChoice,
    #[rasn(identifier = "B")]
    pub b: Option<Integer>,
}

#[test]
fn test_aper_sequence_with_choice() {
    let original = S {
        a: SChoice::First(B(FixedOctetString::from([0xff, 0xff, 0xff, 0xff]))),
        b: Some(42.into()),
    };
    let encoded = rasn::aper::encode(&original).expect("encode");
    println!("encoded: {:02X?}", encoded);
    let decoded: S = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}



