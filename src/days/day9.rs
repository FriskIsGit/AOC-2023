
pub fn mirage1(lines: Vec<String>) -> isize {
    let mut report = parse_input(lines);

    let mut total_sum = 0;
    for top_sequence in report.iter_mut() {
        // Simplest case, given sequence is already a series
        if let Some(common_diff) = get_common_difference(top_sequence) {
            total_sum += top_sequence.last().unwrap() + common_diff;
            continue
        }
        // Create subsequences until one is found that has const difference
        let mut sub_sequence = get_subsequence_of(top_sequence);
        let mut sequence_pyramid: Vec<Vec<isize>> = vec![];
        sequence_pyramid.push(std::mem::take(top_sequence)); // Yoink?
        loop {
            sequence_pyramid.push(sub_sequence);
            let last_seq = sequence_pyramid.last_mut().unwrap();
            if let Some(common_diff) = get_common_difference(&last_seq) {
                // We push the next element in sequence for clarity
                last_seq.push(last_seq.last().unwrap() + common_diff);
                break;
            }
            sub_sequence = get_subsequence_of(last_seq);
        }
        // Go from bottom up and infer next elements above from the sequence below
        let back_index: usize = sequence_pyramid.len()-1;
        for i in (1..=back_index).rev() {
            let last_below: isize = *sequence_pyramid[i].last().unwrap();
            // We need above sequence to be mutable to push next inferred element to it
            let above_seq = &mut sequence_pyramid[i-1];
            let last_above = above_seq.last().unwrap();
            let next = last_above + last_below;
            above_seq.push(next);
        }
        total_sum += sequence_pyramid.first().unwrap().last().unwrap();
    }

    total_sum
}

pub fn parse_input(lines: Vec<String>) -> Vec<Vec<isize>> {
    let mut report: Vec<Vec<isize>> = vec![];
    for line in lines {
        let str_values = line.split(' ');
        let mut values = vec![];
        for str_val in str_values {
            let val = str_val.parse::<isize>().unwrap();
            values.push(val);
        }
        report.push(values);
    }
    report
}

// returns common difference of an arithmetic series, None if difference is not constant
pub fn get_common_difference(vec: &Vec<isize>) -> Option<isize> {
    let first_diff = vec[1] - vec[0];
    for i in 1..vec.len() - 1 {
        let num_left = vec[i];
        let num_right = vec[i+1];
        let diff = num_right - num_left;
        if first_diff != diff {
            return None;
        }
    }
    Some(first_diff)
}

pub fn get_subsequence_of(sequence: &Vec<isize>) -> Vec<isize> {
    let mut sub_sequence = Vec::with_capacity(sequence.len() - 1);
    for i in 0..sequence.len()-1 {
        let num_left = sequence[i];
        let num_right = sequence[i+1];
        let diff = num_right - num_left;
        sub_sequence.push(diff);
    }
    sub_sequence
}

pub fn sequence_sum(first_value: isize, common_diff: isize, n_elements: usize) -> isize {
    let nth_value = element_value(first_value, common_diff, n_elements);
    n_elements as isize * (first_value + nth_value) / 2
}

// Xn = a + d(n-1)
pub fn element_value(first_value: isize, common_diff: isize, n_elements: usize) -> isize {
    first_value + common_diff * (n_elements as isize - 1)
}

// This code is similar but instead processes leftmost values
pub fn mirage2(lines: Vec<String>) -> isize {
    let mut report = parse_input(lines);
    let mut total_sum: isize = 0;
    for top_sequence in report.iter_mut() {
        if let Some(common_diff) = get_common_difference(top_sequence) {
            total_sum += top_sequence.first().unwrap() - common_diff;
            continue
        }
        let mut sub_sequence = get_subsequence_of(top_sequence);
        let mut sequence_pyramid: Vec<Vec<isize>> = vec![];
        sequence_pyramid.push(std::mem::take(top_sequence)); // Yoink?
        loop {
            sequence_pyramid.push(sub_sequence);
            let last_seq = sequence_pyramid.last_mut().unwrap();
            if let Some(common_diff) = get_common_difference(&last_seq) {
                // We push the next element in sequence for clarity
                last_seq.insert(0, last_seq.first().unwrap() - common_diff);
                break;
            }
            sub_sequence = get_subsequence_of(last_seq);
        }
        // Go from bottom up and infer next elements above from the sequence below
        let back_index: usize = sequence_pyramid.len()-1;
        for i in (1..=back_index).rev() {
            let first_below: isize = *sequence_pyramid[i].first().unwrap();
            // We need above sequence to be mutable to push next inferred element to it
            let above_seq = &mut sequence_pyramid[i-1];
            let first_above = above_seq.first().unwrap();
            let next = first_above - first_below;
            above_seq.insert(0, next);
        }
        for seq in &sequence_pyramid {
            println!("{:?}", seq);
        }
        total_sum += sequence_pyramid.first().unwrap().first().unwrap();
    }
    total_sum
}