use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs::read_to_string,
};

use lazy_static::lazy_static;
use regex::Regex;

struct Food {
    ingredients: BTreeSet<String>,
    allergens: BTreeSet<String>,
}

impl Food {
    fn parse(text: &str) -> Self {
        lazy_static! {
            static ref FOOD_REGEX: Regex =
                Regex::new(r"^(?P<ingredients>[^(]+) \(contains (?P<allergens>[^)]+)\)$").unwrap();
        }
        let food_match = FOOD_REGEX.captures(text).unwrap();
        let ingredients = food_match["ingredients"]
            .split(' ')
            .map(ToString::to_string)
            .collect();
        let allergens = food_match["allergens"]
            .split(", ")
            .map(ToString::to_string)
            .collect();
        Self {
            ingredients,
            allergens,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let foods = read_to_string("input.txt")?
        .split('\n')
        .map(Food::parse)
        .collect::<Vec<_>>();

    let mut allergen_ingredient_map =
        foods
            .iter()
            .fold(BTreeMap::<_, BTreeSet<_>>::new(), |map, food| {
                food.allergens.iter().fold(map, |mut map, allergen| {
                    map.entry(allergen)
                        .and_modify(|ingredients| {
                            *ingredients = ingredients
                                .intersection(&food.ingredients)
                                .cloned()
                                .collect()
                        })
                        .or_insert(food.ingredients.clone());
                    map
                })
            });

    let safe_ingredient_count = foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|ingredient| {
            allergen_ingredient_map
                .values()
                .all(|allergen_ingredients| !allergen_ingredients.contains(*ingredient))
        })
        .count();
    println!("2020-12-21 Part 1: {safe_ingredient_count}");

    let mut ingredient_allergen_map = BTreeMap::<_, _>::new();
    while let Some((allergen, ingredient)) =
        allergen_ingredient_map
            .iter()
            .find_map(|(allergen, ingredients)| {
                if ingredients.len() == 1 {
                    Some((
                        (*allergen).clone(),
                        ingredients.iter().next().unwrap().clone(),
                    ))
                } else {
                    None
                }
            })
    {
        allergen_ingredient_map.remove(&allergen);
        for (_, ingredients) in allergen_ingredient_map.iter_mut() {
            ingredients.remove(&ingredient);
        }
        ingredient_allergen_map.insert(allergen, ingredient);
    }

    Ok(println!(
        "2020-12-21 Part 2: {}",
        ingredient_allergen_map
            .values()
            .cloned()
            .collect::<Vec<_>>()
            .join(",")
    ))
}
