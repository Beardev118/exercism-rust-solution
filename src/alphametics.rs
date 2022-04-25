use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let split = input.split(" == ");
    let mut input_temp = Vec::new();
    for s in split {
        input_temp.push(s);
    }

    let split_word = input_temp[0].split(" + ");
    let mut input_word = Vec::new();
    for w in split_word {
        input_word.push(w);
    }

    let chrs = input
        .chars()
        .filter(|&c| c.is_ascii() && c.is_alphabetic())
        .collect::<HashSet<char>>();

    println!("{:?}", chrs);

    let perms = (0..10).permutations(chrs.len());
    for i in perms.into_iter() {
        if i[0] == 0 {
            continue;
        }
        let mut add_vals = Vec::new();
        let mut flag = true;

        input_word.clone().into_iter().for_each(|add_str| {
            let mut temp = 0;

            for chr in add_str.chars() {
                let idx = get_index(chrs.clone(), chr);
                temp = temp * 10 + i[idx];
                if temp == 0 {
                    flag = false;
                    break;
                }
            }
            add_vals.push(temp);
        });

        if !flag {
            continue;
        }

        let total: i32 = add_vals.iter().sum();

        let mut ans = 0;
        for chr in input_temp[1].chars() {
            let idx = get_index(chrs.clone(), chr);
            ans = ans * 10 + i[idx];
            if ans == 0 {
                flag = false;
            }
        }

        if (total == ans) && (flag == true) {
            let mut pass = HashMap::new();
            let mut idx = 0;
            chrs.clone().into_iter().for_each(|x| {
                pass.insert(x, i[idx] as u8);
                idx += 1;
            });

            println!("{:?}", pass);
            return Some(pass);
        }
    }

    return None;
}

pub fn get_index(chrs: HashSet<char>, chr: char) -> usize {
    chrs.into_iter()
        .position(|x| x == chr) // x -> accessing the element value directly
        .unwrap()
}
