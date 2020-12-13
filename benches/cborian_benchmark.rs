// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

use cborian::{random::gen_value, Config, GenericDecoder, GenericEncoder};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quickcheck::StdGen;
use std::io::Cursor;

fn make_value(min: usize) -> Vec<u8> {
    let mut generator = StdGen::new(rand::thread_rng(), 20);
    let mut encoder = GenericEncoder::new(Cursor::new(Vec::new()));
    encoder.borrow_mut().array(min).unwrap();
    for _ in 0..min {
        encoder.value(&gen_value(3, &mut generator)).unwrap()
    }
    encoder.into_inner().into_writer().into_inner()
}

fn cborian_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("make_value=30", |bench| {
        let mut cursor = Cursor::new(make_value(black_box(30)));
        bench.iter(|| {
            assert!(GenericDecoder::new(Config::default(), &mut cursor)
                .value()
                .ok()
                .is_some());
            cursor.set_position(0);
        })
    });
}

criterion_group!(benches, cborian_benchmark);
criterion_main!(benches);
