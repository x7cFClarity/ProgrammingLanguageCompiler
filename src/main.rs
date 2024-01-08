use programming_language:: string_keyed_record_searcher:: StringKeyedRecordSearcher;

fn main() {
    let mut skrs = StringKeyedRecordSearcher::default();

    skrs.set_record("abc", 10);
    let record = skrs.find_mutably("abc", false);

    println!("ABC record: {}", record.unwrap().record.unwrap());
}