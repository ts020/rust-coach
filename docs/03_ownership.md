# モジュール3: 所有権システム

Rustの最も特徴的な機能は所有権（Ownership）システムです。このシステムにより、ガベージコレクタなしでメモリ安全性を保証しています。

## 3.1 所有権の基本ルール

Rustの所有権には以下の基本ルールがあります：

1. Rustの各値は、「所有者」と呼ばれる変数を持つ
2. 一度に所有者は1つだけ
3. 所有者がスコープから外れると、値は自動的に破棄される

```rust
{
    let s = String::from("hello"); // sはStringの所有者
    // sを使った処理
} // ここでsはスコープ外になり、Stringは自動的に破棄される
```

## 3.2 変数のスコープ

変数のスコープは、変数が宣言された場所から始まり、そのブロックの終わりまで続きます。

```rust
{                      // sはまだ有効ではない
    let s = "hello";   // sは有効になる
    // sを使った処理
}                      // このスコープは終了し、sは無効になる
```

## 3.3 ムーブセマンティクス

Rustでは、変数の代入や関数への引数渡しでは、デフォルトで「ムーブ」が発生します。

```rust
let s1 = String::from("hello");
let s2 = s1; // s1の所有権がs2に移動（ムーブ）

// println!("{}", s1); // エラー: s1はもう有効ではない
```

### スタックデータのコピー

整数や浮動小数点数などの単純な型（`Copy`トレイトを実装している型）は、ムーブではなくコピーされます。

```rust
let x = 5;
let y = x; // xの値がyにコピーされる
println!("x = {}, y = {}", x, y); // 両方とも有効
```

## 3.4 所有権と関数

関数に値を渡すと、所有権も移動します。

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s); // sの所有権が関数に移動
    // println!("{}", s); // エラー: sはもう有効ではない

    let x = 5;
    makes_copy(x); // xの値がコピーされる
    println!("{}", x); // xはまだ有効
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_stringはスコープ外になり、メモリが解放される

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integerはスコープ外になるが、特別な処理は不要
```

## 3.5 戻り値と所有権

関数は戻り値によって所有権を移動することもできます。

```rust
fn main() {
    let s1 = gives_ownership(); // 戻り値の所有権がs1に移動
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2の所有権が関数に移動し、戻り値の所有権がs3に移動
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 所有権が呼び出し元に移動
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 所有権が呼び出し元に移動
}
```

## 3.6 参照と借用

所有権を移動せずに値を使用するには、「参照」を使用します。参照を作成することを「借用」と呼びます。

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // s1の参照を渡す（借用）
    println!("The length of '{}' is {}.", s1, len); // s1はまだ有効
}

fn calculate_length(s: &String) -> usize { // 参照を受け取る
    s.len()
} // sはスコープ外になるが、参照しているだけなので何も起きない
```

### 可変参照

デフォルトでは、参照は不変です。値を変更するには、可変参照を使用します。

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s); // 可変参照を渡す
    println!("{}", s); // "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 参照のルール

1. 任意の時点で、**1つの可変参照**または**任意の数の不変参照**を持つことができる
2. 参照は常に有効でなければならない

```rust
let mut s = String::from("hello");

// OK: 複数の不変参照
let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);

// OK: r1とr2はここで最後に使用されるので、以降は可変参照が可能
let r3 = &mut s;
println!("{}", r3);

// エラー: 可変参照と不変参照を同時に持つことはできない
// let r1 = &s;
// let r3 = &mut s;
// println!("{} and {}", r1, r3);
```

## 3.7 スライス

スライスは、コレクションの一部への参照です。

### 文字列スライス

```rust
let s = String::from("hello world");

let hello = &s[0..5]; // "hello"
let world = &s[6..11]; // "world"

// 省略形
let hello = &s[..5]; // 0から始まる場合は省略可能
let world = &s[6..]; // 末尾まで続く場合は省略可能
let whole = &s[..]; // 全体を参照
```

### 文字列リテラル

文字列リテラルは実際には文字列スライスです。

```rust
let s = "Hello, world!"; // sの型は&str
```

### その他のスライス

配列などの他のコレクションにもスライスを使用できます。

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3]; // [2, 3]
```

## 演習

1. 文字列を受け取り、最初の単語を返す関数を作成してください。
2. 文字列のリストを受け取り、最も長い文字列への参照を返す関数を作成してください。
3. 可変参照を使用して、整数のベクタの各要素を2倍にする関数を作成してください。

## 次のステップ

これでRustの所有権システムについて学びました。次のモジュールでは、構造体と列挙型について学びます。

[モジュール4: 構造体と列挙型](04_structs_enums.md)に進みましょう。