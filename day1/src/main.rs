use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut existing: Vec<i32> = vec![0];
    let mut initial_frequency: i32 = 0;
    let mut loops = 1;
    let mut f = File::open("input.txt")?;
    let mut input = String::new();

    f.read_to_string(&mut input)?;

    loop {
        match run(&input, &mut initial_frequency, &mut existing) {
            Some(frequency_seen_twice) => {
                println!(
                    "The frequency seen twice is: {}. Seen in loop {}",
                    frequency_seen_twice, loops
                );
                break;
            }
            None => {
                loops += 1;
                continue;
            }
        }
    }

    Ok(())
}

fn run(input: &str, frequency: &mut i32, existing: &mut Vec<i32>) -> Option<i32> {
    for line in input.lines() {
        if let Ok(inc) = line.parse::<i32>() {
            *frequency += inc;

            // println!("Searching {} in {:?}", frequency, existing);

            match existing.binary_search(&frequency) {
                Ok(_) => return Some(*frequency),
                Err(index) => existing.insert(index, *frequency),
            }
        }
    }

    None
}
