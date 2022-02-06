use rust_fizz_buzz::fizzbuzz;

use anyhow::Result;
use clap::{app_from_crate, arg};
use std::env;

fn main() -> Result<()>{
    let matches = app_from_crate!()
        .arg(
            arg!([N] "Print the first N fizz buzzes")
            .required(true)
        )
        .get_matches();

    if let Some(n) = matches.value_of("N") {
        let n: u64 = n.parse()?;
        let result = fizzbuzz(n)?;
        for line in result {
            println!("{0}", line);
        }
    }

    Ok(())
}
