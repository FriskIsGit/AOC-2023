
pub fn lens_library1(lines: Vec<String>) -> usize {
    assert_eq!(lines.len(), 1);
    let sequences = lines[0].split(",");
    let mut sum: usize = 0;
    for seq in sequences {
        sum += hash_string(seq);
    }
    println!("Sum: {sum}");
    sum
}

pub fn lens_library2(lines: Vec<String>) -> usize {
    assert_eq!(lines.len(), 1);
    let sequences = lines[0].split(",");
    let mut lenses = vec![];
    let mut boxes: Vec<Vec<Lens>> = vec![];
    for seq in sequences {
        let equal = seq.find('=');
        if let Some(eq) = equal {
            let name = seq[0..eq].to_string();
            let number = seq[eq+1..seq.len()].parse::<usize>().unwrap();
            lenses.push(Lens::new(name, number));
            continue
        }
        let dash = seq.find('-').unwrap();
        let name = seq[0..dash].to_string();
        for lens in lenses.iter_mut() {
            if lens.name == name {

            }
        }
        continue
    }
    let mut sum: usize = 0;
    println!("Sum: {sum}");
    sum
}

pub struct Lens {
    pub name: String,
    pub focal_length: usize,
}
impl Lens {
    pub fn new(name: String, focal_length: usize) -> Self {
        Self { name, focal_length }
    }
}

pub fn hash_string(str: &str) -> usize {
    let mut number = 0;
    for byte in str.bytes() {
        number += byte as usize;
        number *= 17;
        number %= 256;
    }
    number
}