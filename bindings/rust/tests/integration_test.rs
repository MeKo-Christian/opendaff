//! Integration tests for OpenDAFF Rust bindings
//!
//! Note: These tests require actual DAFF files to run.
//! Place test files in the testdata/ directory to enable these tests.

use opendaff::Reader;

#[test]
fn test_reader_creation() {
    let reader = Reader::new();
    assert!(reader.is_ok(), "Failed to create reader");

    let reader = reader.unwrap();
    assert!(!reader.is_valid(), "Reader should not be valid without file");
}

#[test]
fn test_reader_invalid_file() {
    let mut reader = Reader::new().unwrap();
    let result = reader.open_file("nonexistent_file.daff");
    assert!(result.is_err(), "Should fail to open non-existent file");
}

// Integration tests with actual files would go here
// Uncomment and add test files to enable

/*
#[test]
fn test_open_ir_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::new()?;
    reader.open_file("testdata/impulse_response.daff")?;

    assert!(reader.is_valid());
    assert_eq!(reader.content_type(), ContentType::ImpulseResponse);
    assert!(reader.num_channels() > 0);
    assert!(reader.num_records() > 0);

    let ir = reader.content_ir()?;
    assert!(ir.filter_length() > 0);
    assert!(ir.samplerate() > 0);

    Ok(())
}
*/
