fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    println!("data: {:?}", data);

    double_values(&mut data);
    println!("data: {:?}", data);

    // immutableな参照は複数でも問題ない
    // let ref1 = &mut data;
    // let ref2 = &mut data;

    // imuutableな借用が有効な間はmutableな参照は作れない
    // let ref_mut = &mut data;
    println!("data: {:?}", data);
}

fn double_values(v: &mut Vec<i32>) {
    for i in v {
        *i *= 2;
    }
}
