use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::space0;
use nom::IResult;
use nom::sequence::tuple;

use super::{Command, Direction};

fn numeric(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == '-' || c.is_numeric())(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (i, dir) = alt((tag("0"), tag("1")))(input)?;
    let dir = match dir {
        "0" => Direction::Left,
        "1" => Direction::Right,
        _ => unreachable!(),
    };
    Ok((i, dir))
}

fn fcfs(input: &str) -> IResult<&str, Command> {
    let (i, (_, _, curr)) = tuple((tag("fcfs"), space0, numeric))(input)?;
    Ok((i, Command::FCFS(curr.parse().unwrap())))
}

fn sstf(input: &str) -> IResult<&str, Command> {
    let (i, (_, _, curr, _, dir)) =
        tuple((tag("sstf"), space0, numeric, space0, direction))(input)?;
    Ok((i, Command::SSTF(curr.parse().unwrap(), dir)))
}

fn scan(input: &str) -> IResult<&str, Command> {
    let (i, (_, _, curr, _, dir)) =
        tuple((tag("scan"), space0, numeric, space0, direction))(input)?;
    Ok((i, Command::Scan(curr.parse().unwrap(), dir)))
}

fn cscan(input: &str) -> IResult<&str, Command> {
    let (i, (_, _, curr)) = tuple((tag("cscan"), space0, numeric))(input)?;
    Ok((i, Command::CScan(curr.parse().unwrap())))
}

fn nstep(input: &str) -> IResult<&str, Command> {
    let (i, (_, _, curr, _, dir, _, gsize)) = tuple((
        tag("nstep"),
        space0,
        numeric,
        space0,
        direction,
        space0,
        numeric,
    ))(input)?;
    Ok((
        i,
        Command::NStep(curr.parse().unwrap(), dir, gsize.parse().unwrap()),
    ))
}

pub fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| alt((fcfs, sstf, scan, cscan, nstep))(line))
        .filter_map(|r| r.ok().map(|(_, s)| s))
        .collect()
}
