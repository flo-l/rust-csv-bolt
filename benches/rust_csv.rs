#[macro_use] extern crate criterion;
extern crate itertools;
extern crate csv;

use criterion::Criterion;
use itertools::Itertools;
use std::iter::repeat;

fn create_buffer(row_count: usize, col_count: usize, cell_count: usize) -> Vec<u8> {
    static COL_SEP: &[u8] = b",";
    static ROW_SEP: &[u8] = b"\n";
    static CELL_CONTENT: u8 = b'X';

    let row_seps = repeat(ROW_SEP).take(row_count - 1);
    let col_seps = repeat(COL_SEP).take(col_count - 1);

    let cell: Vec<u8> = repeat(CELL_CONTENT).take(cell_count).collect();
    let row:  Vec<u8> = Itertools::flatten(repeat(&*cell).interleave_shortest(col_seps)).cloned().collect();
    let csv:  Vec<u8> = Itertools::flatten(repeat(&*row).interleave_shortest(row_seps)).cloned().collect();

    csv
}

fn rust_csv(b: &mut criterion::Bencher, (cells, data): &(usize, Vec<u8>)) {
    b.iter(|| {
        let rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .terminator(csv::Terminator::Any(b'\n'))
            .has_headers(false)
            .from_reader(&**data);

        let it = rdr.into_byte_records();
        let n: usize = it.map(|row| row.unwrap().into_iter().count()).sum();
        assert_eq!(*cells, n);
    });
}

fn csv_bolt(b: &mut criterion::Bencher, (_cells, data): &(usize, Vec<u8>)) {
    b.iter(|| {
        for x in data {
            criterion::black_box(x);
        }
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    // init csv file in memory
    let rows = 1024;
    let cols = 10;
    let cells = rows * cols;
    let data = create_buffer(rows, cols, 1024);

    let funs = vec![
        criterion::Fun::new("rust_csv", rust_csv),
        criterion::Fun::new("csv_bolt", csv_bolt)
    ];

    c.bench_functions("10_mb_file_without_quotes", funs, (cells, data));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
