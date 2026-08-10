use nom::{
    bytes::complete::{tag, take_till},
    character::complete::char,
    combinator::{fail, opt},
    sequence::delimited,
    IResult, Parser,
};

use super::DidPart;

pub(super) fn parse_did_web(input: &str) -> IResult<&str, DidPart<'_>> {
    fn did_web_method(input: &str) -> IResult<&str, &str> {
        delimited(char(':'), tag("web"), char(':')).parse(input)
    }

    let (input_left, (prefix, method, namespace, id)) = (
        tag("did"),
        did_web_method,
        opt(fail::<_, &str, _>()),
        take_till(|c| "?/#".contains(c)),
    )
        .parse(input)?;

    Ok((input_left, (prefix, method, namespace, id)))
}
