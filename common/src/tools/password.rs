///
/// 生成密码的工具方法
///
use rand::Rng;
use rand::distributions::Alphanumeric;

pub fn generate_password(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}