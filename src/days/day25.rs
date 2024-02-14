use std::collections::HashMap;

pub fn wires1(lines: Vec<String>) -> usize {
    let map = parse_input(lines);
    for (key, value) in map {
        println!("{} => {:?}", key, value);
    }
    0
}

pub fn parse_input(lines: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::with_capacity(lines.len());
    for line in lines {
        let colon_index = line.find(':').expect("Must contain colon");
        let main_name = line[0..colon_index].to_string();
        let names_primitive = line[colon_index + 2..].split(' ').collect();
        let names = deref_vector(names_primitive);

        if let Some(vec) = map.get_mut(&main_name) {
            vec.extend_from_slice(&names);
        } else {
            map.insert(main_name.clone(), names.clone());
        }
        for name in names {
            // map the name back
            if let Some(vec) = map.get_mut(&name) {
                vec.push(main_name.clone())
            } else {
                map.insert(name, vec![main_name.clone()]);
            }
        }
    }
    map
}

fn deref_vector(vec: Vec<&str>) -> Vec<String> {
    let mut out_vec: Vec<String> = Vec::with_capacity(vec.len());
    for el in vec.iter() {
        out_vec.push(el.to_string());
    }
    out_vec
}

pub fn wires2(lines: Vec<String>) -> usize {
    parse_input(lines);
    0
}