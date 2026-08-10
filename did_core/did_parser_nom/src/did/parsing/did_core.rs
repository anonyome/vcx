// https://www.w3.org/TR/did-core/#did-syntax
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alphanumeric1, char, satisfy},
    combinator::recognize,
    multi::{many0, many1},
    sequence::{delimited, terminated},
    AsChar, IResult, Parser,
};

use super::DidPart;

fn hexadecimal_digit(input: &str) -> IResult<&str, &str> {
    fn is_hexadecimal_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    recognize(satisfy(is_hexadecimal_digit)).parse(input)
}

fn is_lowercase_alphanumeric(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_dec_digit()
}

// pct-encoded = "%" HEXDIG HEXDIG
fn pct_encoded(input: &str) -> IResult<&str, &str> {
    recognize((tag("%"), hexadecimal_digit, hexadecimal_digit)).parse(input)
}

// idchar = ALPHA / DIGIT / "." / "-" / "_" / pct-encoded
pub(super) fn idchar(input: &str) -> IResult<&str, &str> {
    alt((alphanumeric1, tag("."), tag("-"), tag("_"), pct_encoded)).parse(input)
}

// method-name = 1*method-char
// method-char = %x61-7A / DIGIT
fn method_name(input: &str) -> IResult<&str, &str> {
    delimited(char(':'), take_while1(is_lowercase_alphanumeric), char(':')).parse(input)
}

fn method_specific_id_optional_repeat(input: &str) -> IResult<&str, &str> {
    log::trace!("did_core::method_specific_id_optional_repeat >> input: {input:?}");
    let ret = recognize(many0(terminated(many0(idchar), char(':')))).parse(input); // First half of DID Syntax ABNF rule method-specific-id: *( *idchar ":"
                                                                                   // )recognize(many1(idchar))(input)
    log::trace!("did_core::method_specific_id_optional_repeat >> ret: {ret:?}");
    ret
}

fn method_specific_id_required_characters(input: &str) -> IResult<&str, &str> {
    log::trace!("did_core::method_specific_id_required_characters >> input: {input:?}");
    let ret = recognize(many1(idchar)).parse(input); // Second half of DID Syntax ABNF rule method-specific-id: 1*idchar
    log::trace!("did_core::method_specific_id_required_characters >> ret: {ret:?}");
    ret
}

pub(super) fn general_did_id(input: &str) -> IResult<&str, &str> {
    log::trace!("did_core::general_did_id >> input: {input:?}");
    let (input, did_id) = recognize((
        method_specific_id_optional_repeat,
        method_specific_id_required_characters,
    ))
    .parse(input)?;
    log::trace!("did_core::general_did_id >> did_id: {did_id:?}");
    Ok((input, did_id))
}

// did = "did:" method-name ":" method-specific-id
pub(super) fn parse_qualified_did(input: &str) -> IResult<&str, DidPart<'_>> {
    let (input_left, (prefix, method, id)) =
        (tag("did"), method_name, general_did_id).parse(input)?;

    Ok((input_left, (prefix, method, None, id)))
}
