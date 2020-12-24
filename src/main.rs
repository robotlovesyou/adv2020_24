use std::collections::HashSet;

fn main() {
    let black_tiles = get_black_tiles(include_str!("../input.txt").lines());
    println!("part 1: {}", black_tiles.len());

    let mut tile_game = TileGame::new(black_tiles);
    tile_game.advance_n(100);
    println!("part 2: {}", tile_game.black_tile_count());
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

use self::Direction::*;

struct Directions {
    source: Vec<char>,
    index: usize,
}

impl Directions {
    fn new(source: &str) -> Directions {
        Directions {
            source: source.chars().collect(),
            index: 0,
        }
    }

    fn coordinates(&mut self) -> (i64, i64) {
        let mut q = 0;
        let mut r = 0;
        for direction in self {
            match direction {
                East => q += 1,
                SouthEast => r += 1,
                SouthWest => {
                    q -= 1;
                    r += 1;
                }
                West => q -= 1,
                NorthWest => r -= 1,
                NorthEast => {
                    q += 1;
                    r -= 1;
                }
            }
        }

        (q, r)
    }
}

impl Iterator for Directions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.source.len() {
            return None;
        }
        let direction = match self.source[self.index] {
            'e' => East,
            'w' => West,
            's' => {
                self.index += 1;
                match self.source[self.index] {
                    'e' => SouthEast,
                    'w' => SouthWest,
                    other => panic!("invalid char '{}' after s", other),
                }
            }
            'n' => {
                self.index += 1;
                match self.source[self.index] {
                    'e' => NorthEast,
                    'w' => NorthWest,
                    other => panic!("invalid char '{}' after n", other),
                }
            }
            other => panic!("invalid char '{}'", other),
        };
        self.index += 1;
        Some(direction)
    }
}

fn get_black_tiles<'a>(instructions: impl Iterator<Item = &'a str>) -> HashSet<(i64, i64)> {
    let mut black_tiles = HashSet::new();

    for path in instructions {
        let coordinates = path.to_directions().coordinates();
        if black_tiles.contains(&coordinates) {
            black_tiles.remove(&coordinates);
        } else {
            black_tiles.insert(coordinates);
        }
    }

    black_tiles
}

trait ToDirections {
    fn to_directions(&self) -> Directions;
}

impl ToDirections for &str {
    fn to_directions(&self) -> Directions {
        Directions::new(self)
    }
}

struct TileGame {
    state: HashSet<(i64, i64)>,
}

impl TileGame {
    fn new(state: HashSet<(i64, i64)>) -> TileGame {
        TileGame { state }
    }

    fn adjacent_coordinates(&self, coordinates: (i64, i64)) -> Vec<(i64, i64)> {
        let (q, r) = coordinates;
        let mut adjacent = Vec::with_capacity(6);

        adjacent.push((q + 1, r - 1));
        adjacent.push((q + 1, r));
        adjacent.push((q, r + 1));
        adjacent.push((q - 1, r + 1));
        adjacent.push((q - 1, r));
        adjacent.push((q, r - 1));
        adjacent
    }

    fn white_tiles_to_test(&self) -> HashSet<(i64, i64)> {
        let mut to_test = HashSet::new();
        for (q, r) in self.state.iter().cloned() {
            for coordinate in self.adjacent_coordinates((q, r)).into_iter() {
                if !self.state.contains(&coordinate) {
                    to_test.insert(coordinate);
                }
            }
        }
        to_test
    }

    fn adjacent_black_tile_count(&self, coordinate: (i64, i64)) -> usize {
        let (q, r) = coordinate;
        let mut adjacent = 0;
        for to_test in [
            (q + 1, r - 1),
            (q + 1, r),
            (q, r + 1),
            (q - 1, r + 1),
            (q - 1, r),
            (q, r - 1),
        ]
        .iter()
        {
            if self.state.contains(to_test) {
                adjacent += 1;
            }
        }

        adjacent
    }

    fn advance(&mut self) {
        let mut new_state = HashSet::new();

        for tile in self.state.iter() {
            let adjacent = self.adjacent_black_tile_count(*tile);
            if adjacent == 1 || adjacent == 2 {
                new_state.insert(*tile);
            }
        }

        for tile in self.white_tiles_to_test().iter() {
            let adjacent = self.adjacent_black_tile_count(*tile);
            if adjacent == 2 {
                new_state.insert(*tile);
            }
        }
        self.state = new_state;
    }

    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    fn black_tile_count(&self) -> usize {
        self.state.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INSTRUCTIONS: &str = indoc! {"
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew
        "};

    #[test]
    fn returns_the_correct_direction_sequence() {
        let directions = "ewseswnenw".to_directions().collect::<Vec<Direction>>();
        assert_eq!(
            &[East, West, SouthEast, SouthWest, NorthEast, NorthWest],
            &directions[..]
        );
    }

    #[test]
    fn returns_the_correct_coordinates() {
        let coordinates = "newswseenew".to_directions().coordinates();
        assert_eq!((0, 0), coordinates);

        let coordinates = "nenenewwwswswswseseseeeenenene"
            .to_directions()
            .coordinates();
        assert_eq!((3, 0), coordinates);
    }

    #[test]
    fn it_correctly_collects_the_coordinates_of_black_tiles() {
        let black_tiles = get_black_tiles(TEST_INSTRUCTIONS.lines());
        assert_eq!(10, black_tiles.len());
    }

    #[test]
    fn it_correctly_calculates_the_number_of_black_tiles_after_1_day() {
        let black_tiles = get_black_tiles(TEST_INSTRUCTIONS.lines());
        let mut tile_game = TileGame::new(black_tiles);
        tile_game.advance();
        assert_eq!(15, tile_game.black_tile_count());
    }

    #[test]
    fn it_correctly_calculates_the_number_of_tiles_after_n_days() {
        let black_tiles = get_black_tiles(TEST_INSTRUCTIONS.lines());
        let mut tile_game = TileGame::new(black_tiles);
        tile_game.advance_n(10);
        assert_eq!(37, tile_game.black_tile_count());

        tile_game.advance_n(10);
        assert_eq!(132, tile_game.black_tile_count());

        tile_game.advance_n(80);
        assert_eq!(2208, tile_game.black_tile_count());
    }
}
