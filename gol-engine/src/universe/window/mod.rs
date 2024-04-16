mod tests;

pub struct Window {}

impl Window {
    pub fn scan<T, S, F>(
        row_index_start: isize,
        column_index_start: isize,
        row_index_end: isize,
        column_index_end: isize,
        value: &S,
        extract: F,
    ) -> Vec<Vec<T>>
    where
        F: Fn(&S, isize, isize) -> T,
    {
        let mut content = Vec::new();
        for row_index in row_index_start..=row_index_end {
            let mut content_row = Vec::new();
            for column_index in column_index_start..=column_index_end {
                content_row.push(extract(value, row_index, column_index))
            }
            content.push(content_row);
        }
        content
    }

    pub fn neighborhood(row_index: isize, column_index: isize) -> [(isize, isize); 9] {
        [
            (
                row_index.overflowing_sub(1).0,
                column_index.overflowing_sub(1).0,
            ),
            (row_index.overflowing_sub(1).0, column_index),
            (
                row_index.overflowing_sub(1).0,
                column_index.overflowing_add(1).0,
            ),
            (row_index, column_index.overflowing_sub(1).0),
            (row_index, column_index),
            (row_index, column_index.overflowing_add(1).0),
            (
                row_index.overflowing_add(1).0,
                column_index.overflowing_sub(1).0,
            ),
            (row_index.overflowing_add(1).0, column_index),
            (
                row_index.overflowing_add(1).0,
                column_index.overflowing_add(1).0,
            ),
        ]
    }
}
