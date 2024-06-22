use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub struct PrintLn {
    pub text: String,
}

#[macro_export]
macro_rules! plugin_println {
    ($string:expr) => {
        crash_ide_plugin_api::ServerboundPluginMessage::PrintLn(crash_ide_plugin_api::PrintLn { text: $string.into() }).send();
    };
}

