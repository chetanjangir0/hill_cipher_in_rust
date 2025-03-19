fn check_key_validity(key: &Vec<Vec<i32>>) -> Result<(), &'static str> {
    let n = key.len();
    if n == 0 {
        return Err("Error: key is empty");
    }
    
    if !key.iter().all(|row| row.len() == n) {
        return Err("Error: key matrix should be a square matrix");
    }

    Ok(())
}


fn mod_26(key: &mut Vec<Vec<i32>>) {
    for row in key.iter_mut() {
        for elem in row.iter_mut() {
            *elem = (*elem % 26 + 26) % 26;
        }
    }
}
pub fn encode_hill(text: &str, mut key: Vec<Vec<i32>>) -> Result<String, &'static str>{
    check_key_validity(&key)?;
    mod_26(&mut key);

    Ok(String::from("ok"))


}

// pub fn decode_hill() {}




#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn empty_key() {
        assert_eq!(encode_hill("test", vec![]), Err("Error: key is empty"));
    }

    #[test]
    fn non_square_key() {
        assert_eq!(encode_hill("test", vec![vec![1, 2], vec![0]]), Err("Error: key matrix should be a square matrix"));
    }

    #[test]
    fn mod_26_negative_vals() {
        let mut key = vec![vec![-90, -57], vec![12, 33]];
        mod_26(&mut key);
        assert_eq!(
            key, 
            vec![vec![14, 21], vec![12, 7]]
        );
    }


}