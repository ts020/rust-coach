pub fn validate_email(email: &str) -> bool {
    // 簡易的なメールアドレス検証
    email.contains('@')
}
