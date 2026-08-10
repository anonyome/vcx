use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::char,
    combinator::{cut, fail, opt, recognize},
    sequence::delimited,
    IResult, Parser,
};

use super::{DidPart, BASE58CHARS};

fn is_base58char(c: char) -> bool {
    BASE58CHARS.contains(c)
}

// mb-value       := z[a-km-zA-HJ-NP-Z1-9]+
fn parse_mb_value(input: &str) -> IResult<&str, &str> {
    recognize((char('z'), take_while1(is_base58char))).parse(input)
}

// did-key-format := did:key:<mb-value>
pub(super) fn parse_did_key(input: &str) -> IResult<&str, DidPart<'_>> {
    fn did_key_method(input: &str) -> IResult<&str, &str> {
        delimited(char(':'), tag("key"), char(':')).parse(input)
    }

    let (input_left, (prefix, method, namespace, id)) = (
        tag("did"),
        did_key_method,
        opt(fail::<_, &str, _>()),
        cut(parse_mb_value),
    )
        .parse(input)?;

    Ok((input_left, (prefix, method, namespace, id)))
}
