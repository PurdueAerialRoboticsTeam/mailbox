use std::net::TcpStream;
use std::io::Write;
use serde_json::json;
use std::thread;
use std::time::Duration;

mod mailbox;
use mailbox::{Mailbox, MessageType, Message};

fn send_message(msg_type: MessageType, data: &str) {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            let message = json!({
                "class": format!("{:?}", msg_type),
                "data": data
            });

            if let Err(e) = stream.write_all(message.to_string().as_bytes()) {
                eprintln!("⚠️ Failed to send message: {}", e);
            }
        }
        Err(e) => {
            eprintln!("⚠️ Failed to connect to Groundstation: {}", e);
        }
    }
}

fn main() {
    let mut mailbox = Mailbox::new();

    // 添加测试消息
    mailbox.add_message(Message {
        msg_type: MessageType::ImageMap,
        data: "Map Data".to_string(),
    });

    mailbox.add_message(Message {
        msg_type: MessageType::Error,
        data: "System Overload".to_string(),
    });

    mailbox.add_message(Message {
        msg_type: MessageType::Logs,
        data: "System initialized.".to_string(),
    });

    println!("📬 Sending messages...");

    while let Some(msg) = mailbox.get_next_message() {
        send_message(msg.msg_type, &msg.data);
        thread::sleep(Duration::from_millis(100));
    }

    println!("✅ Messages sent!");
}
