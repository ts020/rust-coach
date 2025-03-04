pub fn demonstrate_string_slices() {
    let message = String::from("Hello, Rust World!");
    
    // 文字列スライスの基本
    let hello = &message[0..5];  // "Hello"
    let rust = &message[7..11];  // "Rust"
    
    println!("完全な文字列: {}", message);
    println!("スライス1: {}", hello);
    println!("スライス2: {}", rust);
    
    // 文字列スライスのパターンマッチング
    match_greeting(&message);
    
    // 文字列の一部を安全に取得
    if let Some(first_word) = get_first_word(&message) {
        println!("最初の単語: {}", first_word);
    }
}

fn match_greeting(text: &str) {
    match &text[..] {
        "Hello" => println!("英語の挨拶です"),
        "Hola" => println!("スペイン語の挨拶です"),
        _ => println!("他の挨拶、もしくは挨拶ではありません"),
    }
}

fn get_first_word(text: &str) -> Option<&str> {
    // 文字列が空の場合はNoneを返す
    if text.is_empty() {
        return None;
    }
    
    // スペースで区切られた最初の単語を探す
    match text.find(' ') {
        Some(pos) => Some(&text[..pos]),
        None => Some(text),  // スペースがない場合は全体を返す
    }
}