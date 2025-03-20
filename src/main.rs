use nalgebra::DMatrix;

fn check_key_validity(key: &DMatrix<f64>) -> Result<(), &'static str> {
    if key.nrows() == 0 {
        return Err("Error: key matrix is empty");
    }
    
    if !(key.nrows() == key.ncols()) {
        return Err("Error: key matrix should be a square matrix");
    }

    if key.determinant() == 0.0 {
        return Err("Error: key matrix should be invertible");
    }

    Ok(())
}

fn text_to_numbers(text: &str) -> Vec<u8> {
    text.chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                Some(c.to_ascii_uppercase() as u8 - b'A')
            } else {
                None
            }
        })
        .collect()
}

fn numbers_to_text(nums: &[u8]) -> String {
    nums.iter()
        .map(|&n| (n + b'A') as char)
        .collect()
}


fn mod_26(key: &mut DMatrix<f64>) {
    for val in key.iter_mut() {
        *val = val.rem_euclid(26.0);
    }
}

fn process_chunks(text_vector: Vec<u8>, key: &DMatrix<f64>) -> Vec<u8> {
    let mut result_data = Vec::new();
    let chunk_size = key.ncols();
    
    for chunk in text_vector.chunks(chunk_size) {
        let mut text_chunk = vec![0.0; chunk_size]; // padding if chunk is small
        for (i, &num) in chunk.iter().enumerate() {
            text_chunk[i] = num as f64;
        }
        let text_chunk = DMatrix::from_vec(chunk_size, 1, text_chunk);
        let mut result = key * text_chunk;
        mod_26(&mut result);
        result_data.extend(result.iter().map(|&x| x as u8));
    }
    
    result_data
}


pub fn encode_hill(text: &str, mut key: DMatrix<f64>) -> Result<String, &'static str> {
    check_key_validity(&key)?;
    mod_26(&mut key);

    let text_vector = text_to_numbers(text);
    let encrypted_data = process_chunks(text_vector, &key);
    Ok(numbers_to_text(&encrypted_data))

}

pub fn decode_hill(text: &str, mut key: DMatrix<f64>) -> Result<String, &'static str> {
    check_key_validity(&key)?;
    mod_26(&mut key);

    let mut inversed_key = match key.try_inverse() {
        Some(inversed) => inversed,
        None => return Err("Error: key matrix is not invertible")
    };
    mod_26(&mut inversed_key);

    let text_vector = text_to_numbers(text);
    let decrypted_data = process_chunks(text_vector, &inversed_key);
    Ok(numbers_to_text(&decrypted_data))
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_key_should_fail() {
        assert_eq!(
            encode_hill("test", DMatrix::zeros(0, 0)), 
            Err("Error: key matrix is empty")
        );
    }

    #[test]
    fn test_non_square_key_should_fail() {
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0
        ];
        assert_eq!(
            encode_hill("test", DMatrix::from_row_slice(2, 3, &data)), 
            Err("Error: key matrix should be a square matrix")
        );
    }

    #[test]
    fn test_mod_26_applies_correctly() {
        let data = vec![-90.0, -57.0, 12.0, 33.0];
        let mut key = DMatrix::from_row_slice(2, 2, &data);
        mod_26(&mut key);
        assert_eq!(
            key, 
            DMatrix::from_row_slice(2, 2, &vec![14.0, 21.0, 12.0, 7.0])
        );
    }

    #[test]
    fn test_non_invertible_matrix_should_fail() {
        assert_eq!(
            encode_hill("test", DMatrix::from_row_slice(2, 2, &vec![2.0, 4.0, 1.0, 2.0])),
            Err("Error: key matrix should be invertible")
        );
    }

    #[test]
    fn test_text_to_numbers_conversion() {
        assert_eq!(
            text_to_numbers("123 ABC xyz!?"),
            vec![0, 1, 2, 23, 24, 25]
        );
    }

    #[test]
    fn test_numbers_to_text_conversion() {
        assert_eq!(
            numbers_to_text(&vec![0, 1, 2, 23, 24, 25]),
            "ABCXYZ"
        );
    }

    #[test]
    fn test_encoding_with_valid_key() {
        let key_data = vec![6.0, 24.0, 1.0, 13.0];
        let key = DMatrix::from_row_slice(2, 2, &key_data);
        let result = encode_hill("HELP", key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decoding_with_valid_key() {
        let key_data = vec![6.0, 24.0, 1.0, 13.0];
        let key = DMatrix::from_row_slice(2, 2, &key_data);
        let encoded_text = encode_hill("HELP", key.clone()).unwrap();
        let decoded_text = decode_hill(&encoded_text, key).unwrap();
        assert_eq!(decoded_text, "HELP");
    }

    #[test]
    fn test_encoding_with_padding() {
        let key_data = vec![3.0, 3.0, 2.0, 5.0];
        let key = DMatrix::from_row_slice(2, 2, &key_data);
        let result = encode_hill("ABC", key);
        assert!(result.is_ok());
    }
}