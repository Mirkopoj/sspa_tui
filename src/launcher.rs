use polling::{Event, Poller};
use ringbuf::{HeapRb, Rb};
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::mpsc::{channel, error::TryRecvError, Receiver},
};

pub struct Launcher {
    ring_buffer: HeapRb<String>,
    rx: Option<Receiver<String>>,
}

impl Launcher {
    pub fn new(buf_size: usize) -> Launcher {
        Launcher {
            ring_buffer: HeapRb::new(buf_size),
            rx: None,
        }
    }

    pub fn read(&mut self) -> String {
        let mut vec = Vec::new();
        if let Some(rx) = &mut self.rx {
            loop {
                match rx.try_recv() {
                    Ok(line) => {
                        vec.push(line);
                    }
                    Err(e) => {
                        match e {
                            TryRecvError::Empty => {}
                            _ => self.rx = None,
                        }
                        break;
                    }
                }
            }
        }
        for line in vec {
            self.ring_buffer.push_overwrite(line);
        }
        let mut ret = String::new();
        for line in self.ring_buffer.iter() {
            ret.push_str(line);
        }
        ret
    }

    pub fn launch(&mut self, command: &str) {
        let mut command: Vec<&str> = command.split(' ').rev().collect();
        if let Some(program) = command.pop() {
            let (tx, rx) = channel(128);
            self.rx = Some(rx);
            let mut output = Command::new(program);
            for arg in command.iter().rev() {
                output.arg(arg);
            }
            tokio::spawn(async move {
                let mut output = output
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("ERROR: Launcher failed to execute process");
                let stdout = output.stdout.take().unwrap();
                let stderr = output.stderr.take().unwrap();
                let mut reader_out = BufReader::new(stdout);
                let mut reader_err = BufReader::new(stderr);
                let key_out = 1;
                let key_err = 2;
                let mut out_closed = false;
                let mut err_closed = false;

                let poller = Poller::new().unwrap();
                poller
                    .add(reader_out.get_ref(), Event::readable(key_out))
                    .unwrap();
                poller
                    .add(reader_err.get_ref(), Event::readable(key_err))
                    .unwrap();

                let mut events = Vec::new();
                loop {
                    events.clear();
                    poller.wait(&mut events, None).unwrap();
                    for ev in &events {
                        if ev.key == key_out {
                            let mut str = String::new();
                            let len = match reader_out.read_line(&mut str).await {
                                Ok(len) => len,
                                Err(e) => {
                                    println!("stdout read returned error: {}", e);
                                    0
                                }
                            };
                            if len == 0 {
                                println!("stdout closed (len is null)");
                                out_closed = true;
                                poller.delete(reader_out.get_ref()).unwrap();
                            } else {
                                tx.send(str)
                                    .await
                                    .expect("ERROR: Failed to send stdout line");
                                poller
                                    .modify(reader_out.get_ref(), Event::readable(key_out))
                                    .unwrap();
                            }
                        }

                        if ev.key == key_err {
                            let mut str = String::new();
                            let len = match reader_err.read_line(&mut str).await {
                                Ok(len) => len,
                                Err(e) => {
                                    println!("stderr read returned error: {}", e);
                                    0
                                }
                            };
                            if len == 0 {
                                println!("stderr closed (len is null)");
                                err_closed = true;
                                poller.delete(reader_err.get_ref()).unwrap();
                            } else {
                                tx.send(str)
                                    .await
                                    .expect("ERROR: Failed to send stderr line");
                                poller
                                    .modify(reader_err.get_ref(), Event::readable(key_err))
                                    .unwrap();
                            }
                        }
                    }

                    if err_closed || out_closed {
                        println!("Stream closed, exiting process thread");
                        break;
                    }
                }
            });
        }
    }
}
