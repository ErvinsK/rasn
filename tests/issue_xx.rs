use rasn::prelude::*;

/*
World-Schema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
    Variable_Octet_String ::= OCTETSTRING (SIZE(3..8))
END
*/

#[doc = "OCTET STRING (SIZE(3..8))"]
#[derive(Default, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("3..=8"))]
pub struct OCSTR(pub OctetString);


#[test]
fn test_aper_alignment_variable_octet_string() {
    let original = OCSTR(OctetString::from(vec![0xff; 5]));
    let encoded = rasn::aper::encode(&original).expect("encode");
    println!("Encoded OCSTR: {:x?}", encoded);
    let decoded: OCSTR = rasn::aper::decode(&encoded).expect("decode");
    assert_eq!(original, decoded);
}


