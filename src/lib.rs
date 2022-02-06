use anyhow::Result;

pub fn fizzbuzz(n: u64) -> Result<Vec<String>> {
    let result = (1 .. n+1).map(|k| {
        if k % 15 == 0 {
            "fizz buzz".to_string()
        } else if k % 5 == 0 {
            "buzz".to_string()
        } else if k % 3 == 0 {
            "fizz".to_string()
        } else {
            k.to_string()
        }
    }).collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_15() {
        let result = super::fizzbuzz(15);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(15, result.len());

        for k in 1 .. 16usize {
            let value = &result[k - 1];

            if k % 15 == 0 {
                assert_eq!("fizz buz", value);
            } else if k % 5 == 0 {
                assert_eq!("buz", value);
            } else if k % 3 == 0 {
                assert_eq!("fizz", value);
            } else {
                assert_eq!(&k.to_string(), value);
            }
        }
    }
}
