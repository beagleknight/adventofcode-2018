extern crate regex;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use regex::Regex;

fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut input = String::new();

    f.read_to_string(&mut input)?;

    let lines: Vec<&str> = input.lines().into_iter().collect();
    let lines_count = lines.len();
    let mut inches = 0;

    for i in 0..lines_count {
        for j in (i + 1)..lines_count {
            let rect_1 = Rectangle::new(&lines[i].to_string());
            let rect_2 = Rectangle::new(&lines[j].to_string());

            if rectangle_overlap(&rect_1, &rect_2) {
                inches += inches_within(&rect_1, &rect_2);
            }
        }
    }

    println!("Total inches within: {}", inches);

    Ok(())
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn new(input: &String) -> Rectangle {
        // let input_1 = String::from("#1 @ 1,3: 4x4");
        // let input_2 = String::from("#2 @ 3,1: 4x4");

        let re = Regex::new(r"#(.*) @ (.*),(.*): (.*)x(.*)").unwrap();
        let cap = re.captures(input).unwrap();

        let x_1: i32 = cap[2].parse().unwrap();
        let y_1: i32 = cap[3].parse().unwrap();
        let width: i32 = cap[4].parse().unwrap();
        let height: i32 = cap[5].parse().unwrap();

        Rectangle {
            p1: Point {
                x: x_1,
                y: height - y_1 + 1,
            },
            p2: Point {
                x: x_1 + width - 1,
                y: height - y_1,
            },
        }
    }
}

fn rectangle_overlap(rect_1: &Rectangle, rect_2: &Rectangle) -> bool {
    let Rectangle { p1: p1_1, p2: p1_2 } = rect_1;
    let Rectangle { p1: p2_1, p2: p2_2 } = rect_2;

    p1_1.x < p2_2.x && p2_1.x < p1_2.x && p1_1.y > p2_2.y && p2_1.y > p1_2.y
}

fn inches_within(rect_1: &Rectangle, rect_2: &Rectangle) -> i32 {
    let Rectangle { p1: p1_1, p2: p1_2 } = rect_1;
    let Rectangle { p1: p2_1, p2: p2_2 } = rect_2;
    let within_rect_width = match p1_2.x > p2_2.x {
        true => p2_2.x - p1_1.x,
        false => p1_2.x - p2_1.x,
    };
    let within_rect_height = match p1_2.y < p2_2.y {
        true => (p2_2.y - p1_1.y).abs(),
        false => (p1_2.y - p2_1.y).abs(),
    };

    within_rect_width * within_rect_height
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rectangle_overlap() {
        let rect_1 = Rectangle {
            p1: Point { x: 2, y: 5 },
            p2: Point { x: 5, y: 2 },
        };

        let rect_2 = Rectangle {
            p1: Point { x: 1, y: 3 },
            p2: Point { x: 4, y: 1 },
        };

        let rect_3 = Rectangle {
            p1: Point { x: 5, y: 3 },
            p2: Point { x: 6, y: 1 },
        };

        assert!(
            rectangle_overlap(&rect_1, &rect_2),
            "Rectangle 1 doesn't overlap with Rectangle 2"
        );
        assert!(
            !rectangle_overlap(&rect_1, &rect_3),
            "Rectangle 1 does overlap with Rectangle 3"
        );
        assert!(!rectangle_overlap(
            &rect_2,
            &rect_3)
            "Rectangle 2 does overlap with Rectangle 3"
        );
    }

    #[test]
    fn test_inches_within() {
        let rect_1 = Rectangle {
            p1: Point { x: 1, y: 5 },
            p2: Point { x: 5, y: 1 },
        };

        let rect_2 = Rectangle {
            p1: Point { x: 3, y: 6 },
            p2: Point { x: 6, y: 3 },
        };

        assert_eq!(inches_within(&rect_1, &rect_2), 4);
    }

    #[test]
    fn build_rectangle_from_string() {
        let input = String::from("#1 @ 1,3: 4x4");
        let Rectangle { p1, p2 } = Rectangle::new(&input);

        assert_eq!(p1.x, 1);
        assert_eq!(p1.y, 4);
        assert_eq!(p2.x, 4);
        assert_eq!(p2.y, 1);
    }

    #[test]
    fn test_overlap_with_parse() {
        let input_1 = String::from("#1 @ 1,3: 4x4");
        let input_2 = String::from("#2 @ 3,1: 4x4");
        let rect_1 = Rectangle::new(&input_1);
        let rect_2 = Rectangle::new(&input_2);

        println!("Rect 1: {:?}", &rect_1);
        println!("Rect 2: {:?}", &rect_2);
        assert!(rectangle_overlap(&rect_1, &rect_2));
        assert_eq!(inches_within(&rect_1, &rect_2), 4);
    }
}
