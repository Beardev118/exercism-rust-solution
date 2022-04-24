#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    let mut real_val = 0;
    let mut ans = Vec::new();

    for i in 0..number.len() {
        if number[i] >= from_base {
            return Err(Error::InvalidDigit(number[i]));
        }
        real_val += number[i] * from_base.pow((number.len() - 1 - i) as u32);
    }

    loop {
        let temp = real_val % to_base;
        ans.push(temp);
        real_val /= to_base;
        if real_val == 0 {
            break;
        }
    }

    ans.reverse();
    return Ok(ans);
}
