use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::thread;
use std::thread::JoinHandle;

use bevy::prelude::*;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{preview1, WasiCtxBuilder};
use wasmtime_wasi::preview1::WasiP1Ctx;

use crash_ide_plugin_types::PluginInfo;

use crate::stream::{WasmIoInStream, WasmIoOutStream};

#[derive(Component)]
pub struct PluginInstance {
    pub path: PathBuf,
    #[allow(unused)]
    handle: JoinHandle<Result<(), String>>,
    stdout: Arc<Mutex<Receiver<Vec<u8>>>>,
    stderr: Arc<Mutex<Receiver<Vec<u8>>>>,
    stdin: WasmIoInStream,
}

impl PluginInstance {
    pub fn new(engine: &Engine, path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let module = Module::from_file(engine, &path)?;

        let mut linker: Linker<WasiP1Ctx> = Linker::new(engine);
        preview1::add_to_linker_sync(&mut linker, |t| t)?;
        let pre = linker.instantiate_pre(&module)?;

        let (stdout_tx, stdout_rx) = channel();
        let (stderr_tx, stderr_rx) = channel();
        let stdout = WasmIoOutStream::new(stdout_tx);
        let stderr = WasmIoOutStream::new(stderr_tx);
        let stdin = WasmIoInStream::default();
        let wasi_ctx = WasiCtxBuilder::new()
            .stdout(stdout)
            .stderr(stderr)
            .stdin(stdin.clone())
            .build_p1();

        let mut store = Store::new(engine, wasi_ctx);
        let instance = pre.instantiate(&mut store)?;

        let start_plugin = instance.get_typed_func::<(), ()>(&mut store, "_plugin_main")?;

        let handle = thread::spawn(move || {
            start_plugin.call(&mut store, ()).map_err(|e| e.to_string())
        });

        Ok(Self {
            path,
            handle,
            stdout: Arc::new(Mutex::new(stdout_rx)),
            stderr: Arc::new(Mutex::new(stderr_rx)),
            stdin,
        })
    }

    pub(crate) fn try_read(&self) -> Result<Vec<u8>, TryRecvError> {
        self.stdout.lock().unwrap().try_recv()
    }

    pub(crate) fn try_read_error(&self) -> Result<Vec<u8>, TryRecvError> {
        self.stderr.lock().unwrap().try_recv()
    }

    pub fn send(&self, bytes: Vec<u8>) {
        self.stdin.bytes.lock().unwrap().extend(bytes);
    }
}

#[derive(Component)]
pub struct LoadedPluginInfo(pub PluginInfo);