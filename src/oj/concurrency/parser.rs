use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::space0;
use nom::IResult;
use nom::sequence::tuple;

use super::{Statement, VarOrNum};

fn variable(input: &str) -> IResult<&str, &str> {
    take_while1(char::is_alphabetic)(input)
}

fn numeric(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == '-' || c.is_numeric())(input)
}

fn variable_or_numeric(input: &str) -> IResult<&str, VarOrNum> {
    let (i, o) = alt((numeric, variable))(input)?;
    let o = if let Ok(n) = o.parse::<i32>() {
        VarOrNum::Right(n)
    } else {
        VarOrNum::Left(o.into())
    };
    Ok((i, o))
}

fn assignment(input: &str) -> IResult<&str, Statement> {
    let equal = tag("=");
    let (i, (var, _, _, var_or_num)) =
        nom::sequence::tuple((variable, space0, equal, variable_or_numeric))(input)?;
    Ok((i, Statement::Assignment(var.into(), var_or_num)))
}

fn add_assignment(input: &str) -> IResult<&str, Statement> {
    let add_equal = tag("+=");
    let (i, (var, _, _, var_or_num)) =
        tuple((variable, space0, add_equal, variable_or_numeric))(input)?;
    Ok((i, Statement::AddAssignment(var.into(), var_or_num)))
}

fn print(input: &str) -> IResult<&str, Statement> {
    let (i, (_, _, var)) = tuple((tag("print"), space0, variable))(input)?;
    Ok((i, Statement::Print(var.into())))
}

fn lock(input: &str) -> IResult<&str, Statement> {
    let (i, _) = tag("lock")(input)?;
    Ok((i, Statement::Lock))
}

fn unlock(input: &str) -> IResult<&str, Statement> {
    let (i, _) = tag("unlock")(input)?;
    Ok((i, Statement::UnLock))
}

fn yield_(input: &str) -> IResult<&str, Statement> {
    let (i, _) = tag("yield")(input)?;
    Ok((i, Statement::Yield))
}

fn end(input: &str) -> IResult<&str, Statement> {
    let (i, _) = tag("end")(input)?;
    Ok((i, Statement::End))
}

pub fn parse(input: &str) -> Vec<Statement> {
    input
        .lines()
        .map(|line| alt((assignment, add_assignment, print, lock, unlock, yield_, end))(line))
        .filter_map(|r| r.ok().map(|(_, s)| s))
        .collect()
}
