extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use regex::Regex;

fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut input = String::new();

    f.read_to_string(&mut input)?;

    let mut fabrics = Vec::new();
    for line in input.lines() {
        fabrics.push(Fabric::new(&line.to_string()));
    }

    let result = overlap_count(&fabrics);

    println!("Total overlap count: {}", result);

    Ok(())
}

#[derive(Debug)]
struct Fabric {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Fabric {
    fn new(input: &String) -> Fabric {
        let re = Regex::new(r"#(.*) @ (.*),(.*): (.*)x(.*)").unwrap();
        let cap = re.captures(input).unwrap();

        let x: u32 = cap[2].parse().unwrap();
        let y: u32 = cap[3].parse().unwrap();
        let width: u32 = cap[4].parse().unwrap();
        let height: u32 = cap[5].parse().unwrap();

        Fabric {
            x,
            y,
            width,
            height,
        }
    }
}

fn overlap_count(fabrics: &[Fabric]) -> usize {
    let mut overlap: HashMap<(u32, u32), u32> = HashMap::new();

    for fabric in fabrics.iter() {
        for i in fabric.x..fabric.x + fabric.width {
            for j in fabric.y..fabric.y + fabric.height {
                let entry = overlap.entry((i, j)).or_insert(0);
                *entry += 1;
            }
        }
    }

    overlap.values().filter(|value| *value > &1).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fabric_from_string() {
        let input = String::from("#1 @ 1,3: 4x4");
        let Fabric {
            x,
            y,
            width,
            height,
        } = Fabric::new(&input);

        assert_eq!(x, 1);
        assert_eq!(y, 3);
        assert_eq!(width, 4);
        assert_eq!(height, 4);
    }

    #[test]
    fn test_overlap_count() {
        let fabric_1 = Fabric {
            x: 1,
            y: 3,
            width: 4,
            height: 4,
        };
        let fabric_2 = Fabric {
            x: 3,
            y: 1,
            width: 4,
            height: 4,
        };
        let fabric_3 = Fabric {
            x: 5,
            y: 5,
            width: 2,
            height: 2,
        };
        let fabrics = vec![fabric_1, fabric_2, fabric_3];

        assert_eq!(overlap_count(&fabrics), 4);
    }
}
