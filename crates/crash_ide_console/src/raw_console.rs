use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct RawConsole {
    #[allow(unused)]
    cmd: Child,
    stdout_read: Arc<Mutex<Receiver<String>>>,
    stdin: ChildStdin,
    #[allow(unused)]
    stdout_task: JoinHandle<()>,
    #[allow(unused)]
    stderr_task: JoinHandle<()>,
}

impl RawConsole {
    pub fn new(start_dir: impl AsRef<Path>) -> Result<Self, String> {
        #[cfg(target_os = "linux")]
            let mut cmd = Command::new("bash");
        #[cfg(target_os = "windows")]
            let mut cmd = Command::new("cmd");
        let mut cmd = cmd
            .current_dir(start_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().map_err(|e| e.to_string())?;

        let mut stdout = BufReader::new(cmd.stdout.take().unwrap());
        let mut stderr = BufReader::new(cmd.stderr.take().unwrap());
        let mut stdin = cmd.stdin.take().unwrap();

        let (stdout_write, stdout_read) = channel();

        let cloned_write = stdout_write.clone();

        let stdout_task = thread::spawn(move || {
            let mut buf = String::new();
            while let Ok(_) = stdout.read_line(&mut buf) {
                if cloned_write.send(buf.clone()).is_err() {
                    break;
                }

                buf.clear();
            }
        });

        let stderr_task = thread::spawn(move || {
            let mut buf = String::new();
            while let Ok(_) = stderr.read_line(&mut buf) {
                if stdout_write.send(buf.clone()).is_err() {
                    break;
                }

                buf.clear();
            }
        });

        stdin.write_all(b"echo $PWD$\n").unwrap();

        Ok(Self {
            cmd,
            stdout_read: Arc::new(Mutex::new(stdout_read)),
            stdin,
            stdout_task,
            stderr_task,
        })
    }

    pub fn try_read(&mut self) -> Result<String, TryRecvError> {
        self.stdout_read.lock().unwrap().try_recv()
    }

    pub fn execute_command(&mut self, command: String) {
        self.stdin.write_all(command.as_bytes()).unwrap();
    }
}