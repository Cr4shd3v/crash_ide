use std::thread::sleep;
use std::time::Duration;

use crash_ide_plugin_api::*;

plugin_main!(start);

fn start() {
    PluginInfo {
        technical_name: "minimal-plugin".to_string(),
        display_name: "Minimal Plugin".to_string(),
        // config_fields: vec![
        //     ConfigField::new("test_1", "Test 1 Box", ConfigFieldType::Bool),
        //     ConfigField::new("test_2", "Test 2 Box", ConfigFieldType::Bool),
        // ],
    }.register();

    loop {
        sleep(Duration::from_secs(1));
        plugin_println!("Hello from plugin!");
    }
}