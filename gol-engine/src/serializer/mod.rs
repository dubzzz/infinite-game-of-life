pub struct Serializer {}

impl Serializer {
    fn row_to_string<T>(row: &Vec<T>, extrator: fn(&T) -> &str) -> String {
        row.iter()
            .map(|cell| extrator(cell))
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn serialize<T>(grid: &Vec<Vec<T>>, extrator: fn(&T) -> &str) -> String {
        grid.iter()
            .map(|row| -> String { Serializer::row_to_string(row, extrator) })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
