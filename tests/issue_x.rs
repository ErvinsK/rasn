use rasn::prelude::*;

/*
World-Schema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
    ItemList ::= SEQUENCE (SIZE(0..65535)) OF INTEGER
END
*/

#[doc = "Fixed OCTET STRING <=2 octets"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, identifier = "FixedOctetString<2>")]
pub struct A1(pub FixedOctetString<2>);

#[doc = "Fixed OCTET STRING >2 octets"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, identifier = "FixedOctetString<3>")]
pub struct A2(pub FixedOctetString<3>);

#[doc = "Fixed BIT STRING <=16 bits"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, identifier = "FixedBitString<12>")]
pub struct B1(pub FixedBitString<12>);

#[doc = "Fixed BIT STRING >16 bits"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, identifier = "FixedBitString<17>")]
pub struct B2(pub FixedBitString<17>);

#[doc = "SEQUENCE To Test APER alignment"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(identifier = "SEQUENCE To Test APER alignment")]
#[non_exhaustive]
pub struct S1 {
    pub a1: A2,
    pub b2: Option<B2>,
}

#[doc = "SEQUENCE OF To Test APER alignment"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("0..=300"))]
#[rasn(identifier = "SEQUENCE OF (SIZE(0..=300, ...)) To Test APER alignment")]
#[non_exhaustive]
pub struct S2(pub SequenceOf<A2>);

#[doc = "SEQUENCE OF (SIZE(1..300)) To Test APER alignment"]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1..=300"))]
#[rasn(identifier = "SEQUENCE OF (SIZE(1..300)) To Test APER alignment")]
#[non_exhaustive]
pub struct S3(pub SequenceOf<A2>);


#[test]
fn test_aper_alignment_sequence() {
    let original = S1 {
        a1: A2(FixedOctetString::new([0xff, 0xff, 0xff])),
        b2: Some(B2(FixedBitString::new([0b11111111, 0b11111111, 0b10000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))),
    };
    let encoded = rasn::aper::encode(&original).expect("encode");
    let decoded: S1 = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_aper_alignment_sequence_of() {
    let original = S2(SequenceOf::from(vec![
        A2(FixedOctetString::new([0xff, 0xff, 0xff])),
        A2(FixedOctetString::new([0xaa, 0xaa, 0xaa])),
        A2(FixedOctetString::new([0x55, 0x55, 0x55])),
    ]));
    let encoded = rasn::aper::encode(&original).expect("encode");
    let decoded: S2 = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_aper_alignment_sequence_of_1_300() {
    let original = S3(SequenceOf::from(vec![
        A2(FixedOctetString::new([0xff, 0xff, 0xff])),
        A2(FixedOctetString::new([0xaa, 0xaa, 0xaa])),
        A2(FixedOctetString::new([0x55, 0x55, 0x55])),
    ]));
    let encoded = rasn::aper::encode(&original).expect("encode");
    let decoded: S3 = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}
