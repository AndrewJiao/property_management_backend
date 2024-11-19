///
/// 封装一些默认自定义操作
///
pub trait AutoOperation {
    fn create_time(self) -> Self;
    fn update_time(self) -> Self;
}
