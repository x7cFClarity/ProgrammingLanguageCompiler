use programming_language::string_to_record_mapped_searcher::StringToRecordMappedSearcher;

fn main() {
    let mut strms = StringToRecordMappedSearcher::default();

    {
        strms.set_record("abc", 10);
        let record = strms.get_mutable_node("abc", false);
        println!("ABC record: {}", record.unwrap().record.unwrap());
    }

    strms.set_record("abc", 100);
    let record = strms.get_mutable_node("abc", false);
    println!("ABC record: {}", record.unwrap().record.unwrap());
}