use std::process;

mod cli;
mod pseudo;
mod strategies;

fn main() {
    let args = cli::parse();

    match pseudo::run(&args) {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {e}");
            let code = match &e {
                pseudo::PseudoError::AmbiguousFormat { .. } => 2,
                pseudo::PseudoError::InvalidArgs(_) => 2,
                _ => 1,
            };
            process::exit(code);
        }
    }
}
