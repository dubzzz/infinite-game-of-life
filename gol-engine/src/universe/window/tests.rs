#[cfg(test)]
mod neighborhood {
    use crate::universe::window::Window;

    #[test]
    fn should_compute_neighbors_of_origin() {
        let neighbors = Window::neighborhood(0, 0);
        let expected_neighbors = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        assert_eq!(neighbors, expected_neighbors);
    }

    #[test]
    fn should_compute_neighbors_of_any_cell_not_on_diagonals() {
        let neighbors = Window::neighborhood(11, 5);
        let expected_neighbors = [
            (10, 4),
            (10, 5),
            (10, 6),
            (11, 4),
            (11, 5),
            (11, 6),
            (12, 4),
            (12, 5),
            (12, 6),
        ];
        assert_eq!(neighbors, expected_neighbors);
    }

    #[test]
    fn should_compute_neighbors_of_top_left() {
        let neighbors = Window::neighborhood(isize::min_value(), isize::min_value());
        let expected_neighbors = [
            (isize::max_value(), isize::max_value()),
            (isize::max_value(), isize::min_value()),
            (isize::max_value(), isize::min_value() + 1),
            (isize::min_value(), isize::max_value()),
            (isize::min_value(), isize::min_value()),
            (isize::min_value(), isize::min_value() + 1),
            (isize::min_value() + 1, isize::max_value()),
            (isize::min_value() + 1, isize::min_value()),
            (isize::min_value() + 1, isize::min_value() + 1),
        ];
        assert_eq!(neighbors, expected_neighbors);
    }

    #[test]
    fn should_compute_neighbors_of_bottom_right() {
        let neighbors = Window::neighborhood(isize::max_value(), isize::max_value());
        let expected_neighbors = [
            (isize::max_value() - 1, isize::max_value() - 1),
            (isize::max_value() - 1, isize::max_value()),
            (isize::max_value() - 1, isize::min_value()),
            (isize::max_value(), isize::max_value() - 1),
            (isize::max_value(), isize::max_value()),
            (isize::max_value(), isize::min_value()),
            (isize::min_value(), isize::max_value() - 1),
            (isize::min_value(), isize::max_value()),
            (isize::min_value(), isize::min_value()),
        ];
        assert_eq!(neighbors, expected_neighbors);
    }
}
