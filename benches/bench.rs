use chimtool::chim_core::entry::CEEntry;
use std::collections::HashMap;

#[macro_use]
extern crate bencher;
use bencher::Bencher;

fn cedict_to_hashmap(bench: &mut Bencher) {
    bench.iter(|| {
        let fl = std::fs::File::open("./res/cedict_1_0_ts_utf-8_mdbg.txt").unwrap();
        let parsed = cedict::parse_reader(fl);

        let _hmm: HashMap<String, CEEntry> = parsed
            .map(|x| (x.simplified().to_string(), CEEntry::from(x)))
            .collect();
    })
}

benchmark_group!(benches, cedict_to_hashmap);
benchmark_main!(benches);
