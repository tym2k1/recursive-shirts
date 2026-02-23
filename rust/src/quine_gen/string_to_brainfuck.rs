fn shift_by(num: isize) -> String {
    if num == 0 {
        String::new()
    } else if num > 0 {
        ">".repeat(num as usize)
    } else {
        "<".repeat((-num) as usize)
    }
}

fn change_by(num: isize) -> String {
    if num == 0 {
        String::new()
    } else if num > 0 {
        "+".repeat(num as usize)
    } else {
        "-".repeat((-num) as usize)
    }
}

fn generate_code(
    input: &str,
    max_branch_distance: usize,
    loop_base: usize,
) -> (String, Vec<u8>, usize, Vec<usize>) {
    let mut final_code = "+".repeat(loop_base);
    final_code.push('[');

    let (branches, seq) = generate_path(input, max_branch_distance);

    let mut appr: Vec<i32> = Vec::new();

    // Bootstrap loop
    for branch in &branches {
        let coefficient = (branch[0] as f32 / loop_base as f32).round() as i32;
        final_code.push('>');
        final_code += &change_by(coefficient as isize);
        appr.push(coefficient * loop_base as i32);
    }

    final_code += &shift_by(-(branches.len() as isize));
    final_code.push_str("-]>");

    let mut indexes = vec![0usize; branches.len()];

    for (step, &branch_index) in seq.iter().enumerate() {
        let index = indexes[branch_index];
        let branch = &branches[branch_index];

        if step > 0 {
            let prev = seq[step - 1];
            final_code += &shift_by(branch_index as isize - prev as isize);
        }

        if index > 0 {
            final_code += &change_by(
                branch[index] as isize - branch[index - 1] as isize
            );
        } else {
            final_code += &change_by(
                branch[0] as isize - appr[branch_index] as isize
            );
        }

        indexes[branch_index] += 1;
        final_code.push('.');
    }

    let final_pointer = *seq.last().unwrap_or(&0);
    let final_cells: Vec<u8> = branches.iter().map(|b| *b.last().unwrap()).collect();

    (final_code, final_cells, final_pointer, seq.clone())
}

fn generate_path(
    input: &str,
    max_branch_distance: usize,
) -> (Vec<Vec<u8>>, Vec<usize>) {
    let mut branch_list: Vec<Vec<u8>> = Vec::new();
    let mut sequence: Vec<usize> = Vec::new();

    for ch in input.bytes() {
        let mut closest_branch: Option<usize> = None;
        let mut min_dist = max_branch_distance as isize;

        for (i, branch) in branch_list.iter().enumerate() {
            let head = *branch.last().unwrap();
            let dist = (head as isize - ch as isize).abs();
            if dist < min_dist {
                min_dist = dist;
                closest_branch = Some(i);
            }
        }

        if let Some(idx) = closest_branch {
            sequence.push(idx);
            branch_list[idx].push(ch);
        } else {
            sequence.push(branch_list.len());
            branch_list.push(vec![ch]);
        }
    }

    (branch_list, sequence)
}

pub fn autotune(input: &str)
    -> (String, usize, (usize, usize), Vec<u8>, usize, Vec<usize>)
{
    fn test_range<I, J>(
        input: &str,
        dist_range: I,
        base_range: J,
        current_best: Option<(String, usize, (usize, usize), Vec<u8>, usize, Vec<usize>)>
    ) -> (String, usize, (usize, usize), Vec<u8>, usize, Vec<usize>)
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize> + Clone,
    {
        let mut best = current_best;
        let mut best_len = best.as_ref().map(|b| b.1);

        for dist in dist_range {
            for base in base_range.clone() {
                let (code, cells, ptr, seq) = generate_code(input, dist, base);
                let length = code.len();

                if best_len.is_none() || length < best_len.unwrap() {
                    best = Some((code, length, (dist, base), cells, ptr, seq));
                    best_len = Some(length);
                }
            }
        }

        best.unwrap()
    }

    // Stage 1
    let mut best = test_range(
        input,
        (1..=40).step_by(3),
        (4..=16).step_by(2),
        None
    );

    let (d, b) = best.2;

    // Stage 2
    best = test_range(
        input,
        (d.saturating_sub(3))..=(d + 3),
        (b.saturating_sub(3).max(3))..=(b + 3),
        Some(best)
    );

    let (d, b) = best.2;

    // Stage 3
    best = test_range(
        input,
        (d.saturating_sub(1))..=(d + 1),
        (b.saturating_sub(1).max(3))..=(b + 1),
        Some(best)
    );

    best
}

fn main() {
    let test = r#"<svg font-family="monospace" font-size="3" style="white-space:pre;line-height:1.6"><text y="3"><![CDATA["#;

    let (best_code, best_len, (best_dist, best_base), cells_state, ptr, _) =
        autotune(test);

    println!("Best branch distance: {}", best_dist);
    println!("Best loop base: {}", best_base);
    println!("Memory cells used: {}", cells_state.len());
    println!("Final pointer position: {}", ptr);
    println!("Final tape state:");

    for (i, v) in cells_state.iter().enumerate() {
        if i == ptr {
            print!("[{}] ", v);
        } else {
            print!("{} ", v);
        }
    }
    println!();

    println!("Program length: {}", best_len);
    println!();
    println!("{}", best_code);
}
