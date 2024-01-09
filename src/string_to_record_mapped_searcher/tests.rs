use crate::string_to_record_mapped_searcher::StringToRecordMappedSearcher;

#[test]
fn setting_records() {
    let mut string_to_record_mapped_searcher = StringToRecordMappedSearcher::default();

    // Set records for testing
    let first_record = 10;
    let second_record = 20;
    let third_record = 100;

    string_to_record_mapped_searcher.set_record("hello", first_record);
    string_to_record_mapped_searcher.set_record("world", second_record);
    string_to_record_mapped_searcher.set_record("help", third_record);

    // Fetch records
    let first_fetched_record = string_to_record_mapped_searcher.get_record("hello");
    assert!(first_fetched_record.is_some());
    assert_eq!(first_record, first_fetched_record.unwrap());

    let second_fetched_record = string_to_record_mapped_searcher.get_record("world");
    assert!(second_fetched_record.is_some());
    assert_eq!(second_record, second_fetched_record.unwrap());

    let third_fetched_record = string_to_record_mapped_searcher.get_record("help");
    assert!(third_fetched_record.is_some());
    assert_eq!(third_record, third_fetched_record.unwrap());
}