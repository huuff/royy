macro_rules! route {
    ($router:ident,) => {};
    ($router:ident, $method:ident $path:expr => $handler:expr; $($tail:tt)*) => {
        $router = $router.route($path, axum::routing::casey::lower!($method)($handler));
        route!($router, $($tail)*)
    };
}

// TODO: Document this (and maybe test it?)
#[macro_export]
macro_rules! router {
    ($router:ident, { $($tail:tt)* } ) => {
        route!(router, $($tail)*);
    };
    ($($tail:tt)*) => {
        {
            let mut router = axum::Router::new();
            route!(router, $($tail)*);
            router
        }
    };
}
