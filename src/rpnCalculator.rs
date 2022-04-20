use crate::CalculatorInput::{Add, Divide, Multiply, Subtract, Value};
#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let v: Vec<i32> = vec![];
    let mut result = inputs.iter().fold(v, |mut stack, input| {
        if let Some(new) = match input {
            Add => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b + a))),
            Subtract => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b - a))),
            Multiply => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b * a))),
            Divide => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b / a))),
            Value(value) => Some(*value),
        } {
            stack.push(new);
        }
        stack
    });
    result
        .pop()
        .and_then(|x| if result.is_empty() { Some(x) } else { None })
}

// #[derive(Debug)]
// pub enum CalculatorInput {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
//     Value(i32),
// }

// pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
//     let mut calc_stack: Vec<i32> = vec![];
//     if inputs.len() == 0 {
//         None
//     } else {
//         for i in inputs.iter() {
//             match i {
//                 CalculatorInput::Value(x) => {
//                     calc_stack.push(*x);
//                 }
//                 _ => {
//                     if calc_stack.len() < 2 {
//                         return None;
//                     } else {
//                         let ele2 = calc_stack.pop().unwrap();
//                         let ele1 = calc_stack.pop().unwrap();
//                         let ans = calculation(&ele1, &ele2, i);
//                         match ans {
//                             None => return None,
//                             Some(x) => calc_stack.push(x),
//                         }
//                     }
//                 }
//             }
//         }

//         if calc_stack.len() == 1 {
//             Some(calc_stack.pop().unwrap())
//         } else {
//             None
//         }
//     }
// }

// fn calculation(ele1: &i32, ele2: &i32, opr: &CalculatorInput) -> Option<i32> {
//     match opr {
//         CalculatorInput::Add => Some(ele1 + ele2),
//         CalculatorInput::Subtract => Some(ele1 - ele2),
//         CalculatorInput::Multiply => Some(ele1 * ele2),
//         CalculatorInput::Divide => {
//             if *ele2 == 0 {
//                 None
//             } else {
//                 Some(ele1 / ele2)
//             }
//         }
//         _ => None,
//     }
// }
