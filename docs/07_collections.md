# モジュール7: コレクション

Rustの標準ライブラリには、複数の値を格納するための効率的なコレクション型が用意されています。このモジュールでは、最も一般的に使用される3つのコレクション型について学びます：ベクタ、文字列、ハッシュマップ。

## 7.1 ベクタ（Vec<T>）

ベクタ（`Vec<T>`）は、同じ型の値を可変長のリストとして格納するコレクションです。

### ベクタの作成

```rust
// 空のベクタを作成
let v: Vec<i32> = Vec::new();

// マクロを使用して初期値を持つベクタを作成
let v = vec![1, 2, 3];
```

### ベクタへの要素の追加

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### ベクタの要素へのアクセス

ベクタの要素には、インデックスまたは`get`メソッドを使用してアクセスできます。

```rust
let v = vec![1, 2, 3, 4, 5];

// インデックスを使用（範囲外の場合はパニック）
let third: &i32 = &v[2];
println!("The third element is {}", third);

// getメソッドを使用（範囲外の場合はNoneを返す）
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

### ベクタの要素の変更

```rust
let mut v = vec![1, 2, 3, 4, 5];
v[2] = 10;
```

### ベクタの要素の反復処理

```rust
let v = vec![100, 32, 57];

// 不変参照で反復処理
for i in &v {
    println!("{}", i);
}

// 可変参照で反復処理
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;  // デリファレンスして値を変更
}
```

### 異なる型の値を格納する

列挙型を使用すると、異なる型の値をベクタに格納できます。

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

### ベクタのメソッド

ベクタには多くの便利なメソッドがあります：

```rust
let mut v = vec![1, 2, 3];

// 要素の追加
v.push(4);

// 最後の要素を削除して返す
let last = v.pop();  // Some(4)

// 長さを取得
let length = v.len();  // 3

// ベクタが空かどうかを確認
let is_empty = v.is_empty();  // false

// ベクタをクリア
v.clear();  // []

// 要素の挿入
v.push(1);
v.push(2);
v.insert(1, 5);  // [1, 5, 2]

// 要素の削除
v.remove(1);  // [1, 2]
```

## 7.2 文字列（String）

Rustの文字列は、UTF-8でエンコードされたテキストを格納するコレクションです。

### 文字列の作成

```rust
// 空の文字列を作成
let mut s = String::new();

// 文字列リテラルから作成
let s = "initial contents".to_string();
let s = String::from("initial contents");
```

### 文字列の更新

```rust
let mut s = String::from("foo");

// 文字列の追加
s.push_str("bar");  // "foobar"
s.push('!');  // "foobar!"

// 文字列の連結
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1はムーブされ、s3は"Hello, world!"

// formatマクロを使用
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);  // "tic-tac-toe"
```

### 文字列のインデックス

Rustの文字列は、バイト単位でインデックスを指定することができません。これは、UTF-8エンコーディングでは、1文字が複数のバイトを占める可能性があるためです。

```rust
let s = String::from("hello");
// let h = s[0];  // エラー: Stringはインデックスでアクセスできない
```

### 文字列のスライス

文字列のスライスを取得するには、範囲を指定します。ただし、範囲がUTF-8の文字境界でない場合はパニックします。

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];  // "Зд"（最初の2文字、4バイト）
```

### 文字列の反復処理

文字列を文字単位で処理するには、`chars`メソッドを使用します。

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
// न
// म
// स
// ्
// त
// े
```

バイト単位で処理するには、`bytes`メソッドを使用します。

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
// 224
// 164
// 168
// ...
```

### 文字列のメソッド

文字列には多くの便利なメソッドがあります：

```rust
let mut s = String::from("hello world");

// 部分文字列の検索
let contains = s.contains("world");  // true
let starts_with = s.starts_with("hello");  // true
let ends_with = s.ends_with("world");  // true
let find = s.find("world");  // Some(6)

// 文字列の置換
let replaced = s.replace("world", "Rust");  // "hello Rust"

// 文字列の分割
let split: Vec<&str> = s.split(' ').collect();  // ["hello", "world"]

// 文字列のトリミング
let trimmed = "  hello  ".trim();  // "hello"

// 文字列の長さ
let len = s.len();  // 11（バイト数）
let char_count = s.chars().count();  // 11（文字数）
```

## 7.3 ハッシュマップ（HashMap<K, V>）

ハッシュマップ（`HashMap<K, V>`）は、キーと値のペアを格納するコレクションです。

### ハッシュマップの作成

```rust
use std::collections::HashMap;

// 空のハッシュマップを作成
let mut scores = HashMap::new();

// キーと値を挿入
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// ベクタからハッシュマップを作成
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```

### ハッシュマップの値へのアクセス

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// getメソッドを使用
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // Some(&10)

// 反復処理
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### ハッシュマップの更新

```rust
let mut scores = HashMap::new();

// 値の上書き
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);  // "Blue"の値が25に上書きされる

// キーが存在しない場合のみ挿入
scores.entry(String::from("Yellow")).or_insert(50);  // "Yellow"が存在しないので挿入
scores.entry(String::from("Blue")).or_insert(50);    // "Blue"は既に存在するので何もしない

// 既存の値に基づいて更新
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
// {"hello": 1, "world": 2, "wonderful": 1}
```

### ハッシュ関数

デフォルトでは、`HashMap`は「暗号学的に安全な」ハッシュ関数を使用しますが、パフォーマンスよりもセキュリティを優先しています。別のハッシュ関数を指定するには、`BuildHasher`トレイトを実装した型を使用します。

### ハッシュマップのメソッド

ハッシュマップには多くの便利なメソッドがあります：

```rust
let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);

// キーの存在確認
let has_key = map.contains_key("a");  // true

// 要素の削除
let removed = map.remove("b");  // Some(2)

// ハッシュマップのクリア
map.clear();  // {}

// 要素数の取得
let len = map.len();  // 0
```

## 7.4 その他のコレクション

Rustの標準ライブラリには、他にも多くのコレクション型があります：

- `BTreeMap<K, V>`: キーでソートされたマップ
- `HashSet<T>`: 重複のない値の集合
- `BTreeSet<T>`: ソートされた値の集合
- `VecDeque<T>`: 両端キュー
- `LinkedList<T>`: 双方向連結リスト
- `BinaryHeap<T>`: 優先度キュー

これらのコレクションは、特定のユースケースに最適化されています。

## 演習

1. 整数のベクタを作成し、その中の最大値、最小値、平均値を計算する関数を実装してください。
2. 文字列を受け取り、各単語の出現回数をカウントするプログラムを作成してください。
3. 従業員を部署に割り当てるプログラムを作成してください。ユーザーが「Add Sally to Engineering」のようなコマンドを入力できるようにし、部署ごとの従業員リストを表示できるようにしてください。

## 次のステップ

これでRustのコレクションについて学びました。次のモジュールでは、並行処理について学びます。

[モジュール8: 並行処理](08_concurrency.md)に進みましょう。