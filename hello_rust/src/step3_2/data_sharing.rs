pub fn data_sharing() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum = calc_sum(&data);
    println!("合計値: {}", sum);

    let avg = calc_avg(&data);
    println!("平均値: {}", avg);

    let max = max(&data);
    println!("最大値: {}", max);

    println!("データの長さ: {}", data.len());
}

fn calc_sum(data: &[i32]) -> i32 {
    let mut sum = 0;
    for i in data {
        sum += i;
    }
    sum
}

fn calc_avg(data: &[i32]) -> f64 {
    let sum = calc_sum(data);
    sum as f64 / data.len() as f64
}

fn max(data: &[i32]) -> i32 {
    let mut max = data[0];
    for &i in data {
        if i > max {
            max = i;
        }
    }
    max
}
