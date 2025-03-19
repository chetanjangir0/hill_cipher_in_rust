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

// fn text_to_numbers(&str) -> Vec<u8> {

// }


fn mod_26(key: &mut DMatrix<f64>) {
    for val in key.iter_mut() {
        *val = val.rem_euclid(26.0);
    }
}


pub fn encode_hill(text: &str, mut key: DMatrix<f64>) -> Result<String, &'static str>{
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
        assert_eq!(
            encode_hill("test", DMatrix::zeros(0, 0)), 
            Err("Error: key matrix is empty")
        );
    }

    #[test]
    fn non_square_key() {
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
    fn mod_26_negative_vals() {
        let data = vec![-90.0, -57.0, 12.0, 33.0];
        let mut key = DMatrix::from_row_slice(2, 2, &data);
        mod_26(&mut key);
        assert_eq!(
            key, 
            DMatrix::from_row_slice(2, 2, &vec![14.0, 21.0, 12.0, 7.0])
        );
    }

    #[test]
    fn non_invertible_matrix() {
        assert_eq!(
            encode_hill("test", DMatrix::from_row_slice(2, 2, &vec![2.0, 4.0, 1.0, 2.0])),
            Err("Error: key matrix should be invertible")
        );
    }


}