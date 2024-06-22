use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;

use bytes::Bytes;
use wasmtime_wasi::{HostInputStream, HostOutputStream, StdinStream, StdoutStream, StreamResult, Subscribe};

#[derive(Clone)]
pub struct WasmIoOutStream {
    sender: Sender<Vec<u8>>,
}

impl WasmIoOutStream {
    pub fn new(sender: Sender<Vec<u8>>) -> Self {
        Self {
            sender,
        }
    }
}

#[async_trait::async_trait]
impl Subscribe for WasmIoOutStream {
    async fn ready(&mut self) {}
}

impl HostOutputStream for WasmIoOutStream {
    fn write(&mut self, bytes: Bytes) -> StreamResult<()> {
        self.sender.send(bytes.to_vec()).unwrap();

        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}

impl StdoutStream for WasmIoOutStream {
    fn stream(&self) -> Box<dyn HostOutputStream> {
        Box::new(self.clone())
    }

    fn isatty(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct WasmIoInStream {
    pub(crate) bytes: Arc<Mutex<Vec<u8>>>,
}

impl Default for WasmIoInStream {
    fn default() -> Self {
        Self {
            bytes: Arc::new(Mutex::new(vec![]))
        }
    }
}

#[async_trait::async_trait]
impl Subscribe for WasmIoInStream {
    async fn ready(&mut self) {}
}

impl HostInputStream for WasmIoInStream {
    fn read(&mut self, size: usize) -> StreamResult<Bytes> {
        let mut buffer = self.bytes.lock().unwrap();
        let size = size.min(buffer.len());
        let read = buffer.drain(0..size).collect::<Vec<u8>>();
        Ok(Bytes::from(read))
    }
}

impl StdinStream for WasmIoInStream {
    fn stream(&self) -> Box<dyn HostInputStream> {
        Box::new(self.clone())
    }

    fn isatty(&self) -> bool {
        false
    }
}