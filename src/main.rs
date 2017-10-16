extern crate rff;

fn main() {
    let lines = rff::stdin::slurp();
    println!("Lines:\n{:?}", lines);
}
