use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut input = String::new();

    f.read_to_string(&mut input)?;

    let (box_two_letters, box_three_letters) = count_boxes(&input);
    println!("The checksum is: {}", box_two_letters * box_three_letters);

    Ok(())
}

fn count_letters(word: &str) -> HashMap<char, i32> {
    let mut result: HashMap<char, i32> = HashMap::new();

    for letter in word.chars() {
        let count = match result.get(&letter) {
            Some(occurrences) => occurrences + 1,
            None => 1,
        };
        result.insert(letter, count);
    }

    result
}

fn count_boxes(input: &str) -> (i32, i32) {
    let mut box_two_letters = 0;
    let mut box_three_letters = 0;

    for line in input.lines() {
        let result = count_letters(line);
        let occurrences: Vec<&i32> = result.values().collect();

        match (occurrences.contains(&&2), occurrences.contains(&&3)) {
            (true, true) => {
                box_two_letters += 1;
                box_three_letters += 1;
            }
            (true, false) => box_two_letters += 1,
            (false, true) => box_three_letters += 1,
            (_, _) => (),
        };
    }

    (box_two_letters, box_three_letters)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_letters() {
        let result = count_letters("bababc");

        assert_eq!(result.get(&'a').unwrap(), &2);
        assert_eq!(result.get(&'b').unwrap(), &3);
        assert!(result.values().collect::<Vec<&i32>>().contains(&&2));
    }

    #[test]
    fn test_count_boxes() {
        let input = String::from(
            "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab",
        );
        let (box_two_letters, box_three_letters) = count_boxes(&input);
        assert_eq!(box_two_letters, 4);
        assert_eq!(box_three_letters, 3);
    }
}
