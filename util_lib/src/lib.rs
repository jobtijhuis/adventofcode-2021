use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[path = "nom/nom.rs"]
pub mod nom;

pub fn read_file<T: FromStr>(filename: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> 
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

use std::cmp::PartialEq;
use std::ops::BitXor;
use std::ops::Shl;

#[derive(Debug)]
pub enum ConversionError {
    Overflow,
    NonBinaryInput,
}

pub fn convert<T: PartialEq + From<u8> + BitXor<Output=T> + Shl<Output=T> + Clone>(
    bits: &[u8],
) -> Result<T, ConversionError> {
    if bits.len() > (std::mem::size_of::<T>() * 8) {
        return Err(ConversionError::Overflow);
    }
    if bits.iter()
        .filter(|&&bit| bit != 0 && bit != 1).count() > 0 {
        return Err(ConversionError::NonBinaryInput);
    }

    Ok(bits.iter()
        .fold(T::from(0), |result, &bit| {
            (result << T::from(1)) ^ T::from(bit)
        }))
}

pub fn convert_bits<
    T: From<U> + Shl<Output=T> + BitXor<Output=T>,
    U: From<bool> + Copy>(bits: &[U]) -> T {
    bits.iter()
        .fold(T::from(U::from(false)), |result, &bit| {
            (result << T::from(U::from(true))) ^ T::from(bit)
        })
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
