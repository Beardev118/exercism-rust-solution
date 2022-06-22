pub mod macros;

fn main() {
    println!("Hello, world!");

    let _empty: ::std::collections::HashMap<(), ()> = hashmap!();
    let _without_comma = hashmap!(23=> 623, 34 => 21);
    let _with_trailing = hashmap!(23 => 623, 34 => 21,);
}
