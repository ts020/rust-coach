fn main() {
    for num in 1..20 {
        if is_price(num) {
            println!("{} is a 素数", num);
        } else {
            println!("{} is 素数じゃない", num);
        }
    }
}

fn is_price(n: u32) -> bool {
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
