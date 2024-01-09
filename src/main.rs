fn main() {
    let points: Vec<Vec<i32>> = vec![vec![0,0],vec![1,0],vec![-1,0],vec![0,1],vec![0,-1]];

    println!("{:?}", number_of_boomerangs(points));
}

fn squared_euclidean_distance(point1: Vec<i32>, point2: Vec<i32>) -> f64 {
    ((point2[0] - point1[0]).pow(2) + (point2[1] - point1[1]).pow(2)) as f64
}

fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    let mut ret: i32 = 0;
    
    for i in 1..points.len() - 1 {
        let point1 = points[i-1].clone();
        let point2 = points[i].clone();
        let point3 = points[i].clone();
        let point4 = points[i+1].clone();

        if squared_euclidean_distance(point1, point2) == squared_euclidean_distance(point3, point4) {
            ret += 2
        }
    }

    ret
}
