use anyhow::Result;
use std::env;
use std::fmt::Write;

fn fizzbuzz(n: u64) -> Result<Vec<String>> {
    let mut result = vec![];
    result.reserve(n as usize);

    for k in 1 .. n+1 {
        let mut s = String::new();
        if k % 15 == 0 {
            write!(s, "fizz buz")?;
        } else if k % 5 == 0 {
            write!(s, "buz")?;
        } else if k % 3 == 0 {
            write!(s, "fizz")?;
        } else {
            write!(s, "{}", k)?;
        }
        result.push(s);
    }

    Ok(result)
}

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
