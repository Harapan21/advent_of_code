#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

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
                1 => vec2[output] = input + input2,
                2 => vec2[output] = input * input2,
                99 => continue,
                _ => unreachable!(),
            };
            element.clear();
        }
    }
    vec2
}

fn find(vec: Vec<usize>, get: usize) -> Rc<Option<usize>> {
    let mut result = Rc::new(None);
    let mut temp_vec = vec;
    for noun in 0..=99 {
        for verb in 0..=99 {
            temp_vec[1] = noun;
            temp_vec[2] = verb;
            let computes = compute(temp_vec.to_owned());
            if &computes[0] == &get {
                *Rc::get_mut(&mut result).unwrap() = Some(100 * noun + verb);
            };
        }
    }
    return result;
}

fn main() {
    let vec = file_reader("input.txt");
    let get = 19690720;
    let result = find(vec, get);

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
    #[test]
    fn find_test() {
        let vec = file_reader("input.txt");
        let get = 3267740;
        let exac = Some(1202);
        let result = find(vec, get);
        assert_eq!(*result, exac);
    }
}
