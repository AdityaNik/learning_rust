pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    let cols = if rows > 0 { garden[0].len() } else { 0 };

    let mut result = Vec::with_capacity(rows);

    for i in 0..rows {
        let mut new_row = String::with_capacity(cols);
        for j in 0..cols {
            let cell = garden[i].chars().nth(j).unwrap();

            if cell == '*' {
                new_row.push('*');
                continue;
            }

            let mut count = 0;
            for di in [-1, 0, 1] {
                for dj in [-1, 0, 1] {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0 && ni < rows as isize && nj >= 0 && nj < cols as isize {
                        if garden[ni as usize].chars().nth(nj as usize).unwrap() == '*' {
                            count += 1;
                        }
                    }
                }
            }

            if count == 0 {
                new_row.push(' ');
            } else {
                new_row.push_str(&count.to_string());
            }
        }
        result.push(new_row);
    }

    result
}
