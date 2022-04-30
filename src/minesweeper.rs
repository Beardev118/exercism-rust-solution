static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
pub fn annotate(field: &[&str]) -> Vec<String> {
    let height = field.len() as i32;
    (0..height)
        .map(|y| {
            let width = field[y as usize].len() as i32;
            (0..width)
                .map(|x| {
                    if field[y as usize].as_bytes()[x as usize] == b'*' {
                        '*'
                    } else {
                        match NEIGBOURHOOD_OFFSETS
                            .iter()
                            .map(|&(ox, oy)| (x + ox, y + oy))
                            .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                            .filter(|&(x, y)| field[y as usize].as_bytes()[x as usize] == b'*')
                            .count()
                        {
                            0 => ' ',
                            n => (n as u8 + '0' as u8) as char,
                        }
                    }
                })
                .collect()
        })
        .collect()
}

// use std::str;

// pub fn annotate(minefield: &[&str]) -> Vec<String> {
//     let row = minefield.len();
//     let mut ans = Vec::new();
//     if row == 0 {
//         return ans;
//     }
//     let col = minefield[0].len();
//     if col == 0 {
//         for i in 0..row {
//             ans.push("".to_string());
//         }
//         return ans;
//     }

//     // Base 1d array
//     let mut grid_raw = vec![0; col * row];

//     // Vector of 'width' elements slices
//     let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(col).collect();

//     // Final 2d array `&mut [&mut [_]]`
//     let ms_arr = grid_base.as_mut_slice();

//     for i in 0..row {
//         for j in 0..col {
//             // println!("{:?}", minefield[i].to_string().as_bytes());
//             if minefield[i].to_string().as_bytes()[j] == b'*' {
//                 ms_arr[i][j] = b'*';
//                 if i > 0 {
//                     if j > 0 && ms_arr[i - 1][j - 1] != b'*' {
//                         ms_arr[i - 1][j - 1] += 1;
//                     }

//                     if ms_arr[i - 1][j] != b'*' {
//                         ms_arr[i - 1][j] += 1;
//                     }

//                     if j < (col - 1) && ms_arr[i - 1][j + 1] != b'*' {
//                         ms_arr[i - 1][j + 1] += 1;
//                     }
//                 }

//                 if j > 0 && ms_arr[i][j - 1] != b'*' {
//                     ms_arr[i][j - 1] += 1;
//                 }

//                 if j < (col - 1) && ms_arr[i][j + 1] != b'*' {
//                     ms_arr[i][j + 1] += 1;
//                 }

//                 if i < (row - 1) {
//                     if j > 0 && ms_arr[i + 1][j - 1] != b'*' {
//                         ms_arr[i + 1][j - 1] += 1;
//                     }

//                     if ms_arr[i + 1][j] != b'*' {
//                         ms_arr[i + 1][j] += 1;
//                     }

//                     if j < (col - 1) && ms_arr[i + 1][j + 1] != b'*' {
//                         ms_arr[i + 1][j + 1] += 1;
//                     }
//                 }
//             }
//         }
//     }

//     for i in 0..row {
//         for j in 0..col {
//             match ms_arr[i][j] {
//                 0 => ms_arr[i][j] = b' ',
//                 b'*' => ms_arr[i][j] = b'*',
//                 _ => ms_arr[i][j] += b'0',
//             }
//         }

//         ans.push(str::from_utf8_mut(ms_arr[i]).unwrap().to_string());
//     }

//     return ans;
// }
