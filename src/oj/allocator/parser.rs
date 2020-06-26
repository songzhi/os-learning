use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::space0;
use nom::IResult;
use nom::sequence::tuple;

use super::Statement;

fn numeric(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == '-' || c.is_numeric())(input)
}

fn malloc(input: &str) -> IResult<&str, Statement> {
    let malloc_ = tag("malloc");
    let (i, (_, _, len)) = tuple((malloc_, space0, numeric))(input)?;
    // assuming `len` is non-negative
    Ok((i, Statement::Malloc(len.parse().unwrap())))
}

fn malloc_addr(input: &str) -> IResult<&str, Statement> {
    let malloc = tag("malloc");
    let (i, (_, _, addr, len)) = tuple((malloc, space0, numeric, numeric))(input)?;
    Ok((
        i,
        Statement::MallocAddr(addr.parse().unwrap(), len.parse().unwrap()),
    ))
}

fn free(input: &str) -> IResult<&str, Statement> {
    let free_ = tag("free");
    let (i, (_, _, handle)) = tuple((free_, space0, numeric))(input)?;
    Ok((i, Statement::Free(handle.parse().unwrap())))
}

fn compact(input: &str) -> IResult<&str, Statement> {
    let (i, _) = tag("compact")(input)?;
    Ok((i, Statement::Compact))
}

pub fn parse(input: &str) -> Vec<Statement> {
    input
        .lines()
        .map(|line| alt((malloc_addr, malloc, free, compact))(line))
        .filter_map(|r| r.ok().map(|(_, s)| s))
        .collect()
}
