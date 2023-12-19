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
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for seq in sequences {
        let equal = seq.find('=');
        if let Some(eq) = equal {
            let name = seq[0..eq].to_string();
            let focal_length = seq[eq + 1..seq.len()].parse::<usize>().unwrap();
            let box_number = hash_string(&name);
            let boxx= &mut boxes[box_number];
            let mut contained = false;
            for lens in boxx.iter_mut() {
                // replace the old lens
                if lens.name == name {
                    lens.focal_length = focal_length;
                    contained = true;
                    break;
                }
            }
            if contained {
                continue;
            }
            boxx.push(Lens::new(name, focal_length));
            continue;
        }
        let dash = seq.find('-').unwrap();
        let name = seq[0..dash].to_string();
        let box_number = hash_string(&name);
        let boxx= &mut boxes[box_number];
        let mut lens_index = None;
        for (i, lens) in boxx.iter_mut().enumerate() {
            // Remove if contained
            if lens.name == name {
                lens_index = Some(i);
                break;
            }
        }
        if let Some(index) = lens_index {
            boxx.remove(index);
        }
    }
    let mut power: usize = 0;
    for (box_num, boxx) in boxes.iter().enumerate() {
        for (lens_index, lens) in boxx.iter().enumerate() {
            let product = (1 + box_num) * (lens_index + 1) * lens.focal_length;
            power += product;
        }
    }
    println!("power: {power}");
    power
}

#[derive(Clone)]
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