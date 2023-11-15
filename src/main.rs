fn main() {
    let prices = vec![7,1,5,3,6,4];

    println!("{:?}", max_profit(prices));
    
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;
    for p in prices {
        min_price = std::cmp::min(min_price, p);
        max_profit = std::cmp::max(max_profit, p - min_price);
    }

    max_profit
}