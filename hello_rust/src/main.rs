use std::fs;
use std::io::Write;

fn main() {
    let start = 1;
    let end = 100;

    println!("{} - {} の範囲の素数を計算...", start, end);

    // 素数リストを作成
    let mut primes = Vec::new();
    for num in start..=end {
        if is_prime(num) {
            primes.push(num);
        }
    }

    println!("素数の件数 {}", primes.len());

    // 素数リストをファイルに書き込み
    let filename = "primes.csv";
    let mut file = fs::File::create(filename).expect("ファイルの作成失敗");

    for prime in &primes {
        writeln!(file, "{}", prime).expect("ファイルへの書き込み失敗");
    }

    let contents = fs::read_to_string(filename).expect("ファイルの読み込み失敗");

    println!("\n{}の内容", filename);
    println!("{}", contents);
}
/// 素数を判定するコードです
/// # Arguments
/// * `n` - 判定する数値
/// # Returns
/// 素数の場合は true、それ以外は false
/// # Examples
/// ```
/// assert_eq!(true, is_prime(2));
/// assert_eq!(true, is_prime(3));
/// assert_eq!(false, is_prime(4));
/// assert_eq!(true, is_prime(5));
/// assert_eq!(false, is_prime(6));
/// assert_eq!(true, is_prime(7));
/// ```
fn is_prime(n: u32) -> bool {
    match n {
        n if n < 2 => false,
        n if n == 2 => true,
        n if n % 2 == 0 => false,
        _ => {
            let mut i = 3;
            while i * i <= n {
                if n % i == 0 {
                    return false;
                }
                i += 2;
            }
            return true;
        }
    }
}
