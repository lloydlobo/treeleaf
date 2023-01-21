use dialogue::*;

fn main() {
    match run() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("called `Result::unwrap()` on an `Err` value: {}", &e);
            std::process::exit(1);
        }
    }
}
