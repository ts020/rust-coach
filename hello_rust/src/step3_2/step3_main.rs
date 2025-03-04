pub fn step3_main() {
    // 以下のコードはコンパイルエラーになります
    // let reference_to_nothing = dangle();

    // 代わりに、値を直接返す関数は問題ありません
    let string = no_dangle();
    println!("返された文字列: {}", string);
}

// この関数はコンパイルエラーになります
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // sはここでスコープを抜けて解放されるが、その参照を返そうとしている
// }

// この関数は問題ありません
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 所有権ごと返す
}
