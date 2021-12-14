use std::str::FromStr;

use nom::{
    IResult,
    Finish,
    branch::alt,
    bytes::complete::{tag},
    character::complete::*,
    combinator::{value, map},
    error::{Error, ParseError},
    sequence::{pair, delimited},
};

// pub type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, Clone, Copy)]
struct Movement {
    pub direction: Direction,
    pub units: u8,
}

impl FromStr for Movement {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match movement(s).finish() {
            Ok((_remaining, movement)) => Ok(movement),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            })
        }
    }
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and 
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_file<T: FromStr>(filename: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> 
where 
    T: FromStr, 
    T::Err: std::error::Error + 'static
{
    let input_file = File::open(filename).unwrap();
    let buffered_input = BufReader::new(input_file);

    let numbers = buffered_input.lines()
                    .map(|line| Ok(line?.parse()?) );

    return numbers.collect();
}

fn direction(input: &str)-> IResult<&str, Direction>{
    alt((
        value(Direction::Forward,   tag("forward")),
        value(Direction::Down,      tag("down")),
        value(Direction::Up,        tag("up")),
    ))(input)
}

fn movement(input: &str) -> IResult<&str, Movement> {
    map(
        pair(
            ws(direction),
            ws(u8)
        ),
        |(direction, units)| { Movement{ direction, units } }
    )(input)
}

fn get_dive_position_simple(movements: &Vec<Movement>) -> (u16, u16) {
    let mut horizontal: u16 = 0;
    let mut depth: u16 = 0;

    for &mov in movements {
        match mov {
            Movement { direction: Direction::Forward, units}    => horizontal += units as u16,
            Movement { direction: Direction::Down, units}       => depth += units as u16,
            Movement { direction: Direction::Up, units}         => depth -= units as u16,
        }
    }
    (horizontal, depth)
}

fn get_dive_position(movements: &Vec<Movement>) -> (u16, u32) {
    let mut horizontal: u16 = 0;
    let mut depth: u32 = 0;
    let mut aim: u16 = 0;

    for &mov in movements {
        match mov {
            Movement { direction: Direction::Forward, units}    => {
                horizontal += units as u16;
                depth += aim as u32 * units as u32;
            }
            Movement { direction: Direction::Down, units}       => aim += units as u16,
            Movement { direction: Direction::Up, units}         => aim -= units as u16,
        }
    }
    (horizontal, depth)
}

fn main() -> std::io::Result<()> {
    let input: Vec<Movement> = read_file("input.txt").unwrap();

    let (horizontal, depth) = get_dive_position_simple(&input);
    let result: u32 = horizontal as u32 * depth as u32;

    println!("Answer day 2 part 1: {}", result);

    let (horizontal, depth) = get_dive_position(&input);
    let result: u32 = horizontal as u32 * depth as u32;

    println!("Answer day 2 part 2: {}", result);

    Ok(())
}
