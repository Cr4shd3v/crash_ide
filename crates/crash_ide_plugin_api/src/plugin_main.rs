#[macro_export]
macro_rules! plugin_main {
    ($fun:ident) => {
        #[no_mangle]
        fn _plugin_main() {
            $fun();
        }
    };
}