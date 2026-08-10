use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{cut, peek},
    sequence::delimited,
    IResult, Parser,
};

use super::DidPart;
use crate::did::parsing::did_core::general_did_id;

fn did_peer_method(input: &str) -> IResult<&str, &str> {
    delimited(char(':'), tag("peer"), char(':')).parse(input)
}

fn check_4(input: &str) -> IResult<&str, &str> {
    peek(tag("4")).parse(input)
}

pub(super) fn parse_did_peer_4(input: &str) -> IResult<&str, DidPart<'_>> {
    let (input_left, (prefix, method, _peek, id)) =
        (tag("did"), did_peer_method, check_4, cut(general_did_id)).parse(input)?;
    Ok((input_left, (prefix, method, None, id)))
}
