use std::{cell::RefCell, collections::HashSet, error::Error, rc::Rc};

struct Cup {
    number: usize,
    next: Option<Rc<RefCell<Cup>>>,
    pred: Option<Rc<RefCell<Cup>>>,
}

impl Cup {
    fn new(number: usize) -> Self {
        Self {
            number,
            next: None,
            pred: None,
        }
    }

    fn lock_in(&self) -> (Rc<RefCell<Cup>>, HashSet<usize>) {
        let mut locked_numbers = HashSet::new();
        let mut previous_locked_node = None;
        let mut locked_node = self.next.clone().unwrap();
        for _ in 0..3 {
            locked_numbers.insert(locked_node.borrow().number);
            let next_locked_node = locked_node.borrow().next.clone().unwrap();
            previous_locked_node = Some(locked_node);
            locked_node = next_locked_node;
        }
        (previous_locked_node.unwrap(), locked_numbers)
    }
}

struct Cups(Rc<RefCell<Cup>>);

impl Cups {
    fn parse(text: &str) -> Self {
        let cups = (1..=text.len())
            .map(|number| Rc::new(RefCell::new(Cup::new(number))))
            .collect::<Vec<_>>();
        let cups_len = cups.len();

        for (index, cup) in cups.iter().enumerate() {
            cup.borrow_mut().pred = Some(cups[(cups_len + index - 1) % cups_len].clone());
        }

        let mut first_cup = None;
        let mut last_cup = None;
        for char in text.chars().rev() {
            let number = char.to_digit(10).unwrap() as usize;
            cups[number - 1].borrow_mut().next = first_cup;
            first_cup = Some(cups[number - 1].clone());
            if last_cup.is_none() {
                last_cup = first_cup.clone();
            }
        }
        last_cup.unwrap().borrow_mut().next = first_cup.clone();

        Self(first_cup.unwrap())
    }

    fn locate(&self, number: usize, check_next: bool) -> Rc<RefCell<Cup>> {
        let mut node = self.0.clone();
        while !check_next && node.borrow().number != number
            || check_next && node.borrow().next.clone().unwrap().borrow().number != number
        {
            let next_node = node.borrow().next.clone().unwrap();
            node = next_node;
        }
        node
    }

    fn perform_move(&mut self) {
        let (last_locked_node, locked_numbers) = self.0.borrow().lock_in();

        let mut target_cup = self.0.borrow().pred.clone().unwrap();
        while locked_numbers.contains(&target_cup.borrow().number) {
            let next_target_cup = target_cup.borrow().pred.clone().unwrap();
            target_cup = next_target_cup;
        }

        let first_locked_node = self.0.borrow().next.clone().unwrap();
        let after_target_cup = target_cup.borrow().next.clone().unwrap();
        self.0.borrow_mut().next = last_locked_node.borrow().next.clone();
        target_cup.borrow_mut().next = Some(first_locked_node);
        last_locked_node.borrow_mut().next = Some(after_target_cup);
        let next_current_cup = self.0.borrow().next.clone().unwrap();
        self.0 = next_current_cup;
    }

    fn extend(&mut self) {
        let mut last_node = self.locate(self.0.borrow().number, true);
        let mut pred_node = self.locate(9, false);
        for number in 10..=1_000_000 {
            let node = Rc::new(RefCell::new(Cup::new(number)));
            node.borrow_mut().pred = Some(pred_node);
            last_node.borrow_mut().next = Some(node.clone());
            pred_node = node.clone();
            last_node = node;
        }
        last_node.borrow_mut().next = Some(self.0.clone());
        let node_1 = self.locate(1, false);
        node_1.borrow_mut().pred = Some(last_node);
    }

    fn get_labels_from_1(&self) -> String {
        let mut node_1 = self.locate(1, false);
        let mut label_string = "".to_string();
        while node_1.borrow().next.clone().unwrap().borrow().number != 1 {
            let next_node_1 = node_1.borrow().next.clone().unwrap();
            label_string += &next_node_1.borrow().number.to_string();
            node_1 = next_node_1;
        }
        label_string
    }

    fn clockwise_from_1(&self) -> usize {
        let node_1 = self.locate(1, false);
        let node_1_neighbor_1 = node_1.borrow().next.clone().unwrap();
        let node_1_neighbor_2 = node_1_neighbor_1.borrow().next.clone().unwrap();
        let n1 = node_1_neighbor_1.borrow().number;
        let n2 = node_1_neighbor_2.borrow().number;
        n1 * n2
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cup_string = "586439172";

    let mut few_cups = Cups::parse(cup_string);
    for _ in 0..100 {
        few_cups.perform_move();
    }
    println!("2020-12-23 Part 1: {}", few_cups.get_labels_from_1());

    let mut many_cups = Cups::parse(cup_string);
    many_cups.extend();
    for _ in 0..10_000_000 {
        many_cups.perform_move();
    }
    println!("2020-12-23 Part 2: {}", many_cups.clockwise_from_1());
    Ok(())
}
