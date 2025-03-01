# モジュール10: 実践プロジェクト

このモジュールでは、これまでに学んだRustの知識を活用して、実践的なプロジェクトに取り組みます。各プロジェクトは、特定の概念やテクニックに焦点を当てています。

## 10.1 コマンドラインツール

### プロジェクト1: ファイル検索ツール

簡単なファイル検索ツールを作成します。このツールは、指定されたディレクトリ内のファイルから、指定されたパターンを検索します。

#### 要件

1. コマンドライン引数でパターンとディレクトリを受け取る
2. 指定されたディレクトリ内のすべてのファイルを再帰的に検索
3. パターンにマッチする行を含むファイルと行番号を表示

#### 実装のヒント

```rust
use std::env;
use std::fs;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <directory>", args[0]);
        std::process::exit(1);
    }
    
    let pattern = &args[1];
    let directory = &args[2];
    
    search_files(pattern, directory)?;
    
    Ok(())
}

fn search_files(pattern: &str, directory: &str) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(directory)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            search_files(pattern, path.to_str().unwrap())?;
        } else {
            search_file(pattern, &path)?;
        }
    }
    
    Ok(())
}

fn search_file(pattern: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    
    for (i, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            println!("{}:{}:{}", path.display(), i + 1, line);
        }
    }
    
    Ok(())
}
```

#### 拡張課題

1. 大文字と小文字を区別しないオプションを追加
2. 正規表現を使用したパターンマッチングを実装
3. 検索結果をファイルに出力するオプションを追加

### プロジェクト2: TODOリストマネージャ

コマンドラインベースのTODOリストマネージャを作成します。

#### 要件

1. タスクの追加、表示、完了、削除ができる
2. タスクをファイルに保存し、プログラム終了後も保持できる
3. タスクに優先度や期限を設定できる

#### 実装のヒント

```rust
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::env;

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
    priority: Priority,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Task {
    fn new(id: usize, description: String, priority: Priority) -> Task {
        Task {
            id,
            description,
            completed: false,
            priority,
        }
    }
    
    fn toggle_completion(&mut self) {
        self.completed = !self.completed;
    }
}

struct TodoList {
    tasks: Vec<Task>,
    file_path: String,
}

impl TodoList {
    fn new(file_path: String) -> Result<TodoList, io::Error> {
        let mut todo_list = TodoList {
            tasks: Vec::new(),
            file_path,
        };
        
        todo_list.load()?;
        
        Ok(todo_list)
    }
    
    fn add_task(&mut self, description: String, priority: Priority) {
        let id = self.tasks.len() + 1;
        let task = Task::new(id, description, priority);
        self.tasks.push(task);
        self.save().unwrap();
    }
    
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks.");
            return;
        }
        
        for task in &self.tasks {
            let status = if task.completed { "[x]" } else { "[ ]" };
            let priority = match task.priority {
                Priority::Low => "Low",
                Priority::Medium => "Medium",
                Priority::High => "High",
            };
            
            println!("{} {} (Priority: {}) - {}", task.id, status, priority, task.description);
        }
    }
    
    fn complete_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.toggle_completion();
            self.save().unwrap();
            Ok(())
        } else {
            Err(format!("Task with ID {} not found.", id))
        }
    }
    
    fn delete_task(&mut self, id: usize) -> Result<(), String> {
        let position = self.tasks.iter().position(|t| t.id == id);
        
        if let Some(pos) = position {
            self.tasks.remove(pos);
            self.save().unwrap();
            Ok(())
        } else {
            Err(format!("Task with ID {} not found.", id))
        }
    }
    
    fn load(&mut self) -> Result<(), io::Error> {
        let path = Path::new(&self.file_path);
        
        if !path.exists() {
            return Ok(());
        }
        
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        // ファイルからタスクを読み込む実装（簡略化のため省略）
        
        Ok(())
    }
    
    fn save(&self) -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        
        // タスクをファイルに保存する実装（簡略化のため省略）
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = "todo.txt".to_string();
    let mut todo_list = TodoList::new(file_path)?;
    
    if args.len() < 2 {
        println!("Usage: {} <command> [args...]", args[0]);
        println!("Commands: add, list, complete, delete");
        return Ok(());
    }
    
    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: {} add <description> [priority]", args[0]);
                return Ok(());
            }
            
            let description = &args[2];
            let priority = if args.len() > 3 {
                match args[3].to_lowercase().as_str() {
                    "high" => Priority::High,
                    "medium" => Priority::Medium,
                    _ => Priority::Low,
                }
            } else {
                Priority::Medium
            };
            
            todo_list.add_task(description.clone(), priority);
            println!("Task added.");
        },
        "list" => {
            todo_list.list_tasks();
        },
        "complete" => {
            if args.len() < 3 {
                println!("Usage: {} complete <id>", args[0]);
                return Ok(());
            }
            
            let id = args[2].parse::<usize>()?;
            match todo_list.complete_task(id) {
                Ok(_) => println!("Task {} marked as completed.", id),
                Err(e) => println!("{}", e),
            }
        },
        "delete" => {
            if args.len() < 3 {
                println!("Usage: {} delete <id>", args[0]);
                return Ok(());
            }
            
            let id = args[2].parse::<usize>()?;
            match todo_list.delete_task(id) {
                Ok(_) => println!("Task {} deleted.", id),
                Err(e) => println!("{}", e),
            }
        },
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Commands: add, list, complete, delete");
        }
    }
    
    Ok(())
}
```

