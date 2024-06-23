use bincode::{Decode, Encode};

/// Print a message to IDE console
#[derive(Encode, Decode, Debug)]
pub struct PrintLn {
    /// Message
    pub text: String,
}

#[macro_export]
/// Shortcut macro to print a message to IDE console.
macro_rules! plugin_println {
    ($string:expr) => {
        crash_ide_plugin_api::ServerboundPluginMessage::PrintLn(crash_ide_plugin_api::PrintLn { text: $string.into() }).send();
    };
}

