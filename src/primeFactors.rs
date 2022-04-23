pub fn factors(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![];
    };
    let mut lst: Vec<u64> = Vec::new();
    let mut curn = n;
    loop {
        let m = firstfac(curn);
        lst.push(m);
        if m == curn {
            break;
        } else {
            curn /= m
        };
    }
    lst
}

pub fn firstfac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };

    for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }

    x
}
