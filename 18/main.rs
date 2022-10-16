use std::{collections::VecDeque, error::Error, fs::read_to_string, iter::once};

#[derive(Clone)]
enum Term {
    NumberChars(Vec<char>),
    Number(i64),
    Mark,
    Plus(i64),
    Times(i64),
}

fn evaluate_formula(formula: &str, hi_pred_plus: bool) -> i64 {
    let mut formula_chars = formula
        .replace(' ', "")
        .chars()
        .chain(once(' '))
        .collect::<VecDeque<_>>();
    let mut queue = VecDeque::new();

    while let Some(formula_char) = formula_chars.pop_front() {
        if formula_char.is_ascii_digit() {
            match queue.get_mut(0) {
                Some(Term::NumberChars(number_chars)) => {
                    number_chars.push(formula_char);
                    continue;
                }
                _ => {
                    queue.push_front(Term::NumberChars(vec![formula_char]));
                    continue;
                }
            }
        } else if let Some(Term::NumberChars(number_chars)) = queue.get(0) {
            let number = number_chars
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            queue.pop_front();
            if let Some(Term::Plus(summand)) = queue.get(0).cloned() {
                queue.pop_front();
                if let (true, Some(Term::Times(factor)), true) =
                    (hi_pred_plus, queue.get(0).cloned(), formula_char != '+')
                {
                    queue.pop_front();
                    queue.push_front(Term::Number(factor * (summand + number)));
                } else {
                    queue.push_front(Term::Number(summand + number));
                }
            } else if let Some(Term::Times(factor)) = queue.get(0).cloned() {
                if hi_pred_plus && formula_char == '+' {
                    queue.push_front(Term::Number(number));
                } else {
                    queue.pop_front();
                    queue.push_front(Term::Number(factor * number));
                }
            } else {
                queue.push_front(Term::Number(number));
            }
        }

        match formula_char {
            '(' => queue.push_front(Term::Mark),
            ')' => {
                if let (Some(Term::Number(number)), Some(Term::Mark)) =
                    (queue.pop_front(), queue.pop_front())
                {
                    if let Some(Term::Plus(summand)) = queue.get(0).cloned() {
                        queue.pop_front();
                        if let (true, Some(Term::Times(factor)), true) = (
                            hi_pred_plus,
                            queue.get(0).cloned(),
                            formula_chars.get(0) != Some(&'+'),
                        ) {
                            queue.pop_front();
                            queue.push_front(Term::Number(factor * (summand + number)));
                        } else {
                            queue.push_front(Term::Number(summand + number));
                        }
                    } else if let Some(Term::Times(factor)) = queue.get(0).cloned() {
                        if hi_pred_plus && formula_chars.get(0) == Some(&'+') {
                            queue.push_front(Term::Number(number));
                        } else {
                            queue.pop_front();
                            queue.push_front(Term::Number(factor * number));
                        }
                    } else {
                        queue.push_front(Term::Number(number));
                    }
                }
            }
            '+' => {
                if let Some(Term::Number(number)) = queue.pop_front() {
                    queue.push_front(Term::Plus(number));
                }
            }
            '*' => {
                if let Some(Term::Number(number)) = queue.pop_front() {
                    queue.push_front(Term::Times(number));
                }
            }
            ' ' => (),
            _ => unreachable!(),
        }
    }

    match queue.pop_front() {
        Some(Term::Number(number)) => number,
        _ => unreachable!(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let formulas = read_to_string("input.txt")?
        .split("\n")
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let part1 = formulas
        .iter()
        .map(|formula| evaluate_formula(formula, false))
        .sum::<i64>();
    println!("2020-12-18 Part 1: {part1}");

    let part2 = formulas
        .iter()
        .map(|formula| evaluate_formula(formula, true))
        .sum::<i64>();
    Ok(println!("2020-12-18 Part 2: {part2}"))
}
