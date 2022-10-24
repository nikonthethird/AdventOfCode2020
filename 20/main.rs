use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs::read_to_string,
    iter::once,
};

use lazy_static::lazy_static;
use regex::Regex;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, EnumIter, Eq, Hash, PartialEq)]
enum Direction {
    Top,
    Left,
    Bottom,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Left => Direction::Right,
            Direction::Bottom => Direction::Top,
            Direction::Right => Direction::Left,
        }
    }

    fn coords(&self, x: i16, y: i16) -> (i16, i16) {
        match self {
            Direction::Top => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Right => (x + 1, y),
        }
    }
}

#[derive(Clone, Copy, EnumIter)]
enum Operation {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
    FlipRot0,
    FlipRot90,
    FlipRot180,
    FlipRot270,
}

struct Tile(u16, Vec<char>);

impl Tile {
    fn parse(text: &str) -> Self {
        lazy_static! {
            static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^Tile (?P<number>\d+):$").unwrap();
        }
        let (identifier_text, contents) = text.split_once('\n').unwrap();
        let identifier = IDENTIFIER_REGEX.captures(identifier_text).unwrap()["number"]
            .parse::<u16>()
            .unwrap();
        Self(identifier, contents.replace('\n', "").chars().collect())
    }

    fn transform(&self, operation: Operation) -> Self {
        fn rot90(chars: &Vec<char>) -> Vec<char> {
            (0..10)
                .flat_map(|o1| (0..=90).rev().step_by(10).map(move |o2| chars[o1 + o2]))
                .collect()
        }
        fn flip(chars: &Vec<char>) -> Vec<char> {
            (0..=90)
                .rev()
                .step_by(10)
                .flat_map(|o1| (0..10).map(move |o2| chars[o1 + o2]))
                .collect()
        }
        Self(
            self.0,
            match operation {
                Operation::Rot0 => self.1.clone(),
                Operation::Rot90 => rot90(&self.1),
                Operation::Rot180 => rot90(&rot90(&self.1)),
                Operation::Rot270 => rot90(&rot90(&rot90(&self.1))),
                Operation::FlipRot0 => flip(&self.1),
                Operation::FlipRot90 => rot90(&flip(&self.1)),
                Operation::FlipRot180 => rot90(&rot90(&flip(&self.1))),
                Operation::FlipRot270 => rot90(&rot90(&rot90(&flip(&self.1)))),
            },
        )
    }

    fn border(&self, direction: Direction) -> String {
        match direction {
            Direction::Top => (0..10).map(|o| self.1[o]).collect(),
            Direction::Left => (0..=90).step_by(10).map(|o| self.1[o]).collect(),
            Direction::Bottom => (90..100).map(|o| self.1[o]).collect(),
            Direction::Right => (9..=99).step_by(10).map(|o| self.1[o]).collect(),
        }
    }

    fn find_neighbor<'tile>(
        &self,
        direction: Direction,
        mut tiles: impl Iterator<Item = &'tile Tile>,
    ) -> Option<Tile> {
        let border_to_match = self.border(direction);
        tiles.find_map(|tile| {
            Operation::iter().find_map(|operation| {
                let tile_to_match = tile.transform(operation);
                if border_to_match == tile_to_match.border(direction.opposite()) {
                    Some(tile_to_match)
                } else {
                    None
                }
            })
        })
    }
}

struct PlacedTile(Tile, i16, i16);

fn monster_coords(xo: usize, yo: usize) -> Vec<(usize, usize)> {
    vec![
        (xo + 18, yo),
        (xo, yo + 1),
        (xo + 5, yo + 1),
        (xo + 6, yo + 1),
        (xo + 11, yo + 1),
        (xo + 12, yo + 1),
        (xo + 17, yo + 1),
        (xo + 18, yo + 1),
        (xo + 19, yo + 1),
        (xo + 1, yo + 2),
        (xo + 4, yo + 2),
        (xo + 7, yo + 2),
        (xo + 10, yo + 2),
        (xo + 13, yo + 2),
        (xo + 16, yo + 2),
    ]
}

fn mark_monster(image_lines: &mut Vec<Vec<(char, bool)>>) -> bool {
    let mut marked = false;
    for y in 0..image_lines.len() - 3 {
        for x in 0..image_lines[0].len() - 20 {
            let monster_coords = monster_coords(x, y);
            if monster_coords
                .iter()
                .all(|(mx, my)| image_lines[*my][*mx].0 == '#')
            {
                marked = true;
                for (mx, my) in monster_coords.into_iter() {
                    image_lines[my][mx].1 = true;
                }
            }
        }
    }
    marked
}

