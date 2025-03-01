# モジュール2: 基本的な構文と概念

## 2.1 変数と可変性

Rustでは、デフォルトで変数は不変（immutable）です。変数を宣言するには`let`キーワードを使用します。

```rust
// 不変変数
let x = 5;
// x = 6; // エラー: 不変変数に再代入はできない

// 可変変数
let mut y = 5;
y = 6; // OK
```

### シャドーイング

Rustでは同じ名前の変数を再宣言することができます（シャドーイング）。

```rust
let x = 5;
let x = x + 1; // 新しいxが古いxをシャドーイング
let x = x * 2; // さらにシャドーイング
println!("The value of x is: {}", x); // 12
```

## 2.2 データ型

Rustは静的型付け言語で、コンパイル時にすべての変数の型が決定されている必要があります。

### スカラー型

Rustには4つの基本的なスカラー型があります：

1. **整数型**: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
2. **浮動小数点型**: `f32`, `f64`
3. **論理値型**: `bool`（`true`または`false`）
4. **文字型**: `char`（Unicodeスカラー値）

```rust
let integer: i32 = 42;
let float: f64 = 3.14;
let boolean: bool = true;
let character: char = '😊';
```

### 複合型

Rustには2つの基本的な複合型があります：

1. **タプル型**: 異なる型の値をグループ化
2. **配列型**: 同じ型の値のコレクション

```rust
// タプル
let tup: (i32, f64, bool) = (500, 6.4, true);
let (x, y, z) = tup; // 分解
let first = tup.0; // インデックスでアクセス

// 配列
let arr = [1, 2, 3, 4, 5]; // 固定長配列
let first = arr[0]; // インデックスでアクセス
```

## 2.3 関数

Rustでは関数を定義するには`fn`キーワードを使用します。

```rust
fn main() {
    println!("Hello from main!");
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

### 戻り値

関数は値を返すことができます。戻り値の型は`->`の後に指定します。

```rust
fn five() -> i32 {
    5 // 最後の式が戻り値（セミコロンなし）
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 最後の式が戻り値
}
```

## 2.4 制御フロー

### if式

条件分岐には`if`式を使用します。

```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

`if`は式なので、変数に代入することもできます。

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

### ループ

Rustには3種類のループがあります：

1. **loop**: 無限ループ
2. **while**: 条件付きループ
3. **for**: コレクションの要素を反復処理するループ

```rust
// loop
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // ループから値を返す
    }
};

// while
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}

// for
let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("the value is: {}", element);
}

// 範囲を使ったfor
for number in 1..4 {
    println!("{}!", number);
}
```

## 2.5 コメント

Rustでは、コメントは`//`で始まります。

```rust
// これは単一行コメントです

/*
 これは
 複数行
 コメントです
*/

/// ドキュメンテーションコメント（ドキュメント生成に使用）
fn documented_function() {
    // 関数の実装
}
```

## 2.6 基本的な入出力

### 標準出力

`println!`マクロを使用して標準出力に出力できます。

```rust
println!("Hello, world!");
println!("The value is: {}", 42);
println!("x = {}, y = {}", 10, 20);
```

### 標準入力

標準入力からの読み取りには、`std::io`モジュールを使用します。

```rust
use std::io;

fn main() {
    println!("Please input your name:");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("Hello, {}!", input.trim());
}
```

## 演習

1. 摂氏と華氏の温度変換プログラムを作成してください。
2. フィボナッチ数列の最初の10項を計算して表示するプログラムを作成してください。
3. ユーザーから数値を入力として受け取り、その数値が素数かどうかを判定するプログラムを作成してください。

## 次のステップ

これでRustの基本的な構文と概念について学びました。次のモジュールでは、Rustの最も重要な特徴の一つである所有権システムについて学びます。

[モジュール3: 所有権システム](03_ownership.md)に進みましょう。