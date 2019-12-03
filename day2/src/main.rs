use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
fn file_reader(file: &'static str) -> Vec<usize> {
    let file = File::open(file).expect("Failed to load file");
    let mut reader = std::io::BufReader::new(file);
    let mut content = String::new();
    let _ = reader.read_to_string(&mut content);

    content
        .split(",")
        .map(|e| e.trim_end().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn compute(vec: Vec<usize>) -> Vec<usize> {
    let mut vec2: Vec<usize> = vec.clone();
    let mut element: Vec<usize> = vec![];
    for (i, _val) in vec.iter().enumerate() {
        element.push(vec2[i]);
        if (i + 1) % 4 == 0 {
            let op_code = element[0];
            let input = vec2[element[1]];
            let input2 = vec2[element[2]];
            let output = element[3];
            match op_code {
                1 => std::mem::replace(&mut vec2[output], input + input2),
                2 => std::mem::replace(&mut vec2[output], input * input2),
                99 => break,
                _ => unreachable!(),
            };

            element.clear();
        }
    }
    vec2
}

fn main() {
    let vec = file_reader("input.txt");
    let result = compute(vec);
    println!("result:\t {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let test1 = vec![1, 0, 0, 0, 99];
        let exac = vec![2, 0, 0, 0, 99];
        let result = compute(test1);
        assert_eq!(result, exac);
    }
    #[test]
    fn test_02() {
        let test2 = vec![2, 3, 0, 3, 99];
        let exac = vec![2, 3, 0, 6, 99];
        let result = compute(test2);
        assert_eq!(result, exac);
    }
    #[test]
    fn test_03() {
        let test3 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let exac = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let result = compute(test3);
        assert_eq!(result, exac);
    }
}
