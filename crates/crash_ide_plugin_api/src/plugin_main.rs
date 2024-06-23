#[macro_export]
/// Creates the required _plugin_main method.
///
/// Takes a function as argument. Function must have no params and should not have any return value.
macro_rules! plugin_main {
    ($fun:ident) => {
        #[no_mangle]
        fn _plugin_main() {
            $fun();
        }
    };
}