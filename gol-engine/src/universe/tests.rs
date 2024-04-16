#[cfg(test)]
mod next_gen {
    use crate::universe::{Status, Universe};

    #[test]
    fn should_converge_to_a_square() {
        let mut universe = Universe::new();
        universe.set_alive(1, 1);
        universe.set_alive(1, 2);
        universe.set_alive(2, 1);
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Alive, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(0, 0, 4, 3), expected_universe);

        let universe = universe.next_gen();
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Alive, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Alive, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(0, 0, 4, 3), expected_universe);

        let universe = universe.next_gen();
        assert_eq!(universe.window(0, 0, 4, 3), expected_universe);
    }

    #[test]
    fn should_converge_to_nothing_after_passing_to_one_cell() {
        let mut universe = Universe::new();
        universe.set_alive(1, 1);
        universe.set_alive(1, 3);
        universe.set_alive(3, 1);
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Dead, Status::Alive],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(0, 0, 4, 3), expected_universe);

        let universe = universe.next_gen();
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Alive, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(0, 0, 4, 3), expected_universe);

        let universe = universe.next_gen();
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(0, 0, 4, 3), expected_universe);

        let universe = universe.next_gen();
        assert_eq!(universe.window(0, 0, 4, 3), expected_universe);
    }

    #[test]
    fn should_handle_highly_spread_envs() {
        let mut universe = Universe::new();
        universe.set_alive(isize::min_value(), isize::min_value());
        universe.set_alive(0, 0);
        universe.set_alive(isize::max_value(), isize::max_value());
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Alive, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(-1, -1, 1, 1), expected_universe);

        let universe = universe.next_gen();
        let expected_universe = vec![
            vec![Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead],
            vec![Status::Dead, Status::Dead, Status::Dead],
        ];
        assert_eq!(universe.window(-1, -1, 1, 1), expected_universe);

        let universe = universe.next_gen();
        assert_eq!(universe.window(-1, -1, 1, 1), expected_universe);
    }

    #[test]
    fn should_join_the_extreme_parts() {
        let mut universe = Universe::new();
        // left branch
        universe.set_alive(isize::min_value(), isize::max_value() - 3);
        universe.set_alive(isize::min_value(), isize::max_value() - 2);
        universe.set_alive(isize::min_value(), isize::max_value() - 1);
        // right branch
        universe.set_alive(isize::min_value(), isize::min_value() + 2);
        universe.set_alive(isize::min_value(), isize::min_value() + 3);
        universe.set_alive(isize::min_value(), isize::min_value() + 4);
        // top branch
        universe.set_alive(isize::max_value() - 3, isize::min_value());
        universe.set_alive(isize::max_value() - 2, isize::min_value());
        universe.set_alive(isize::max_value() - 1, isize::min_value());
        // bottom branch
        universe.set_alive(isize::min_value() + 2, isize::min_value());
        universe.set_alive(isize::min_value() + 3, isize::min_value());
        universe.set_alive(isize::min_value() + 4, isize::min_value());
        // ....#....
        // ....#....
        // ....#....
        // ...x.....
        // ###.y.###
        // .........
        // ....#....
        // ....#....
        // ....#....
        // with: x = bottom right
        // with: y = top left
        let expected_universe = vec![
            vec![
                Status::Dead,
                Status::Dead,
                Status::Alive,
                Status::Alive,
                Status::Alive,
            ],
            vec![
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
            vec![
                Status::Alive,
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
            vec![
                Status::Alive,
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
            vec![
                Status::Alive,
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
        ];
        assert_eq!(
            universe.window(
                isize::min_value(),
                isize::min_value(),
                isize::min_value() + 4,
                isize::min_value() + 4,
            ),
            expected_universe
        );

        let universe = universe.next_gen();
        // .........
        // ...###...
        // .........
        // .#.x...#.
        // .#..y..#.
        // .#.....#.
        // .........
        // ...###...
        // .........
        // with: x = bottom right
        // with: y = top left
        let expected_universe = vec![
            vec![
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Alive,
                Status::Dead,
            ],
            vec![
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Alive,
                Status::Dead,
            ],
            vec![
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
            vec![
                Status::Alive,
                Status::Alive,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
            vec![
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
        ];
        assert_eq!(
            universe.window(
                isize::min_value(),
                isize::min_value(),
                isize::min_value() + 4,
                isize::min_value() + 4,
            ),
            expected_universe
        );

        let universe = universe.next_gen();
        // ....#....
        // ....#....
        // ....#....
        // ...x.....
        // ###.y.###
        // .........
        // ....#....
        // ....#....
        // ....#....
        // with: x = bottom right
        // with: y = top left
        let expected_universe = vec![
            vec![
                Status::Dead,
                Status::Dead,
                Status::Alive,
                Status::Alive,
                Status::Alive,
            ],
            vec![
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
            vec![
                Status::Alive,
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
            vec![
                Status::Alive,
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
            vec![
                Status::Alive,
                Status::Dead,
                Status::Dead,
                Status::Dead,
                Status::Dead,
            ],
        ];
        assert_eq!(
            universe.window(
                isize::min_value(),
                isize::min_value(),
                isize::min_value() + 4,
                isize::min_value() + 4,
            ),
            expected_universe
        );
    }
}
