pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let p = str.len();
    let mut sum = 0;
    str.chars().into_iter().for_each(|x| {
        sum += (x.to_digit(10)).unwrap().pow(p as u32);
    });
    return num == sum;
}
