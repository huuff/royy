#[macro_export]
macro_rules! __route {
    ($router:ident,) => {};
    ($router:ident, $method:ident $path:expr => $handler:expr; $($tail:tt)*) => {
        $router = $router.route($path, axum::routing::casey::lower!($method)($handler));
        $crate::__route!($router, $($tail)*)
    };
}

// TODO: Document this (and maybe test it?)
#[macro_export]
macro_rules! router {
    ($router:ident, { $($tail:tt)* } ) => {
        $crate::__route!(router, $($tail)*);
    };
    ($($tail:tt)*) => {
        {
            let mut router = axum::Router::new();
            $crate::__route!(router, $($tail)*);
            router
        }
    };
}
