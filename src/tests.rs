use itertools::Itertools;
use std::iter::repeat;
use Csv;

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

#[test]
fn basic_api() {
    let csv = create_buffer(20,20,100);
    for row in Csv::from_bytes(&*csv) {
        for cell in row {
            // do something
        }
    }
}
