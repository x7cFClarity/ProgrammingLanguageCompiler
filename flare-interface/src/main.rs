use flare_lib::record_searcher::StringSearcher;

fn main() {
    println!("--- Flare compiler version {}", env!("CARGO_PKG_VERSION"));

    let mut searcher: StringSearcher<usize> = StringSearcher::new();

    searcher.set_record("abc", 10);
}
