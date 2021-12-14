extern crate num;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::str::FromStr;
use std::iter::Sum;

use num::{Num};


fn read_file<T: Num + FromStr>(filename: &str) -> Result<Vec<T>, Box<dyn Error>> 
where 
    T: FromStr, 
    T::Err: Error + 'static
{
    let input_file = File::open(filename).unwrap();
    let buffered_input = BufReader::new(input_file);

    let numbers = buffered_input.lines()
                    .map(|line| Ok(line?.parse()?) );

    return numbers.collect();
}

fn bigger_than_previous<T: Num + PartialOrd + Copy>(list: &Vec<T>) -> Vec<bool> {
    let mut output : Vec<bool> = vec![false]; // first value can never be bigger
    let mut previous = list[0];
    for &item in list.iter().skip(1) {
        if item > previous {
            output.push(true);
        } else {
            output.push(false);
        }
        previous = item;
    }
    return output;
}

fn moving_average<'a, T: 'a + Num + Sized + Copy>(input: &'a Vec<T>, window_size: usize) -> Vec<T> 
where
    T: Sum<&'a T>
{
    let mut output : Vec<T> = vec![];
    for window in input.windows(window_size) {
        output.push(window.iter().sum())
    }

    return output;
}

fn main() -> std::io::Result<()> {

    let input : Vec<u16> = read_file("input.txt").unwrap();
    let bigger = bigger_than_previous(&input);
    let result1: u64 = bigger.iter().map(|b| *b as u64).sum();

    println!("Answer day 1 part 1: {}", result1);

    let mov_avg = moving_average(&input, 3);
    let bigger_avg = bigger_than_previous(&mov_avg);
    let result2: u64 = bigger_avg.iter().map(|b| *b as u64).sum();

    println!("Answer day 1 part 2: {}", result2);

    Ok(())
}
