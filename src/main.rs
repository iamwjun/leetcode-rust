fn main() {
    let s = String::from("A");
    let num_rows = 3;

    println!("{:?}", convert(s, num_rows));
}

fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 || s.len() == 1 {
        return s;
    }
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut ret: Vec<char> = Vec::new();
    let rows = num_rows as usize;
    let num_columns = s.len();

    for _i in 0..rows {
        matrix.push(vec!['0'; num_columns]);
    }

    let mut start = 0;
    for j in 0..num_columns {
        let n = j % (rows - 1);

        if start >= num_columns {
            break;
        }

        if n == 0 {
            for i in 0..rows {
                if let Some(char) = chars.get(start) {
                    matrix[i][j] = *char;
                    start += 1;
                }
            }
        } else {
            if let Some(char) = chars.get(start) {
                matrix[rows - n - 1][j] = *char;
                start += 1;
            }
        }
    }

    for r in 0..rows {
        for c in 0..num_columns {
            if matrix[r][c] != '0' {
                ret.push(matrix[r][c]);
            }
        }
    }

    ret.iter().collect::<String>()
}
