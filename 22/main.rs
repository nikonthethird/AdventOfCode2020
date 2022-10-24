use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::read_to_string,
};

fn play_and_score(init_p1_cards: &VecDeque<u8>, init_p2_cards: &VecDeque<u8>, recurse: bool) {
    let (_, winning_deck) = play(init_p1_cards.clone(), init_p2_cards.clone(), recurse);
    let score = winning_deck
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, card)| acc + (index + 1) * (*card as usize));
    println!(
        "2020-12-22 Part {}: {score}",
        if recurse { "2" } else { "1" }
    );
}

fn play(
    mut p1_cards: VecDeque<u8>,
    mut p2_cards: VecDeque<u8>,
    recurse: bool,
) -> (bool, VecDeque<u8>) {
    let mut deck_hashes = HashSet::new();
    loop {
        if recurse && !deck_hashes.insert((p1_cards.clone(), p2_cards.clone()))
            || p2_cards.len() == 0
        {
            return (true, p1_cards);
        } else if p1_cards.len() == 0 {
            return (false, p2_cards);
        }

        let (p1_card, p2_card) = (p1_cards.pop_front().unwrap(), p2_cards.pop_front().unwrap());
        if recurse && p1_cards.len() >= p1_card as usize && p2_cards.len() >= p2_card as usize {
            let new_p1_cards = p1_cards.iter().take(p1_card as usize).copied().collect();
            let new_p2_cards = p2_cards.iter().take(p2_card as usize).copied().collect();
            if play(new_p1_cards, new_p2_cards, true).0 {
                p1_cards.push_back(p1_card);
                p1_cards.push_back(p2_card);
            } else {
                p2_cards.push_back(p2_card);
                p2_cards.push_back(p1_card);
            }
        } else if p1_card > p2_card {
            p1_cards.push_back(p1_card.max(p2_card));
            p1_cards.push_back(p1_card.min(p2_card));
        } else {
            p2_cards.push_back(p1_card.max(p2_card));
            p2_cards.push_back(p1_card.min(p2_card));
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (init_p1_cards, init_p2_cards) = read_to_string("input.txt")?
        .split_once("\n\n")
        .map(|(p1_text, p2_text)| {
            (
                p1_text
                    .split('\n')
                    .skip(1)
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<VecDeque<_>>(),
                p2_text
                    .split('\n')
                    .skip(1)
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<VecDeque<_>>(),
            )
        })
        .unwrap();

    play_and_score(&init_p1_cards, &init_p2_cards, false);
    play_and_score(&init_p1_cards, &init_p2_cards, true);

    Ok(())
}