#### 拡張課題

1. タスクにタグを追加し、タグでフィルタリングできるようにする
2. タスクの編集機能を追加
3. タスクの期限が近づいたら通知する機能を追加

## 10.2 Webアプリケーション

### プロジェクト3: シンプルなWebサーバー

HTTPリクエストを処理する基本的なWebサーバーを作成します。

#### 要件

1. 指定されたポートでHTTPリクエストをリッスン
2. 静的ファイルを提供
3. 簡単なルーティングシステムを実装

#### 実装のヒント

```rust
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running at http://127.0.0.1:7878/");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_hello = b"GET /hello HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(get_hello) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        String::from("<html><body><h1>404 Not Found</h1></body></html>")
    });

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

#### 拡張課題

1. マルチスレッドサーバーをスレッドプールを使用して最適化
2. 動的なコンテンツ生成をサポート
3. HTTPメソッド（GET、POST、PUT、DELETE）を処理

### プロジェクト4: RESTful API

シンプルなRESTful APIを作成します。

#### 要件

1. JSONデータを処理
2. CRUD操作（作成、読み取り、更新、削除）をサポート
3. データを永続化

#### 実装のヒント

```rust
// Cargo.toml
// [dependencies]
// rocket = "0.5.0-rc.1"
// rocket_contrib = "0.5.0-rc.1"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}

struct AppState {
    tasks: Mutex<HashMap<usize, Task>>,
    next_id: Mutex<usize>,
}

#[get("/tasks")]
fn get_tasks(state: &rocket::State<AppState>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();
    let tasks_vec: Vec<Task> = tasks.values().cloned().collect();
    Json(tasks_vec)
}

#[get("/tasks/<id>")]
fn get_task(id: usize, state: &rocket::State<AppState>) -> Option<Json<Task>> {
    let tasks = state.tasks.lock().unwrap();
    tasks.get(&id).map(|task| Json(task.clone()))
}

#[post("/tasks", data = "<task>")]
fn create_task(task: Json<Task>, state: &rocket::State<AppState>) -> Json<Task> {
    let mut next_id = state.next_id.lock().unwrap();
    let mut tasks = state.tasks.lock().unwrap();
    
    let id = *next_id;
    *next_id += 1;
    
    let new_task = Task {
        id,
        title: task.title.clone(),
        completed: task.completed,
    };
    
    tasks.insert(id, new_task.clone());
    
    Json(new_task)
}

#[put("/tasks/<id>", data = "<task>")]
fn update_task(id: usize, task: Json<Task>, state: &rocket::State<AppState>) -> Option<Json<Task>> {
    let mut tasks = state.tasks.lock().unwrap();
    
    if tasks.contains_key(&id) {
        let updated_task = Task {
            id,
            title: task.title.clone(),
            completed: task.completed,
        };
        
        tasks.insert(id, updated_task.clone());
        Some(Json(updated_task))
    } else {
        None
    }
}

#[delete("/tasks/<id>")]
fn delete_task(id: usize, state: &rocket::State<AppState>) -> Option<Json<Task>> {
    let mut tasks = state.tasks.lock().unwrap();
    tasks.remove(&id).map(|task| Json(task))
}

