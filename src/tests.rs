#[cfg(test)]
mod tests {
    use crate::ui;

    use std::fs;
    use std::path::Path;
    use ui::generate_receipt;
    use std::fs::write;
    use std::io::ErrorKind;
    use crate::data_processing::read_csv_file;

    /// Test case generates a mock receipt and check whether the feceipt file is created and its
    /// constents match the expected output.
    #[test]
    fn test_generate_receipt() {
        let patron_name = "Test Patron".to_string();
        let amount_owed = "10.00".to_string();
        let card_number = "1234567890123456".to_string();
        let card_expiration = "12/23".to_string();
        let card_cvv = "123".to_string();

        generate_receipt(
            patron_name.clone(),
            amount_owed.clone(),
            card_number.clone(),
            card_expiration.clone(),
            card_cvv.clone(),
        );

        let receipt_count = fs::read_dir("receipts")
            .expect("Could not read receipts directory")
            .count();

        let receipt_file_name = format!("receipts/receipt{}.txt", receipt_count);
        assert!(Path::new(&receipt_file_name).exists());

        let contents =
            fs::read_to_string(receipt_file_name).expect("Something went wrong reading the file");

        let expected_receipt = format!(
            "Transaction ID: #{}\nName: {}\nAmount Charge: ${}\nCard Number: {}\nCard Expiration: {}\nCard CVV: {}",
            receipt_count, patron_name, amount_owed, card_number, card_expiration, card_cvv
        );
        assert_eq!(contents, expected_receipt);
    }

        #[test]
    fn test_read_csv_file() {
        // Arrange
        let test_file_content = "col1,col2,col3\nval1,val2,val3\n";
        write("./menus/test.csv", test_file_content).unwrap();

        // Act
        let result = read_csv_file("./menus/test.csv");

        // Assert
        match result {
            Ok(dm) => {
                assert_eq!(dm.get("col1"), Some(&vec!["val1".to_string()]));
                assert_eq!(dm.get("col2"), Some(&vec!["val2".to_string()]));
                assert_eq!(dm.get("col3"), Some(&vec!["val3".to_string()]));
            }
            Err(e) => panic!("Error reading CSV file: {}", e),
        }

        // Clean up
        std::fs::remove_file("./menus/test.csv").unwrap();
    }

    #[test]
    fn test_read_csv_file_fail() {
        // Act
        let result = read_csv_file("./menus/non_existent.csv");

        // Assert
           match result {
            Ok(_) => panic!("Should have failed because the file doesn't exist."),
            Err(e) => {
                let io_error = e.downcast::<std::io::Error>().unwrap();
                assert_eq!(io_error.kind(), ErrorKind::NotFound)
            }
        }
    }
}
