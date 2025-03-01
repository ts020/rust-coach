# モジュール6: モジュールとパッケージ

Rustのコードを整理し、再利用可能にするためには、モジュールシステムを理解することが重要です。Rustのモジュールシステムには、パッケージ、クレート、モジュール、パスという概念があります。

## 6.1 パッケージとクレート

### パッケージ

パッケージは、1つ以上のクレートを含み、特定の機能セットを提供するものです。パッケージには`Cargo.toml`ファイルがあり、クレートのビルド方法が記述されています。

### クレート

クレートは、モジュールのツリーであり、ライブラリまたは実行可能ファイルを生成します。

- **バイナリクレート**: 実行可能なプログラムを生成するクレート（`src/main.rs`）
- **ライブラリクレート**: 他のプログラムで使用される機能を提供するクレート（`src/lib.rs`）

一つのパッケージには、複数のバイナリクレートを含めることができますが、ライブラリクレートは最大で1つです。

```
my_package/
├── Cargo.toml
├── src/
│   ├── main.rs      // バイナリクレートのルート
│   ├── lib.rs       // ライブラリクレートのルート
│   └── bin/         // 追加のバイナリクレート
│       ├── app1.rs
│       └── app2.rs
```

## 6.2 モジュール

モジュールは、コードを論理的なグループに整理するための仕組みです。モジュールは、コードの可視性（プライバシー）を制御し、名前空間を提供します。

### モジュールの定義

モジュールは`mod`キーワードを使用して定義します。

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

### モジュールツリー

モジュールは、ファイルシステムのディレクトリのように、階層構造を形成します。

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### パスを使用したモジュール内の項目の参照

モジュール内の項目を参照するには、パスを使用します。

- **絶対パス**: クレートのルートから始まるパス
- **相対パス**: 現在のモジュールから始まるパス

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 相対パス
    front_of_house::hosting::add_to_waitlist();
}
```

### pubキーワードによる公開

デフォルトでは、モジュール内の項目はプライベートです。`pub`キーワードを使用して、項目を公開できます。

```rust
mod front_of_house {
    pub mod hosting {  // hostingモジュールを公開
        pub fn add_to_waitlist() {}  // 関数を公開
    }
}
```

### superキーワード

`super`キーワードを使用すると、親モジュールからの相対パスを作成できます。

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();  // 親モジュールのserve_order関数を呼び出す
    }
    
    fn cook_order() {}
}
```

### 構造体と列挙型の公開

構造体を公開する場合、フィールドも個別に公開する必要があります。

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // 公開フィールド
        seasonal_fruit: String, // プライベートフィールド
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");  // OK: toastは公開フィールド
    // meal.seasonal_fruit = String::from("blueberries");  // エラー: seasonal_fruitはプライベート
}
```

列挙型を公開する場合、すべてのバリアントが自動的に公開されます。

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## 6.3 useキーワード

`use`キーワードを使用すると、パスをスコープに取り込み、項目を短い名前で参照できます。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;  // hostingモジュールをスコープに取り込む

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();  // 短い名前で参照
}
```

### 慣用的なuseパス

関数の場合は、親モジュールをスコープに取り込むのが慣用的です。

```rust
use crate::front_of_house::hosting;  // 関数の親モジュール

// hosting::add_to_waitlist();
```

構造体、列挙型、その他の項目の場合は、完全なパスを指定するのが慣用的です。

```rust
use std::collections::HashMap;  // 構造体の完全なパス

// let mut map = HashMap::new();
```

### asキーワード

`as`キーワードを使用すると、取り込んだ項目に新しい名前を付けることができます。

```rust
use std::fmt::Result;
use std::io::Result as IoResult;  // 名前の衝突を避ける

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```

### 再エクスポート

`pub use`を使用すると、取り込んだ項目を再エクスポートできます。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;  // hostingモジュールを再エクスポート

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

### ネストされたパス

複数の項目を同じクレートやモジュールから取り込む場合、ネストされたパスを使用できます。

```rust
// 個別に取り込む場合
use std::cmp::Ordering;
use std::io;

// ネストされたパスを使用する場合
use std::{cmp::Ordering, io};
```

`self`を使用して、同じモジュールからパスとモジュール自体を取り込むことができます。

```rust
use std::io;
use std::io::Write;

// 上記と同等
use std::io::{self, Write};
```

### グロブ演算子

`*`（グロブ演算子）を使用すると、モジュール内のすべての公開項目を取り込むことができます。

```rust
use std::collections::*;  // std::collectionsの全ての公開項目を取り込む
```

グロブ演算子は、テストモジュールなど、特定の状況でのみ使用することをお勧めします。

## 6.4 モジュールを別のファイルに分割する

大きなプロジェクトでは、モジュールを別のファイルに分割することが一般的です。

### モジュールファイルの作成

モジュール名と同じ名前のファイルを作成します。

```rust
// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

### モジュールの読み込み

`mod`宣言を使用して、ファイルからモジュールを読み込みます。

```rust
// src/lib.rs
mod front_of_house;  // front_of_house.rsからモジュールを読み込む

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

### サブモジュールの分割

サブモジュールも別のファイルに分割できます。その場合、親モジュールと同じ名前のディレクトリを作成し、その中にサブモジュールのファイルを配置します。

```
src/
├── front_of_house.rs       // front_of_houseモジュール
├── front_of_house/         // front_of_houseモジュールのサブモジュール用ディレクトリ
│   └── hosting.rs          // hostingサブモジュール
└── lib.rs                  // クレートのルート
```

```rust
// src/front_of_house.rs
pub mod hosting;  // hosting.rsからサブモジュールを読み込む
```

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

## 6.5 Cargoとcrates.io

### 依存関係の追加

外部クレートを使用するには、`Cargo.toml`ファイルに依存関係を追加します。

```toml
[dependencies]
rand = "0.8.5"
```

### クレートの公開

自分のクレートを[crates.io](https://crates.io/)に公開するには、以下の手順を実行します。

1. [crates.io](https://crates.io/)でアカウントを作成
2. APIキーを取得
3. `cargo login`コマンドでAPIキーを登録
4. `Cargo.toml`にメタデータを追加
5. `cargo publish`コマンドでクレートを公開

```toml
[package]
name = "my_crate"
version = "0.1.0"
edition = "2021"
description = "A brief description of my crate"
license = "MIT OR Apache-2.0"
```

### ワークスペース

複数の関連するパッケージを一緒に開発する場合、Cargoワークスペースを使用できます。

```
add/
├── Cargo.toml
├── add-one/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── adder/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

```toml
# add/Cargo.toml
[workspace]
members = [
    "add-one",
    "adder",
]
```

## 演習

1. 簡単な計算機ライブラリを作成し、加算、減算、乗算、除算の機能をモジュールに分割してください。
2. 外部クレートを使用して、ランダムな数値を生成するプログラムを作成してください。
3. 複数のモジュールを持つライブラリクレートと、それを使用するバイナリクレートを含むパッケージを作成してください。

## 次のステップ

これでRustのモジュールとパッケージについて学びました。次のモジュールでは、コレクションについて学びます。

[モジュール7: コレクション](07_collections.md)に進みましょう。