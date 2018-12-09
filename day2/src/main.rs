use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut input = String::new();

    f.read_to_string(&mut input)?;

    // let (box_two_letters, box_three_letters) = count_boxes(&input);
    // println!("The checksum is: {}", box_two_letters * box_three_letters);

    let (word_1, word_2) = correct_boxes(&input);
    println!("The correct boxes are '{}' and '{}'", word_1, word_2);
    println!(
        "Letters in common: '{}'",
        letters_in_common(&word_1, &word_2)
    );

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

fn words_diff(word_1: &str, word_2: &str) -> usize {
    let letters_word_1: Vec<i8> = word_1.chars().map(|letter| letter as i8).collect();
    let letters_word_2: Vec<i8> = word_2.chars().map(|letter| letter as i8).collect();

    let letters_diff: Vec<i8> = letters_word_1
        .iter()
        .zip(letters_word_2)
        .map(|(letter_1, letter_2)| letter_1 - letter_2)
        .collect();

    letters_diff.iter().filter(|score| score != &&0).count()
}

fn correct_boxes(input: &str) -> (String, String) {
    let lines: Vec<&str> = input.lines().into_iter().collect();
    let lines_count = lines.len();
    let mut minimum_score: Option<usize> = None;
    let mut word_1: String = String::from(lines[0]);
    let mut word_2: String = String::from(lines[1]);

    for i in 0..lines_count {
        for j in (i + 1)..lines_count {
            let score = words_diff(&lines[i], &lines[j]);
            match minimum_score {
                Some(min) => {
                    if score < min {
                        minimum_score = Some(score);
                        word_1 = String::from(lines[i]);
                        word_2 = String::from(lines[j]);
                    }
                }
                None => {
                    minimum_score = Some(score);
                    word_1 = String::from(lines[i]);
                    word_2 = String::from(lines[j]);
                }
            }
        }
    }
    (word_1, word_2)
}

fn letters_in_common(word_1: &str, word_2: &str) -> String {
    let letters_word_1: Vec<char> = word_1.chars().collect();
    let letters_word_2: Vec<char> = word_2.chars().collect();

    let letters_diff: Vec<u8> = letters_word_1
        .iter()
        .zip(letters_word_2)
        .filter(|(letter_1, letter_2)| *letter_1 == letter_2)
        .map(|(letter_1, _)| *letter_1 as u8)
        .collect();

    String::from_utf8(letters_diff).unwrap()
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

    #[test]
    fn test_words_diff() {
        let word_1 = "abcde";
        let word_2 = "axcye";

        assert_eq!(words_diff(&word_1, &word_2), 2);

        let word_1 = "fghij";
        let word_2 = "fguij";

        assert_eq!(words_diff(&word_1, &word_2), 1);
    }

    #[test]
    fn test_correct_boxes() {
        let input = String::from(
            "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz",
        );

        let (box_1, box_2) = correct_boxes(&input);

        assert_eq!(box_1, "fghij");
        assert_eq!(box_2, "fguij");
    }

    #[test]
    fn test_letters_in_common() {
        let word_1 = "fghij";
        let word_2 = "fguij";

        assert_eq!(letters_in_common(word_1, word_2), "fgij");
    }
}
