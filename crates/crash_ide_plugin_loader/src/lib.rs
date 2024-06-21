use std::error::Error;
use std::path::PathBuf;

use bevy::prelude::*;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{preview1, WasiCtxBuilder};
use wasmtime_wasi::preview1::WasiP1Ctx;

use crash_ide_config::HomeDir;

pub struct CrashIDEPluginLoaderPlugin;

impl Plugin for CrashIDEPluginLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlobalWasmEngine>()
            .add_systems(Startup, load_plugins)
        ;
    }
}

#[derive(Resource, Default)]
pub(crate) struct GlobalWasmEngine {
    pub engine: Engine,
}

fn load_plugins(
    wasm_engine: Res<GlobalWasmEngine>,
    home_dir: Res<HomeDir>,
) {
    for entry in home_dir.plugin_path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension.to_str().unwrap() != "wasm" {
                    continue;
                }

                println!("Load plugin {}", path.to_str().unwrap());
                match create_wasm_instance(&wasm_engine.engine, path) {
                    Ok(instance) => instance,
                    Err(e) => {
                        println!("Error loading plugin: {}", e);
                        continue;
                    }
                }
            }
        }
    }
}

fn create_wasm_instance(engine: &Engine, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let module = Module::from_file(engine, path)?;

    let mut linker: Linker<WasiP1Ctx> = Linker::new(engine);
    preview1::add_to_linker_sync(&mut linker, |t| t)?;
    let pre = linker.instantiate_pre(&module)?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .build_p1();

    let mut store = Store::new(engine, wasi_ctx);
    let instance = pre.instantiate(&mut store)?;
    let start_plugin = instance.get_typed_func::<(), ()>(&mut store, "_start_plugin")?;

    start_plugin.call(&mut store, ())?;

    Ok(())
}