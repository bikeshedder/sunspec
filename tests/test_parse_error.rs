#![cfg(all(feature = "model1", feature = "model160"))]

use sunspec::{
    models::{model1::Model1, model160::Model160},
    Model, ParseError,
};

#[test]
fn test_too_short() {
    assert_eq!(Model1::parse(&[0; 10]), Err(ParseError::TooShort));
}

#[test]
fn test_invalid_group_length() {
    // Model 160 contains 8 registers of points followed by modules of
    // 20 registers each.
    assert_eq!(Model160::parse(&[0; 8 + 2 * 20]).unwrap().module.len(), 2);
    assert_eq!(
        Model160::parse(&[0; 8 + 2 * 20 + 5]),
        Err(ParseError::InvalidGroupLength)
    );
}
