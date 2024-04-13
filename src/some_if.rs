/// Creates a `Some` with the first argument if the second argument evaluates to true
/// or a `None` otherwise
/// ```
/// #[macro_use] extern crate royy;
/// # fn main() {
///     assert_eq!(some_if!("even", 8 % 2 == 0), Some("even"))
/// # }
/// ```
#[macro_export]
macro_rules! some_if {
    ($res:expr, $cond:expr) => {
        if $cond {
            Some($res)
        } else {
            None
        }
    };
}
