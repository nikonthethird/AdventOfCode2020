use std::error::Error;

fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut result = 1;
    for _ in 0..loop_size {
        result = (result * subject_number) % 20201227;
    }
    result
}

fn find_loop_size(target_key: usize) -> usize {
    let mut key = 1;
    let mut loop_size = 0;
    while key != target_key {
        loop_size += 1;
        key = (key * 7) % 20201227;
    }
    loop_size
}

fn main() -> Result<(), Box<dyn Error>> {
    let key = transform(16915772, find_loop_size(18447943));
    Ok(println!("2020-12-25: {key}"))
}
