#[macro_export]
macro_rules! __route {
    ($router:ident,) => {};
    ($router:ident, GET $path:expr => $handler:expr; $($tail:tt)*) => {
        $router = $router.route($path, axum::routing::get($handler));
        $crate::__route!($router, $($tail)*)
    };
    ($router:ident, POST $path:expr => $handler:expr; $($tail:tt)*) => {
        $router = $router.route($path, axum::routing::post($handler));
        $crate::__route!($router, $($tail)*)
    };
    ($router:ident, DELETE $path:expr => $handler:expr; $($tail:tt)*) => {
        $router = $router.route($path, axum::routing::delete($handler));
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
