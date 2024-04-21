#[macro_export]
macro_rules! __route {
    ($router:ident,) => {};
    (@method $method:ident) => { axum::routing::$it };
    ($router:ident, $method:ident $path:expr => $handler:expr; $($tail:tt)*) => {
        $router = $router.route($path, $crate::_route!(casey::lower!($method))($handler));
        $crate::__route!($router, $($tail)*)
    };
}

// TODO: Document this (and maybe test it?)
#[macro_export]
macro_rules! router {
    ($($tail:tt)*) => {
        {
            let mut router = axum::Router::new();
            $crate::__route!(router, $($tail)*);
            router
        }
    };
}
