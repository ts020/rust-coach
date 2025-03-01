# モジュール1: Rustの紹介と環境構築

## 1.1 Rustとは

Rustは、Mozilla Research によって開発された、パフォーマンス、信頼性、生産性の3つを兼ね備えたシステムプログラミング言語です。以下の特徴があります：

- **メモリ安全性**: コンパイル時にメモリ関連のバグを防止
- **並行性**: データ競合のないスレッド安全なコードを書くことが可能
- **ゼロコスト抽象化**: 高水準の抽象化を提供しながらも低レベルの制御を可能に
- **パフォーマンス**: C/C++と同等の実行速度
- **クロスプラットフォーム**: 多くのプラットフォームで動作

## 1.2 Rustが解決する問題

Rustは以下のような問題を解決するために設計されています：

- メモリ安全性とパフォーマンスのトレードオフ
- 並行プログラミングの難しさ
- コード保守性と拡張性

## 1.3 Rustの使用例

Rustは以下のような分野で活用されています：

- システムプログラミング
- WebAssembly
- ネットワークサービス
- 組み込みシステム
- ブロックチェーン
- ゲーム開発

## 1.4 環境構築

### Rustupのインストール

Rustをインストールする最も簡単な方法は、公式のインストーラである「Rustup」を使用することです。

#### Linuxまたは macOS

ターミナルで以下のコマンドを実行します：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows

[Rustupのダウンロードページ](https://www.rust-lang.org/tools/install)からインストーラをダウンロードして実行します。

### インストールの確認

インストールが完了したら、以下のコマンドでRustのバージョンを確認できます：

```bash
rustc --version
cargo --version
```

### 開発環境の設定

Rustの開発には以下のツールが役立ちます：

- **Visual Studio Code**: [Rust拡張機能](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)をインストール
- **IntelliJ IDEA / CLion**: [Rust プラグイン](https://plugins.jetbrains.com/plugin/8182-rust)をインストール
- **Vim / Neovim**: [rust.vim](https://github.com/rust-lang/rust.vim)などのプラグインを使用

## 1.5 最初のRustプログラム

### プロジェクトの作成

新しいRustプロジェクトを作成するには、以下のコマンドを使用します：

```bash
cargo new hello_rust
cd hello_rust
```

これにより、`hello_rust`ディレクトリが作成され、その中に基本的なプロジェクト構造が生成されます。

### コードの確認

生成された`src/main.rs`ファイルには、以下のようなコードが含まれています：

```rust
fn main() {
    println!("Hello, world!");
}
```

### プログラムの実行

プログラムを実行するには、以下のコマンドを使用します：

```bash
cargo run
```

これにより、プログラムがコンパイルされ、実行されます。

## 1.6 Cargoの基本

Cargoは、Rustのパッケージマネージャおよびビルドシステムです。主なコマンドは以下の通りです：

- `cargo new`: 新しいプロジェクトを作成
- `cargo build`: プロジェクトをビルド
- `cargo run`: プロジェクトをビルドして実行
- `cargo test`: テストを実行
- `cargo doc`: ドキュメントを生成
- `cargo publish`: クレートを公開

## 演習

1. Rustをインストールし、バージョンを確認してください。
2. 新しいRustプロジェクトを作成し、「Hello, Rust!」と表示するプログラムを実行してください。
3. Cargoのドキュメントを生成し、ブラウザで確認してください（`cargo doc --open`）。

## 次のステップ

これでRustの環境構築は完了です。次のモジュールでは、Rustの基本的な構文と概念について学びます。

[モジュール2: 基本的な構文と概念](02_basic_syntax.md)に進みましょう。