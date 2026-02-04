use rasn::prelude::*;

/*
World-Schema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
    Variable_Octet_String ::= OCTETSTRING (SIZE(3..8), ...)
END
*/

#[doc = "OCTET STRING (SIZE(3..8), ...)"]
#[derive(Default, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("3..=8", extensible))]
pub struct OCSTR(pub OctetString);

#[test]
fn test_aper_alignment_variable_octet_string() {
    // Test value within base range
    let original = OCSTR(OctetString::from(vec![0xff; 5]));
    let encoded = rasn::aper::encode(&original).expect("encode");
    let decoded: OCSTR = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded); 
    // Test extended value
    let original = OCSTR(OctetString::from(vec![0xff; 10]));
    let encoded = rasn::aper::encode(&original).expect("encode");
    let decoded: OCSTR = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

/*
World-Schema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
    Variable_Octet_String ::= OCTETSTRING (SIZE(3), ...)
END
*/

#[doc = "OCTET STRING (SIZE(3), ...)"]
#[derive(Default, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("3", extensible))]
pub struct OCSTR2(pub OctetString);

#[test]
fn test_aper_alignment_variable_octet_string2() {
    // Test value within base range
    let original = OCSTR2(OctetString::from(vec![0xff; 3]));
    let encoded = rasn::aper::encode(&original).expect("encode");
    let decoded: OCSTR2 = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded); 
    // Test extended value
    let original = OCSTR2(OctetString::from(vec![0xff; 10]));
    let encoded = rasn::aper::encode(&original).expect("encode");
    let decoded: OCSTR2 = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

/*
World-Schema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
    Variable_Octet_String ::= OCTETSTRING (SIZE(1..32))
END
*/

#[doc = "OCTET STRING (SIZE(1..32))"]
#[derive(Default, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("1..=32"))]
pub struct OCSTR3(pub OctetString);

#[test]
fn test_aper_alignment_variable_octet_string3() {
    // Test value less than 2
    let original = OCSTR3(OctetString::from(vec![0xff]));
    let encoded = rasn::aper::encode(&original).expect("encode");
    let decoded: OCSTR3 = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded); 
    // Test value more than 2
    let original = OCSTR3(OctetString::from(vec![0xff; 10]));
    let encoded = rasn::aper::encode(&original).expect("encode");
    let decoded: OCSTR3 = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}