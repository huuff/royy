/// Creates a `Some` with the first argument if the second argument evaluates to true
/// or a `None` otherwise
/// ```
/// #[macro_use] extern crate royy;
/// # fn main() {
///     assert_eq!(some_if!(8 % 2 == 0 => "even"), Some("even"))
/// # }
/// ```
#[macro_export]
macro_rules! some_if {
    ($cond:expr => $res:expr) => {
        if $cond {
            Some($res)
        } else {
            None
        }
    };
    ($var:ident,  ) => {};
}
