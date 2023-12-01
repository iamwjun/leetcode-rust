use std::collections::HashMap;

fn main() {
    let arr = vec![6,2,3,1,4,5];
    let mat: Vec<Vec<i32>> = vec![vec![5,1], vec![2,4], vec![6,3]];

    println!("{:?}", first_complete_index(arr, mat));
}

fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let mut coordinates: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut rows: HashMap<usize, Vec<i32>> = HashMap::new();
    let mut columns: HashMap<usize, Vec<i32>> = HashMap::new();

    for (row, m) in mat.iter().enumerate() {
        rows.entry(row).or_insert(m.to_vec());
        for (column, &v) in m.iter().enumerate() {
            coordinates.insert(v, vec![row, column]);
            columns.entry(column).or_insert(Vec::new()).push(v)
        }
    }


    for (i, a) in arr.iter().enumerate() {
        if let Some(v) = coordinates.get(a) {
            let (row, column) = (v[0], v[1]);

            if let Some(curr_row) = rows.get_mut(&row) {
                curr_row[column] = 0;
                if curr_row.iter().all(|&x| x == 0){
                    return i as i32;
                }
            }

            if let Some(curr_column) = columns.get_mut(&column) {
                curr_column[row] = 0;
                if curr_column.iter().all(|&x| x == 0){
                    return i as i32;
                }
            }
        }
    }

    -1
}

