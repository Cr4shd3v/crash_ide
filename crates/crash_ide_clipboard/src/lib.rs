#![warn(missing_docs)]
//! This crate implements the arboard Clipboard as bevy resource.


use std::borrow::Cow;
use std::sync::{Arc, Mutex};
use arboard::Error;
use bevy::prelude::*;

/// Plugin adding the clipboard.
pub struct CrashIDEClipboardPlugin;

impl Plugin for CrashIDEClipboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_clipboard)
        ;
    }
}

/// Wrapper around the [arboard Clipboard](arboard::Clipboard) as bevy resource
#[derive(Resource)]
pub struct Clipboard {
    inner: Arc<Mutex<arboard::Clipboard>>,
}

impl Clipboard {
    /// See [get_text](arboard::Clipboard::get_text)
    pub fn get_text(&self) -> Result<String, Error> {
        self.inner.lock().unwrap().get_text()
    }

    /// See [get_text](arboard::Clipboard::set_text)
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
