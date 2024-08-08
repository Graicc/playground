use std::io::Read;
use std::process::ChildStdin;
use std::str::FromStr;
use std::time::Duration;
use std::{
    io::Write,
    process::{self, Child, ChildStdout, Stdio},
    thread::JoinHandle,
};

use serde::Serialize;

pub struct LSP {
    process: Child,
    read_thread: JoinHandle<()>,
    stdin: ChildStdin,
}

impl LSP {
    pub fn new() -> Self {
        let lsp_name = "rust-analyzer";

        let mut process = process::Command::new(lsp_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start lsp");

        let mut stdin = process.stdin.take().expect("Failed to get stdin");
        let stdout = process.stdout.take().expect("Failed to get stdout");

        let read_thread = std::thread::spawn(|| handle_read_thread(stdout));

        let pid = std::process::id();
        let uri = lsp_types::Uri::from_str(&format!(
            "file:///{}",
            std::env::current_dir()
                .unwrap()
                .display()
                .to_string()
                .replace("\\", "/")
        ))
        .unwrap();
        let folder = lsp_types::WorkspaceFolder {
            uri,
            name: "Main".to_string(),
        };
        let params = lsp_types::InitializeParams {
            process_id: Some(pid),
            workspace_folders: Some(vec![folder]),
            ..Default::default()
        };

        write_content(&mut stdin, &request(0, "initialize", &params));
        write_content(
            &mut stdin,
            &notification("initialized", &lsp_types::InitializedParams {}),
        );

        Self {
            process,
            read_thread,
            stdin,
        }
    }
}

const CONTENT_LENGTH: &str = "Content-Length: ";
const CONTENT_TYPE: &str = "Content-Type: ";

fn request<T>(id: u32, method: &str, params: &T) -> String
where
    T: ?Sized + Serialize,
{
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params
    })
    .to_string()
}

fn notification<T>(method: &str, params: &T) -> String
where
    T: ?Sized + Serialize,
{
    serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params
    })
    .to_string()
}

fn write_content(output: &mut impl Write, message: &str) {
    let len = message.len();

    write!(output, "{}{}\r\n\r\n{}", CONTENT_LENGTH, len, message).unwrap();
    println!("{}{}\r\n\r\n{}", CONTENT_LENGTH, len, message);
}

fn read_single_line(input: &mut impl Read) -> String {
    let mut line: Vec<u8> = Vec::new();

    let mut buf: [u8; 1] = [0];
    loop {
        loop {
            match input.read_exact(&mut buf) {
                Ok(()) => {
                    break;
                }
                Err(x) => {
                    println!("{x:?}");
                    std::thread::sleep(Duration::from_millis(1000))
                }
            }
        }

        match buf[0] {
            b'\r' => {} //skip
            b'\n' => {
                break; // end of line
            }
            c => line.push(c),
        }
    }

    String::from_utf8(line).expect("Header must be utf8 string")
}

fn handle_read_thread(mut input: ChildStdout) {
    loop {
        read_message(&mut input);
    }
}

fn read_message(input: &mut ChildStdout) {
    let mut content_length: Option<usize> = None;
    let mut content_type: Option<String> = None;

    loop {
        let line = read_single_line(input);
        println!("read line: {line}");
        if line.is_empty() {
            break;
        }

        if line.starts_with(CONTENT_LENGTH) {
            let content_length_str = &line[CONTENT_LENGTH.len()..];
            content_length = Some(content_length_str.parse().unwrap());
        } else if line.starts_with(CONTENT_TYPE) {
            content_type = Some(line[CONTENT_TYPE.len()..].to_string());
        }
    }

    let content_length = content_length.unwrap();
    println!("Content-Length: {content_length}");
    // TODO: We assume content type is utf8
    println!("Content-Type  : {content_type:?}");

    let mut content = vec![0u8; content_length];

    input.read_exact(&mut content).unwrap();

    let content = String::from_utf8(content).unwrap();

    println!("Content: {content}");
}
