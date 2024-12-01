pub mod serde;
pub mod validator;
pub mod lock;
///
/// 查看当前泛型的类型
///
pub fn debug_type<T>(_value: &T) {
    println!("{}", std::any::type_name::<T>());
}
pub mod time;