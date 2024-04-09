pub struct Window {}

impl Window {
    pub fn scan<T, S, F>(
        row_index_start: i8,
        column_index_start: i8,
        row_index_end: i8,
        column_index_end: i8,
        value: &S,
        extract: F,
    ) -> Vec<Vec<T>>
    where
        F: Fn(&S, i8, i8) -> T,
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

    pub fn neighborhood(row_index: i8, column_index: i8) -> [(i8, i8); 9] {
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

#[cfg(test)]
mod tests {}