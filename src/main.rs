pub mod pascalsTriangle;

fn main() {
    println!("Hello, world!");

    let pt = pascalsTriangle::PascalsTriangle::new(15);
    println!("{:?}", pt.rows());
}
