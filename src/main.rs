use std::process;

pub const VERSION: &str = "1.0.0";

// A Result shorthand for the CLI integration
type Result<T> = std::result::Result<T, &'static str>;

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
