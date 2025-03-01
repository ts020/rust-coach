# モジュール5: エラー処理

Rustのエラー処理は、プログラムの信頼性を高めるための重要な機能です。Rustでは、エラーを回復可能なエラーと回復不可能なエラーに分類しています。

## 5.1 パニック

回復不可能なエラーが発生した場合、Rustは「パニック」を起こします。パニックが発生すると、プログラムはスタックを巻き戻すか、アボートしてプロセスを終了します。

### パニックの発生

`panic!`マクロを使用して、明示的にパニックを発生させることができます。

```rust
fn main() {
    panic!("crash and burn");
}
```

実行すると、以下のようなエラーメッセージが表示されます：

```
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```

### バックトレース

パニックが発生した場所を特定するために、バックトレースを使用できます。環境変数`RUST_BACKTRACE=1`を設定すると、詳細なバックトレースが表示されます。

```bash
$ RUST_BACKTRACE=1 cargo run
```

### 配列の境界外アクセス

Rustでは、配列の境界外アクセスはパニックを引き起こします。

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99]; // パニック: インデックスが範囲外
}
```

## 5.2 Result型によるエラー処理

回復可能なエラーは、`Result<T, E>`列挙型を使用して処理します。

```rust
enum Result<T, E> {
    Ok(T),  // 成功した場合の値
    Err(E), // エラーの場合の値
}
```

### Result型の使用例

ファイルを開く操作は失敗する可能性があるため、`Result`型を返します。

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("ファイルを開けませんでした: {:?}", error);
        },
    };
}
```

### エラーの種類によるマッチング

エラーの種類に応じて異なる処理を行うことができます。

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("ファイルを作成できませんでした: {:?}", e),
            },
            other_error => {
                panic!("ファイルを開けませんでした: {:?}", other_error);
            }
        },
    };
}
```

### エラー処理のショートカット

#### unwrap

`unwrap`メソッドは、`Result`が`Ok`の場合は中の値を返し、`Err`の場合はパニックを起こします。

```rust
let f = File::open("hello.txt").unwrap();
```

#### expect

`expect`メソッドは`unwrap`と似ていますが、パニックメッセージをカスタマイズできます。

```rust
let f = File::open("hello.txt").expect("hello.txtを開けませんでした");
```

#### ?演算子

`?`演算子は、`Result`が`Ok`の場合は中の値を返し、`Err`の場合は現在の関数から早期リターンします。

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

さらに簡潔に書くこともできます：

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

または、標準ライブラリの関数を使用して：

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

### ?演算子とmain関数

`main`関数でも`?`演算子を使用できますが、その場合は戻り値の型を`Result`にする必要があります。

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
```

## 5.3 エラーの伝播

複数の操作を行う関数では、エラーを呼び出し元に伝播させることが一般的です。

```rust
use std::fs;
use std::io;
use std::io::Read;

fn read_and_process() -> Result<String, io::Error> {
    let content = fs::read_to_string("data.txt")?;
    // contentを処理...
    Ok(content)
}

fn main() {
    match read_and_process() {
        Ok(processed_data) => println!("処理結果: {}", processed_data),
        Err(e) => eprintln!("エラーが発生しました: {}", e),
    }
}
```

## 5.4 カスタムエラー型

複雑なアプリケーションでは、独自のエラー型を定義することが役立ちます。

```rust
#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    InvalidInput(String),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}

fn process_data(filename: &str) -> Result<i32, AppError> {
    let content = std::fs::read_to_string(filename)?; // IoErrorは自動的にAppErrorに変換
    
    if content.is_empty() {
        return Err(AppError::InvalidInput("空のファイル".to_string()));
    }
    
    let number: i32 = content.trim().parse()?; // ParseIntErrorは自動的にAppErrorに変換
    Ok(number * 2)
}
```

## 5.5 エラー処理のベストプラクティス

1. **適切なエラー型を選択する**: 回復可能なエラーには`Result`を、回復不可能なエラーには`panic!`を使用する
2. **エラーメッセージを明確にする**: `expect`を使用して、エラーの原因を明確に示す
3. **エラーを伝播させる**: 低レベルの関数からエラーを伝播させ、上位レベルで処理する
4. **カスタムエラー型を使用する**: 複雑なアプリケーションでは、独自のエラー型を定義する
5. **エラーの変換**: `From`トレイトを実装して、異なるエラー型間の変換を容易にする

## 演習

1. ファイルから数値のリストを読み取り、その合計を計算する関数を作成してください。ファイルが存在しない場合や、数値の解析に失敗した場合は適切なエラーを返してください。
2. 独自のエラー型を定義し、異なる種類のエラーを処理できるようにしてください。
3. `?`演算子を使用して、複数のファイル操作を行う関数を作成してください。

## 次のステップ

これでRustのエラー処理について学びました。次のモジュールでは、モジュールとパッケージについて学びます。

[モジュール6: モジュールとパッケージ](06_modules_packages.md)に進みましょう。