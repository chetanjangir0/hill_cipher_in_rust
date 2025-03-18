

pub fn encode_hill(text: &str, keyMatrix: Vec<Vec<i32>>) -> Result<String, &'static str>{
    let n = keyMatrix.len();
    if n == 0 {
        return Err("Error: key is empty");
    }

    if !keyMatrix.iter().all(|row| row.len() == n) {
        return Err("Error: key matrix should be a square matrix");
    }

    Ok(String::from("ok"))


}

// pub fn decode_hill() {}




#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn empty_key() {
        assert_eq!(encode_hill("test", vec![]), Err("Error: key is empty"))
    }

}