use crosscard_interview::fizzbuzz;

use anyhow::Result;
use std::env;

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();

    if let Some(pos_arg) = args.get(1) {
        let n: u64 = pos_arg.parse()?;
        let result = fizzbuzz(n)?;
        for line in result {
            println!("{0}", line);
        }
    } else {
        println!("Usage {0} <n: u64>", args.get(0).unwrap_or(&"fizzbuzz".to_string()));
    }

    Ok(())
}