#[launch]
fn rocket() -> _ {
    let app_state = AppState {
        tasks: Mutex::new(HashMap::new()),
        next_id: Mutex::new(1),
    };
    
    rocket::build()
        .manage(app_state)
        .mount("/", routes![get_tasks, get_task, create_task, update_task, delete_task])
}
```

#### 拡張課題

1. ユーザー認証を追加
2. レート制限を実装
3. APIドキュメントを自動生成

## 10.3 システムプログラミング

### プロジェクト5: シンプルなシェル

基本的なコマンドラインシェルを作成します。

#### 要件

1. コマンドの実行
2. パイプとリダイレクト
3. 環境変数の管理

#### 実装のヒント

```rust
use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "exit" {
            break;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];
        
        match command {
            "cd" => {
                let new_dir = args.get(0).map_or("/", |s| *s);
                if let Err(e) = env::set_current_dir(new_dir) {
                    eprintln!("cd: {}", e);
                }
            },
            "pwd" => {
                if let Ok(path) = env::current_dir() {
                    println!("{}", path.display());
                }
            },
            "echo" => {
                println!("{}", args.join(" "));
            },
            _ => {
                let output = Command::new(command)
                    .args(args)
                    .output();
                
                match output {
                    Ok(output) => {
                        io::stdout().write_all(&output.stdout)?;
                        io::stderr().write_all(&output.stderr)?;
                    },
                    Err(e) => {
                        eprintln!("{}: {}", command, e);
                    }
                }
            }
        }
    }
    
    Ok(())
}
```

#### 拡張課題

1. コマンド履歴機能を追加
2. タブ補完を実装
3. ジョブ制御（バックグラウンド実行など）をサポート

### プロジェクト6: ファイルシステムモニター

ファイルシステムの変更を監視するツールを作成します。

#### 要件

1. 指定されたディレクトリの変更を監視
2. ファイルの作成、変更、削除を検出
3. 変更の通知

#### 実装のヒント

```rust
// Cargo.toml
// [dependencies]
// notify = "4.0.0"
// chrono = "0.4"

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;
use chrono::Local;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }
    
    let directory = &args[1];
    
    println!("Monitoring directory: {}", directory);
    
    let (tx, rx) = channel();
    
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch(directory, RecursiveMode::Recursive).unwrap();
    
    loop {
        match rx.recv() {
            Ok(event) => {
                let time = Local::now().format("%H:%M:%S").to_string();
                
                match event {
                    DebouncedEvent::Create(path) => {
                        println!("[{}] File created: {:?}", time, path);
                    },
                    DebouncedEvent::Write(path) => {
                        println!("[{}] File modified: {:?}", time, path);
                    },
                    DebouncedEvent::Remove(path) => {
                        println!("[{}] File deleted: {:?}", time, path);
                    },
                    DebouncedEvent::Rename(from, to) => {
                        println!("[{}] File renamed: {:?} -> {:?}", time, from, to);
                    },
                    _ => {}
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
```

#### 拡張課題

1. 変更をログファイルに記録
2. 特定のパターンに一致するファイルのみを監視
3. 変更に応じてカスタムアクションを実行

## 10.4 ゲーム開発

### プロジェクト7: テキストアドベンチャーゲーム

シンプルなテキストベースのアドベンチャーゲームを作成します。

#### 要件

1. 部屋や場所の移動
2. アイテムの収集と使用
3. 簡単なパズルや敵

#### 実装のヒント

```rust
use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Room {
    description: String,
    exits: HashMap<String, String>,
    items: Vec<String>,
}

#[derive(Debug)]
struct Game {
    rooms: HashMap<String, Room>,
    current_room: String,
    inventory: Vec<String>,
    game_over: bool,
}

impl Game {
    fn new() -> Game {
        let mut rooms = HashMap::new();
        
        // 部屋の定義
        rooms.insert(
            "start".to_string(),
            Room {
                description: "You are in a dark room. There is a door to the north.".to_string(),
                exits: {
                    let mut map = HashMap::new();
                    map.insert("north".to_string(), "corridor".to_string());
                    map
                },
                items: vec!["flashlight".to_string()],
            },
        );
        
        rooms.insert(
            "corridor".to_string(),
            Room {
                description: "You are in a long corridor. There are doors to the north and south.".to_string(),
                exits: {
                    let mut map = HashMap::new();
                    map.insert("north".to_string(), "treasure".to_string());
                    map.insert("south".to_string(), "start".to_string());
                    map
                },
                items: vec![],
            },
        );
        
        rooms.insert(
            "treasure".to_string(),
            Room {
                description: "You found the treasure room! There is a golden key here.".to_string(),
                exits: {
                    let mut map = HashMap::new();
                    map.insert("south".to_string(), "corridor".to_string());
                    map
                },
                items: vec!["key".to_string()],
            },
        );
        
        Game {
            rooms,
            current_room: "start".to_string(),
            inventory: vec![],
            game_over: false,
        }
    }
    
    fn get_current_room(&self) -> &Room {
        self.rooms.get(&self.current_room).unwrap()
    }
    
    fn describe_room(&self) {
        let room = self.get_current_room();
        println!("{}", room.description);
        
        if !room.items.is_empty() {
            println!("You see: {}", room.items.join(", "));
        }
        
        println!("Exits: {}", room.exits.keys().cloned().collect::<Vec<String>>().join(", "));
    }
    
    fn go(&mut self, direction: &str) {
        let room = self.get_current_room();
        
        if let Some(next_room) = room.exits.get(direction) {
            self.current_room = next_room.clone();
            self.describe_room();
        } else {
            println!("You can't go that way.");
        }
    }
    
    fn take(&mut self, item: &str) {
        let room = self.get_current_room();
        
        if room.items.contains(&item.to_string()) {
            let mut new_room = room.clone();
            new_room.items.retain(|i| i != item);
            self.rooms.insert(self.current_room.clone(), new_room);
            self.inventory.push(item.to_string());
            println!("You took the {}.", item);
        } else {
            println!("There is no {} here.", item);
        }
    }
    
    fn inventory(&self) {
        if self.inventory.is_empty() {
            println!("Your inventory is empty.");
        } else {
            println!("Inventory: {}", self.inventory.join(", "));
        }
    }
    
    fn process_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if parts.is_empty() {
            return;
        }
        
        match parts[0] {
            "go" | "move" => {
                if parts.len() > 1 {
                    self.go(parts[1]);
                } else {
                    println!("Go where?");
                }
            },
            "take" | "get" => {
                if parts.len() > 1 {
                    self.take(parts[1]);
                } else {
                    println!("Take what?");
                }
            },
            "inventory" | "i" => {
                self.inventory();
            },
            "look" => {
                self.describe_room();
            },
            "quit" | "exit" => {
                self.game_over = true;
            },
            "help" => {
                println!("Commands: go <direction>, take <item>, inventory, look, quit, help");
            },
            _ => {
                println!("I don't understand that command.");
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    
    println!("Welcome to the Adventure Game!");
    println!("Type 'help' for a list of commands.");
    println!();
    
    game.describe_room();
    
    while !game.game_over {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let command = input.trim().to_lowercase();
        game.process_command(&command);
    }
    
    println!("Thanks for playing!");
}
```

#### 拡張課題

1. セーブとロード機能を追加
2. NPCとの会話システムを実装
3. 戦闘システムを追加

## 10.5 最終プロジェクト

### プロジェクト8: ミニブログエンジン

完全なブログエンジンを作成します。

#### 要件

1. 記事の作成、編集、削除
2. ユーザー認証
3. コメント機能
4. タグとカテゴリ

#### 実装のヒント

このプロジェクトは複雑なため、以下のクレートを使用することをお勧めします：

- `rocket` - Webフレームワーク
- `diesel` - ORMとデータベース接続
- `tera` - テンプレートエンジン
- `serde` - シリアライゼーション/デシリアライゼーション
- `bcrypt` - パスワードハッシュ化

プロジェクト構造の例：

```
blog_engine/
├── Cargo.toml
├── migrations/
│   └── ...
├── src/
│   ├── main.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── post.rs
│   │   └── comment.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── posts.rs
│   │   └── admin.rs
│   ├── templates/
│   │   ├── base.html.tera
│   │   ├── index.html.tera
│   │   ├── post.html.tera
│   │   └── ...
│   └── schema.rs
└── static/
    ├── css/
    ├── js/
    └── images/
```

#### 拡張課題

1. マークダウンサポートを追加
2. 検索機能を実装
3. RSSフィードを生成
4. APIエンドポイントを提供

## 次のステップ

これでRustの学習カリキュラムは完了です。ここからさらに学習を進めるためのリソースをいくつか紹介します：

1. [The Rust Programming Language](https://doc.rust-lang.org/book/) - 公式ドキュメントの詳細な部分
2. [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) - 一般的なタスクのレシピ集
3. [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 例を通じて学ぶRust
4. [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) - Rustのデザインパターン
5. [Rust Algorithm Club](https://github.com/weihanglo/rust-algorithm-club) - Rustでのアルゴリズムとデータ構造
6. [This Week in Rust](https://this-week-in-rust.org/) - Rustエコシステムの最新情報

また、以下のようなプロジェクトに貢献することも、スキルを向上させる良い方法です：

1. [Rust Language](https://github.com/rust-lang/rust) - Rust言語自体
2. [Cargo](https://github.com/rust-lang/cargo) - Rustのパッケージマネージャ
3. [Rustlings](https://github.com/rust-lang/rustlings) - Rustの小さな練習問題

Rustの学習を続け、素晴らしいプロジェクトを作成してください！