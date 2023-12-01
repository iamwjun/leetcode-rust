use std::collections::HashMap;

fn main() {
    let arr = vec![1,4,5,2,6,3];
    let mat: Vec<Vec<i32>> = vec![vec![4,3,5], vec![1,2,6]];

    println!("{:?}", first_complete_index(arr, mat));
}

fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let mut coordinates: HashMap<i32, (usize, usize)> = HashMap::new();
    let row_len = mat.len();
    let column_len = mat[0].len();

    for i in 0..row_len {
        for j in 0..column_len {
            coordinates.insert(mat[i][j], (i, j));
        }
    }

    let mut rows_count = vec![0;row_len];
    let mut columns_count = vec![0;column_len];

    for (index,a) in arr.iter().enumerate() {
        
        if let Some(&(i, j)) = coordinates.get(a) {
            rows_count[i] += 1;
            columns_count[j] += 1;
            if rows_count[i] == column_len || columns_count[j] == row_len {
                return index as i32;
            }
        }
    }

    unreachable!()
}

