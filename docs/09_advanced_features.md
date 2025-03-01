# モジュール9: 高度な機能

このモジュールでは、Rustの高度な機能について学びます。これらの機能は、より複雑なプログラムを書く際や、コードをより簡潔にする際に役立ちます。

## 9.1 unsafe Rust

Rustは安全性を重視する言語ですが、`unsafe`キーワードを使用することで、コンパイラのいくつかの安全性チェックを無効にすることができます。

### unsafeでできること

`unsafe`ブロック内では、以下の5つの操作が可能になります：

1. 生ポインタの参照外し
2. unsafeな関数やメソッドの呼び出し
3. 可変静的変数へのアクセスや変更
4. unsafeなトレイトの実装
5. `union`フィールドへのアクセス

### 生ポインタ

生ポインタ（raw pointer）は、参照と同様にメモリアドレスを指しますが、安全性の保証がありません。

```rust
let mut num = 5;

// 生ポインタの作成
let r1 = &num as *const i32;  // 不変生ポインタ
let r2 = &mut num as *mut i32;  // 可変生ポインタ

// 生ポインタの参照外しはunsafe
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

### unsafe関数

`unsafe`キーワードを使用して、安全でない関数を定義できます。

```rust
unsafe fn dangerous() {
    // 安全でない操作
}

// unsafe関数の呼び出しはunsafeブロック内で行う必要がある
unsafe {
    dangerous();
}
```

### 外部関数インターフェース（FFI）

Rustから他の言語（主にC）の関数を呼び出すには、外部関数インターフェース（FFI）を使用します。

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

### 可変静的変数

静的変数は、プログラムの実行中に常に存在し、メモリ内の固定位置に配置されます。可変静的変数へのアクセスは、データ競合を引き起こす可能性があるため、`unsafe`です。

```rust
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_counter(3);
    
    unsafe {
        println!("COUNTER: {}", COUNTER);  // 3
    }
}
```

### unsafeトレイト

トレイトを`unsafe`として宣言すると、そのトレイトの実装者は特定の不変条件や契約を守る責任があります。

```rust
unsafe trait Foo {
    // メソッド
}

unsafe impl Foo for i32 {
    // 実装
}
```

## 9.2 高度なトレイト

### 関連型

関連型は、トレイト定義内で型プレースホルダを指定する方法です。

```rust
pub trait Iterator {
    type Item;  // 関連型
    
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        // 実装
    }
}
```

### デフォルト型パラメータ

型パラメータにデフォルト値を指定できます。

```rust
trait Add<RHS=Self> {  // RHSのデフォルト値はSelf
    type Output;
    
    fn add(self, rhs: RHS) -> Self::Output;
}
```

### 完全修飾構文

同じ名前のメソッドが複数ある場合、完全修飾構文を使用して呼び出すメソッドを明示できます。

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    
    person.fly();  // *waving arms furiously*
    Pilot::fly(&person);  // This is your captain speaking.
    Wizard::fly(&person);  // Up!
}
```

関連関数（`self`を取らないメソッド）の場合は、以下のように呼び出します。

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());  // Spot
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());  // puppy
}
```

### スーパートレイト

あるトレイトが別のトレイトの機能に依存する場合、スーパートレイトを使用できます。

```rust
trait Display {
    fn display(&self) -> String;
}

trait DisplayWithDebug: Display + std::fmt::Debug {
    // DisplayとDebugトレイトを実装している型のみがこのトレイトを実装できる
}
```

### newtype パターン

既存の型に外部トレイトを実装するには、newtype パターンを使用します。

```rust
struct Wrapper(Vec<String>);  // タプル構造体

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

## 9.3 高度な型

### 型エイリアス

`type`キーワードを使用して、既存の型に新しい名前（エイリアス）を付けることができます。

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);  // 型エイリアスは同じ型として扱われる
```

複雑な型シグネチャを簡略化するのに役立ちます。

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));
```

### never型

`!`は、決して値を返さない関数の戻り値型を表します。

```rust
fn bar() -> ! {
    // この関数は決して戻らない
    panic!("This function never returns!");
}
```

`continue`、`break`、`panic!`などの式は`!`型を持ちます。

### 動的サイズ型

コンパイル時にサイズがわからない型を動的サイズ型（DST）と呼びます。`str`や`dyn Trait`などがこれに該当します。

```rust
// &strはスライスで、ポインタとサイズの情報を持つ
let s1: &str = "Hello";

// dyn Traitは、トレイトオブジェクトとして使用される
let trait_object: &dyn Display = &String::from("hello");
```

## 9.4 高度な関数とクロージャ

### 関数ポインタ

関数ポインタを使用すると、関数を変数に代入したり、引数として渡したりできます。

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);  // 7
}
```

### クロージャを返す

クロージャを返す関数を定義するには、`Box<dyn Fn(...)>`を使用します。

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

## 9.5 マクロ

マクロは、コードを生成するコードです。Rustには宣言的マクロ（`macro_rules!`）と手続き的マクロの2種類があります。

### 宣言的マクロ

`macro_rules!`を使用して、パターンマッチングに基づくマクロを定義できます。

```rust
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 使用例
let v = vec![1, 2, 3];
```

### 手続き的マクロ

手続き的マクロは、TokenStreamを入力として受け取り、コードを生成します。3種類あります：

1. カスタム派生マクロ（`#[derive]`）
2. 属性様マクロ（`#[attribute]`）
3. 関数様マクロ（`macro_name!(...)`）

```rust
use proc_macro;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // 入力トークンを解析してコードを生成
}
```

## 9.6 その他の高度な機能

### パターンマッチングの詳細

パターンマッチングには、様々な高度な機能があります。

```rust
// マッチガード
match x {
    Some(n) if n < 5 => println!("less than five: {}", n),
    Some(n) => println!("{}", n),
    None => (),
}

// @バインディング
match x {
    Some(n @ 1..=5) => println!("Got a range element {}", n),
    Some(n) => println!("{}", n),
    None => (),
}
```

### 高度なライフタイム

ライフタイムの省略規則や、ライフタイムの境界を指定する方法があります。

```rust
// 'static ライフタイム
let s: &'static str = "I have a static lifetime.";

// ライフタイム境界
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### const ジェネリクス

Rustでは、型パラメータだけでなく、定数パラメータも使用できます。

```rust
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn main() {
    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3, 4, 5];
    
    display_array(arr1);  // [1, 2, 3]
    display_array(arr2);  // [1, 2, 3, 4, 5]
}
```

## 演習

1. `unsafe`を使用して、2つの可変参照を作成し、両方から同じデータにアクセスするプログラムを作成してください。なぜこれが安全でないのかを説明してください。
2. 関連型を使用して、独自のイテレータトレイトを実装してください。
3. 宣言的マクロを作成して、指定された回数だけ式を繰り返し実行する機能を実装してください。

## 次のステップ

これでRustの高度な機能について学びました。次のモジュールでは、実践的なプロジェクトに取り組みます。

[モジュール10: 実践プロジェクト](10_projects.md)に進みましょう。