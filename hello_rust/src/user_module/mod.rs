// src/user_module/mod.rs

// モジュール内の他のファイルをサブモジュールとして公開
pub mod user;
pub mod utils;

// モジュールから直接公開する要素
// （必要に応じてサブモジュールから再エクスポート）
pub use user::User;
pub use user::{display_user, increment_login};
