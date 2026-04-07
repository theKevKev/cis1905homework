use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::mpmc::{Receiver, Sender, TryRecvError, channel};

pub struct HttpServer {
    stop_tx: Sender<()>,
    listener_thread: Option<JoinHandle<()>>,
    worker_threads: Vec<JoinHandle<()>>,
}

fn run_listener(port: u16, stop_rx: Receiver<()>, connection_tx: Sender<TcpStream>) {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    listener.set_nonblocking(true).unwrap();

    loop {
        match stop_rx.try_recv() {
            Ok(()) => break,
            Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {}
        };
        match listener.accept() {
            Ok((stream, _)) => {
                stream.set_nonblocking(false).unwrap();
                connection_tx.send(stream).unwrap();
            }
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(10))
            }
            Err(err) => println!("error accepting connection: {err}"),
        };
    }
}

fn run_worker(connection_rx: Receiver<TcpStream>) {
    loop {
        match connection_rx.recv() {
            None => return,
            Some(stream) => {
                if let Err(e) = handle_conn(stream) {
                    println!("connection error: {e:?}")
                }
            }
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum HttpError {
    Io(std::io::Error),
}

impl From<std::io::Error> for HttpError {
    fn from(e: std::io::Error) -> Self {
        HttpError::Io(e)
    }
}

fn handle_conn(stream: TcpStream) -> Result<(), HttpError> {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    // Get URL from first line
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    let url = first_line.split_whitespace().nth(1).unwrap_or("/");

    // flush out remaining text
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 || line.trim().is_empty() {
            break;
        }
    }

    let path = Path::new(".").join(&url[1..]);

    if path.is_file() {
        let contents = std::fs::read_to_string(&path)?;
        write!(writer, "HTTP/1.1 200 OK\r\n")?;
        write!(writer, "Content-Type: text/plain\r\n")?;
        write!(writer, "Content-Length: {}\r\n\r\n", contents.len())?;
        write!(writer, "{contents}")?;
    } else {
        let body = "File not found";
        write!(writer, "HTTP/1.1 404 Not Found\r\n")?;
        write!(writer, "Content-Type: text/plain\r\n")?;
        write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;
        write!(writer, "{body}")?;
    }

    writer.flush()?;
    Ok(())
}

impl HttpServer {
    #[must_use]
    pub fn new(port: u16, num_workers: u32) -> Self {
        let (stop_tx, stop_rx) = channel::<()>();
        let (connection_tx, connection_rx) = channel::<TcpStream>();

        let listener_thread = thread::spawn(move || run_listener(port, stop_rx, connection_tx));
        let mut worker_threads = Vec::new();

        for _ in 0..num_workers {
            let cloned_connection_rx = connection_rx.clone();
            worker_threads.push(thread::spawn(move || run_worker(cloned_connection_rx)));
        }

        HttpServer {
            stop_tx,
            listener_thread: Some(listener_thread),
            worker_threads,
        }
    }
}

impl Drop for HttpServer {
    fn drop(&mut self) {
        self.stop_tx.send(()).ok();
        // clean up all threads
        if let Some(listener_handle) = self.listener_thread.take() {
            listener_handle.join().ok();
        }
        for join_handle in self.worker_threads.drain(..) {
            join_handle.join().ok();
        }
    }
}
