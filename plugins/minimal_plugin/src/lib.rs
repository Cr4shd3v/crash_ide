use std::thread::sleep;
use std::time::Duration;
use crash_ide_plugin_types::{PluginInfo, PluginMessage};

#[no_mangle]
fn _plugin_main() {
    PluginInfo {
        technical_name: "minimal-plugin".to_string(),
        display_name: "Minimal Plugin".to_string(),
    }.register();

    loop {
        PluginMessage::PrintLn("Hello from plugin!".to_string()).send();
        sleep(Duration::from_secs(1));
    }
}