# モジュール8: 並行処理

Rustは、安全で効率的な並行処理をサポートするために設計されています。所有権システムと型システムにより、コンパイル時に多くの一般的な並行処理のバグを防止できます。

## 8.1 スレッド

### スレッドの作成

Rustの標準ライブラリは、1:1スレッドモデルを実装しています。`thread::spawn`関数を使用して、新しいスレッドを作成できます。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // 新しいスレッドを生成
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // メインスレッドでの処理
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // スレッドの終了を待機
    handle.join().unwrap();
}
```

### スレッド間でのデータの共有

クロージャで外部の変数を使用するには、`move`キーワードを使用して所有権を移動させる必要があります。

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // v はスレッドに移動されたため、ここでは使用できない
    // println!("{:?}", v);  // エラー

    handle.join().unwrap();
}
```

## 8.2 メッセージパッシング

Rustでは、スレッド間の通信にメッセージパッシングを使用できます。`std::sync::mpsc`モジュールは、マルチプロデューサ、シングルコンシューマ（mpsc）チャネルを提供します。

### 基本的なチャネルの使用

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    // チャネルを作成
    let (tx, rx) = mpsc::channel();

    // 送信側のスレッド
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // val はチャネルに移動されたため、ここでは使用できない
        // println!("val is {}", val);  // エラー
    });

    // 受信側（メインスレッド）
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

### 複数の値の送信

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 受信側をイテレータとして使用
    for received in rx {
        println!("Got: {}", received);
    }
}
```

### 複数の送信者

`tx`をクローンすることで、複数の送信者を作成できます。

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    // tx1（元のtx）
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // tx2（txのクローン）
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 両方のスレッドからのメッセージを受信
    for received in rx {
        println!("Got: {}", received);
    }
}
```

## 8.3 共有状態の並行処理

### Mutex<T>

`Mutex<T>`（相互排除）は、一度に1つのスレッドだけがデータにアクセスできるようにするための同期プリミティブです。

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }  // ロックは自動的に解放される

    println!("m = {:?}", m);  // m = Mutex { data: 6 }
}
```

### スレッド間でのMutex<T>の共有

`Arc<T>`（アトミック参照カウント）を使用して、スレッド間で`Mutex<T>`を安全に共有できます。

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());  // Result: 10
}
```

### デッドロックの回避

Rustの型システムは、多くの並行処理のバグを防止しますが、デッドロックは依然として可能です。デッドロックを回避するためには、以下のような方法があります：

1. ロックを取得する順序を一貫させる
2. ロックを保持する時間を最小限にする
3. 可能な限り細粒度のロックを使用する

## 8.4 Send と Sync トレイト

Rustの並行処理の安全性は、`Send`と`Sync`という2つのトレイトに基づいています。

### Send トレイト

`Send`トレイトは、型の所有権がスレッド間で転送できることを示します。ほとんどのRustの型は`Send`ですが、`Rc<T>`のような例外もあります。

```rust
// Sendを実装している型はスレッド間で所有権を転送できる
let v = vec![1, 2, 3];
thread::spawn(move || {
    println!("{:?}", v);  // OK: Vecはsend
});

// Rcはスレッド間で安全に転送できない
use std::rc::Rc;
let rc = Rc::new(1);
thread::spawn(move || {
    println!("{}", rc);  // エラー: RcはSendを実装していない
});
```

### Sync トレイト

`Sync`トレイトは、型が複数のスレッドから同時に参照されても安全であることを示します。つまり、`&T`が`Send`である場合、`T`は`Sync`です。

```rust
// Syncを実装している型は複数のスレッドから同時に参照できる
use std::sync::{Arc, Mutex};

let mutex = Arc::new(Mutex::new(0));
let mutex2 = Arc::clone(&mutex);

thread::spawn(move || {
    let mut num = mutex2.lock().unwrap();
    *num += 1;
});  // OK: MutexはSync
```

## 8.5 並行処理パターン

### ワーカープール

複数のワーカースレッドを作成し、タスクを分散させるパターンです。

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            
            match job {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} disconnected; shutting down.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

### 非同期プログラミング

Rustエコシステムには、非同期プログラミングをサポートするためのクレートがあります。最も一般的なのは`tokio`と`async-std`です。

```rust
// tokioを使用した非同期プログラミングの例
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("Hello from spawned task");
        sleep(Duration::from_secs(1)).await;
        println!("Spawned task done");
        42
    });

    println!("Hello from main");
    sleep(Duration::from_millis(500)).await;
    println!("Main task doing something else");

    let result = handle.await.unwrap();
    println!("Got result: {}", result);
}
```

## 演習

1. 複数のスレッドを使用して、大きな配列の要素の合計を計算するプログラムを作成してください。
2. チャネルを使用して、生産者と消費者のパターンを実装してください。生産者は数値を生成し、消費者はそれらを処理します。
3. スレッドプールを実装し、複数のタスクを並行して実行するプログラムを作成してください。

## 次のステップ

これでRustの並行処理について学びました。次のモジュールでは、高度な機能について学びます。

[モジュール9: 高度な機能](09_advanced_features.md)に進みましょう。