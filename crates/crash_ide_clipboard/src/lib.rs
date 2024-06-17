use std::borrow::Cow;
use std::sync::{Arc, Mutex};
use arboard::Error;
use bevy::prelude::*;

pub struct CrashIDEClipboardPlugin;

impl Plugin for CrashIDEClipboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_clipboard)
        ;
    }
}

#[derive(Resource)]
pub struct Clipboard {
    inner: Arc<Mutex<arboard::Clipboard>>,
}

impl Clipboard {
    pub fn get_text(&self) -> Result<String, Error> {
        self.inner.lock().unwrap().get_text()
    }

    pub fn set_text<'a, T: Into<Cow<'a, str>>>(&self, text: T) -> Result<(), Error> {
        self.inner.lock().unwrap().set_text(text)
    }
}

fn init_clipboard(mut commands: Commands) {
    let Ok(clipboard) = arboard::Clipboard::new() else {
        return;
    };

    commands.insert_resource(Clipboard {
        inner: Arc::new(Mutex::new(clipboard)),
    });
}
