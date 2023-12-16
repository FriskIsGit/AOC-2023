// # - cuba-shaped rocks, O - rounded rocks, . - empty space
pub fn parabolic_dish1(lines: Vec<String>) -> usize {
    // Rocks move north
    let map = parse_input(lines);
    let all_columns = map[0].len();
    for col_index in 0..all_columns {
        let mut index = 0;
        for row in &map {
            let val = row[col_index];

        }
    }





    let mut sum = 0;
    sum
}

pub fn parse_input(mut lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut map = Vec::with_capacity(lines.len());
    for line in lines.iter_mut() {
        let taken_line = std::mem::take(&mut *line);
        map.push(taken_line.into_bytes());
    }
    map
}