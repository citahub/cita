#[macro_export]
macro_rules! micro_service_init {
    ($x:expr, $y:expr) => {
        dotenv::dotenv().ok();
        // Always print backtrace on panic.
        ::std::env::set_var("RUST_BACKTRACE", "full");

        //exit process when panic
        set_panic_handler();

        // log4rs config
        logger::init_config($x);
        info!($y);
    };
}