fn flip_image_lines(image_lines: &Vec<Vec<(char, bool)>>) -> Vec<Vec<(char, bool)>> {
    image_lines.iter().rev().cloned().collect()
}

fn rotate_image_lines(image_lines: &Vec<Vec<(char, bool)>>) -> Vec<Vec<(char, bool)>> {
    (0..image_lines.len())
        .map(|o1| {
            (0..image_lines.len())
                .rev()
                .map(move |o2| image_lines[o2][o1].clone())
                .collect()
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut tiles = read_to_string("input.txt")?
        .split("\n\n")
        .map(Tile::parse)
        .map(|tile| (tile.0, tile))
        .collect::<BTreeMap<_, _>>();

    let initial_tile_number = *tiles.keys().next().unwrap();
    let mut placed_tiles = BTreeMap::<_, _>::from_iter(
        once(tiles.remove(&initial_tile_number).unwrap())
            .map(|tile| (tile.0, PlacedTile(tile, 0, 0))),
    );
    let mut placed_coords = BTreeSet::<_>::from_iter(vec![(0, 0)]);

    while tiles.len() > 0 {
        let (anchor_number, direction, placed_tile) = 'placed_tile: loop {
            for anchor in placed_tiles.values() {
                for direction in Direction::iter()
                    .filter(|d| !placed_coords.contains(&d.coords(anchor.1, anchor.2)))
                {
                    if let Some(neighbor) = anchor.0.find_neighbor(direction, tiles.values()) {
                        break 'placed_tile (anchor.0 .0, direction, neighbor);
                    }
                }
            }
        };

        let coords = direction.coords(
            placed_tiles[&anchor_number].1,
            placed_tiles[&anchor_number].2,
        );
        tiles.remove(&placed_tile.0);
        placed_tiles.insert(placed_tile.0, PlacedTile(placed_tile, coords.0, coords.1));
        placed_coords.insert(coords);
    }

    let min_x = placed_tiles
        .values()
        .map(|placed_tile| placed_tile.1)
        .min()
        .unwrap();
    let max_x = placed_tiles
        .values()
        .map(|placed_tile| placed_tile.1)
        .max()
        .unwrap();
    let min_y = placed_tiles
        .values()
        .map(|placed_tile| placed_tile.2)
        .min()
        .unwrap();
    let max_y = placed_tiles
        .values()
        .map(|placed_tile| placed_tile.2)
        .max()
        .unwrap();

    let coord_num = |x, y| {
        placed_tiles
            .values()
            .find(|placed_tile| placed_tile.1 == x && placed_tile.2 == y)
            .unwrap()
            .0
             .0 as u64
    };

    println!(
        "2020-12-20 Part 1: {}",
        coord_num(min_x, min_y)
            * coord_num(min_x, max_y)
            * coord_num(max_x, min_y)
            * coord_num(max_x, max_y)
    );

    let mut image_line_map = BTreeMap::<_, Vec<_>>::new();
    for ry in min_y..=max_y {
        for dy in 0..8 {
            let line_to_fill = image_line_map.entry(ry * 8 + dy).or_default();
            for rx in min_x..=max_x {
                let tile = placed_tiles
                    .values()
                    .find(|tile| tile.1 == rx && tile.2 == ry)
                    .unwrap();
                for dx in 0..8 {
                    line_to_fill.push((tile.0 .1[1 + dx as usize + (1 + dy as usize) * 10], false));
                }
            }
        }
    }

    let mut unordered_image_lines = image_line_map.into_iter().collect::<Vec<_>>();
    unordered_image_lines.sort_by_key(|(line, _)| *line);

    let image_lines = unordered_image_lines
        .into_iter()
        .map(|(_, line)| line)
        .collect::<Vec<_>>();

    for rotation_count in 0..=3 {
        for flip_count in 0..=1 {
            let mut adjusted_image_lines = image_lines.clone();
            for _ in 0..rotation_count {
                adjusted_image_lines = rotate_image_lines(&adjusted_image_lines);
            }
            for _ in 0..flip_count {
                adjusted_image_lines = flip_image_lines(&adjusted_image_lines);
            }
            if mark_monster(&mut adjusted_image_lines) {
                let unmarked_count = adjusted_image_lines
                    .iter()
                    .flat_map(|line| line.iter())
                    .filter(|(char, marked)| char == &'#' && !marked)
                    .count();
                println!("2020-12-20 Part 2: {unmarked_count}");
                return Ok(());
            }
        }
    }

    Ok(())
}
