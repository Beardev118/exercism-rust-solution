pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|candidate: &u32| {
            if !primes.iter().any(|i| candidate % i == 0) {
                primes.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}

// pub fn nth(n: u32) -> u32 {
//     let mut cnt = 0;
//     let mut ans = 2u32;
//     let mut temp = 3u32;
//     loop {
//         if cnt == n {
//             break;
//         }

//         if temp == firstfac(temp) {
//             cnt += 1;
//             ans = temp;
//         }

//         temp += 2;
//     }

//     ans
// }

// pub fn firstfac(x: u32) -> u32 {
//     if x % 2 == 0 {
//         return 2;
//     };

//     for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
//         if x % n == 0 {
//             return n;
//         };
//     }

//     x
// }
