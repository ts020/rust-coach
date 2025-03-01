# モジュール4: 構造体と列挙型

Rustでは、構造体（Structs）と列挙型（Enums）を使用して、独自のデータ型を定義できます。これらは、関連するデータをグループ化し、コードの構造化に役立ちます。

## 4.1 構造体

構造体は、関連するデータをグループ化するためのカスタムデータ型です。

### 構造体の定義と作成

```rust
// 構造体の定義
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 構造体のインスタンス作成
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 可変インスタンス
let mut user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: true,
    sign_in_count: 1,
};

// フィールドの変更（可変インスタンスのみ）
user2.email = String::from("newemail@example.com");
```

### フィールド初期化省略記法

変数名とフィールド名が同じ場合、省略記法を使用できます。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,    // email: email の省略形
        username, // username: username の省略形
        active: true,
        sign_in_count: 1,
    }
}
```

### 構造体更新記法

既存の構造体インスタンスから新しいインスタンスを作成する際、一部のフィールドだけを変更したい場合に便利です。

```rust
let user3 = User {
    email: String::from("another@example.com"),
    ..user1 // user1の残りのフィールドを使用
};
```

### タプル構造体

名前付きフィールドを持たない、タプルに似た構造体です。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### ユニット様構造体

フィールドを持たない構造体です。トレイトを実装する際に役立ちます。

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

### 構造体のデバッグ表示

`#[derive(Debug)]`属性を使用すると、構造体のデバッグ出力が可能になります。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

let rect = Rectangle {
    width: 30,
    height: 50,
};

println!("rect is {:?}", rect);      // 単行表示
println!("rect is {:#?}", rect);     // 整形表示
```

## 4.2 メソッド

メソッドは、構造体（または列挙型、トレイト）に関連付けられた関数です。

### メソッドの定義

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // メソッド（&selfを第一引数に取る）
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 関連関数（selfを取らない）
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 使用例
let rect1 = Rectangle {
    width: 30,
    height: 50,
};

println!("Area: {}", rect1.area());

let rect2 = Rectangle {
    width: 10,
    height: 40,
};

println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

// 関連関数の呼び出し
let square = Rectangle::square(20);
```

### 複数のimplブロック

一つの型に対して複数の`impl`ブロックを定義することができます。

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

## 4.3 列挙型

列挙型は、取りうる値の集合を定義するデータ型です。

### 基本的な列挙型

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

### データを持つ列挙型

各バリアントにデータを関連付けることができます。

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

### 構造体を持つ列挙型

各バリアントに異なる型や構造体を関連付けることができます。

```rust
struct Ipv4Addr {
    // フィールド
}

struct Ipv6Addr {
    // フィールド
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

### メソッドを持つ列挙型

列挙型にもメソッドを定義できます。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // メソッドの本体
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## 4.4 Option列挙型

Rustには`null`がありません。代わりに、値の存在または不在を表現するために`Option<T>`列挙型を使用します。

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>`は標準ライブラリで定義されており、明示的にスコープに取り込まなくても使用できます。

```rust
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

`Option<T>`を使用することで、値が存在しない可能性を型システムで表現でき、コンパイラがその処理を強制します。

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

// エラー: 異なる型なので加算できない
// let sum = x + y;

// 正しい使用法: unwrapなどでOptionから値を取り出す
let sum = x + y.unwrap_or(0);
```

## 4.5 match式

`match`式は、列挙型の値に基づいてコードを分岐させる強力な制御フロー演算子です。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### パターンとバインディング

`match`のアームでは、列挙型のデータを取り出すことができます。

```rust
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

### Option<T>とのマッチング

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### _プレースホルダ

残りのすべてのパターンにマッチさせるには、`_`プレースホルダを使用します。

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // 他の値は何もしない
}
```

## 4.6 if let

単一のパターンにマッチする場合のみ処理を行いたい場合、`if let`構文を使用できます。

```rust
let some_u8_value = Some(0u8);

// matchを使用した場合
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

// if letを使用した場合（より簡潔）
if let Some(3) = some_u8_value {
    println!("three");
}
```

`else`を追加することもできます。

```rust
let coin = Coin::Penny;

if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    println!("Not a quarter!");
}
```

## 演習

1. 長方形と円を表現できる`Shape`列挙型を作成し、面積を計算するメソッドを実装してください。
2. 学生を表す構造体を作成し、名前、年齢、成績などの情報を格納してください。また、学生の情報を表示するメソッドを実装してください。
3. `Option<T>`を使用して、文字列の最初の単語を返す関数を作成してください。文字列が空の場合は`None`を返します。

## 次のステップ

これでRustの構造体と列挙型について学びました。次のモジュールでは、エラー処理について学びます。

[モジュール5: エラー処理](05_error_handling.md)に進みましょう。