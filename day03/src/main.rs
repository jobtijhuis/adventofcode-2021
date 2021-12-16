use util_lib::read_file;
use util_lib::convert_bits;

use std::str::FromStr;
use std::vec::Vec;
use std::fmt;

use nom::{
    IResult,
    Finish,
    branch::alt,
    bytes::complete::tag,
    combinator::{value, map},
    error::{Error},
    multi::{many1},
};

type BV = Vec<bool>;
#[derive(Clone)]
struct BitVector(BV);

impl From<Vec<bool>> for BitVector {
    fn from(val: Vec<bool>) -> BitVector {
        BitVector(val)
    }
}

impl Into<Vec<bool>> for BitVector {
    fn into(self) -> Vec<bool> {
        self.0
    }
}

impl FromStr for BitVector {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match binary(s).finish() {
            Ok((_remaining, bit_vec)) => Ok(bit_vec),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            })
        }
    }
}

impl fmt::Display for BitVector {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.iter().map(|i| (*i as u8).to_string()).collect::<String>())
    }
}

impl fmt::Debug for BitVector {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.iter().map(|i| (*i as u8).to_string()).collect::<String>())
    }
}

impl std::ops::Deref for BitVector {
    type Target = BV;
    fn deref(&self) -> &BV {
        &self.0
    }
}

impl std::ops::DerefMut for BitVector {
    fn deref_mut(&mut self) -> &mut BV {
        &mut self.0
    }
}

fn binary(input: &str) -> IResult<&str, BitVector> {
    map(
        many1(
            alt((
                value(false, tag("0")),
                value(true, tag("1"))
            ))
        ),
        |x| x.into()
    )(input)
}

fn get_power_consumption(vectors: &Vec<BitVector>) -> u32 {
    let width = vectors[0].len();
    let mut ones_sum: Vec<u16> = vec![0; width];
    let mut zeros_sum: Vec<u16> = vec![0; width];

    for number in vectors {
        let num_vec: Vec<bool> = number.clone().into();
        for (i, bit) in num_vec.iter().enumerate() {
            match bit {
                true => ones_sum[i] += 1,
                false => zeros_sum[i] += 1,
            }
        }
    }

    let gamma_rate_bits: Vec<bool> = ones_sum.iter().enumerate().map(|(i,x)| *x > zeros_sum[i]).collect();
    let epsilon_rate_bits: Vec<bool> = zeros_sum.iter().enumerate().map(|(i,x)| *x > ones_sum[i]).collect();

    let gamma_rate: u16 = convert_bits(&gamma_rate_bits);
    let epsilon_rate: u16 = convert_bits(&epsilon_rate_bits);

    return gamma_rate as u32 * epsilon_rate as u32;
}

fn most_common_bit(input : &Vec<BitVector>, nth: usize) -> bool {
    let mut ones: u16 = 0;
    let mut zeros: u16 = 0;

    for item in input {
        let num_vec: Vec<bool> = item.clone().into();
        match num_vec[nth] {
            true => ones += 1,
            false => zeros += 1,
        }
    }
    return ones >= zeros;
}


fn find_oxygen_generator_rating(vectors: &Vec<BitVector>, nth: usize) -> Vec<BitVector> {

    let most_common_bit = most_common_bit(vectors, nth);
    let result: Vec<BitVector> = vectors.iter().filter(|&x| x.0[nth] == most_common_bit).cloned().collect();
    if result.len() > 1 {
        return find_oxygen_generator_rating(&result, nth+1);
    } else {
        return result;
    }
}


fn find_co2_scrubber_rating(vectors: &Vec<BitVector>, nth: usize) -> Vec<BitVector> {

    let least_common_bit = !most_common_bit(vectors, nth);
    let result: Vec<BitVector> = vectors.iter().filter(|&x| x.0[nth] == least_common_bit).cloned().collect();
    if result.len() > 1 {
        return find_co2_scrubber_rating(&result, nth+1);
    } else {
        return result;
    }
}

fn get_life_support_rating(vectors: &Vec<BitVector>) -> u32{

    let oxygen_generator_bits:Vec<bool> = find_oxygen_generator_rating(vectors, 0)[0].clone().into();
    let oxygen_generator_rating: u16 = convert_bits(&oxygen_generator_bits);

    let co2_scrubber_bits:Vec<bool> = find_co2_scrubber_rating(vectors, 0)[0].clone().into();
    let co2_scrubber_rating: u16 = convert_bits(&co2_scrubber_bits);

    return oxygen_generator_rating as u32 * co2_scrubber_rating as u32;
}

fn main() {
    let input: Vec<BitVector> = read_file("input.txt").unwrap();

    let result = get_power_consumption(&input);
    println!("Answer day 3 part 1: {}", result);

    let result = get_life_support_rating(&input);
    println!("Answer day 3 part 2: {}", result)

}
