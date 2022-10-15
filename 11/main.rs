use std::{error::Error, fs::read_to_string};

use nalgebra::DMatrix;

fn count_near_occupied_seats(seats: &DMatrix<char>, row: usize, column: usize) -> usize {
    let (rows, columns) = seats.shape();
    seats
        .slice(
            (
                if row > 0 { row - 1 } else { row },
                if column > 0 { column - 1 } else { column },
            ),
            (
                if row > 0 && row < rows - 1 { 3 } else { 2 },
                if column > 0 && column < columns - 1 {
                    3
                } else {
                    2
                },
            ),
        )
        .into_iter()
        .filter(|seat| **seat == '#')
        .count()
}

fn count_far_occupied_seats(seats: &DMatrix<char>, row: usize, column: usize) -> usize {
    let is_direction_occupied = |f: &dyn Fn(isize, isize) -> (isize, isize)| {
        let mut cur_position = (row as isize, column as isize);
        loop {
            let next_position = f(cur_position.0, cur_position.1);
            if next_position.0 < 0 || next_position.1 < 0 {
                return 0;
            }
            match seats.get((next_position.0 as usize, next_position.1 as usize)) {
                Some(&'#') => return 1,
                Some(&'L') | None => return 0,
                Some(_) => cur_position = next_position,
            }
        }
    };

    is_direction_occupied(&|row, column| (row - 1, column))
        + is_direction_occupied(&|row, column| (row - 1, column + 1))
        + is_direction_occupied(&|row, column| (row, column + 1))
        + is_direction_occupied(&|row, column| (row + 1, column + 1))
        + is_direction_occupied(&|row, column| (row + 1, column))
        + is_direction_occupied(&|row, column| (row + 1, column - 1))
        + is_direction_occupied(&|row, column| (row, column - 1))
        + is_direction_occupied(&|row, column| (row - 1, column - 1))
}

fn iterate_seats_once(
    seats: &DMatrix<char>,
    f: &dyn Fn(&DMatrix<char>, usize, usize) -> usize,
) -> DMatrix<char> {
    let (rows, columns) = seats.shape();
    DMatrix::from_fn(rows, columns, |row, column| {
        match seats.get((row, column)) {
            Some('L') if f(seats, row, column) == 0 => '#',
            Some('L') => 'L',
            Some('#') if f(seats, row, column) >= 5 => 'L',
            Some('#') => '#',
            Some('.') => '.',
            _ => unreachable!(),
        }
    })
}

fn iterate_seats(seats: &DMatrix<char>, f: &dyn Fn(&DMatrix<char>) -> DMatrix<char>) -> usize {
    let new_seats = f(seats);
    if seats == &new_seats {
        new_seats.into_iter().filter(|seat| **seat == '#').count()
    } else {
        iterate_seats(&new_seats, f)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let seat_lines = read_to_string("input.txt")?
        .split("\n")
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let seats = DMatrix::from_row_iterator(
        seat_lines.len(),
        seat_lines[0].len(),
        seat_lines.iter().flat_map(|seat_line| seat_line.chars()),
    );

    println!(
        "2020-12-11 Part 1: {}",
        iterate_seats(&seats, &|seats| iterate_seats_once(
            seats,
            &count_near_occupied_seats
        ))
    );
    println!(
        "2020-12-11 Part 2: {}",
        iterate_seats(&seats, &|seats| iterate_seats_once(
            seats,
            &count_far_occupied_seats
        ))
    );

    Ok(())
}
