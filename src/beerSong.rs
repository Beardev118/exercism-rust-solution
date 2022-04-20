pub fn verse(n: u32) -> String {
    match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        _ => format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\nTake {} down and pass it around, {} bottle{} of beer on the wall.\n", n, match n { 2..=99 => "s", _ => ""}, n, match n { 2..=99 => "s", _ => ""}, match n{ 1 => "it", _ => "one"},  match n { 1 => "no more".to_string(), _ => format!("{}", n-1)}, match n { 2 => "", _ => "s"} )
    }
}

pub fn sing(from: u32, to: u32) -> String {
    (to..from + 1)
        .rev()
        .map(|x| verse(x))
        .collect::<Vec<_>>()
        .join("\n")
}
