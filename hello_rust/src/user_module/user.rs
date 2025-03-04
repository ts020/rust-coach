pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

// ユーザー情報を表示する関数（不変参照を使用）
pub fn display_user(user: &User) {
    println!("ユーザー名: {}", user.username);
    println!("メール: {}", user.email);
    println!("ログイン回数: {}", user.sign_in_count);
    println!("アクティブ: {}", user.active);
}

// ユーザーのログイン回数を増やす関数（可変参照を使用）
pub fn increment_login(user: &mut User) {
    user.sign_in_count += 1;
}
