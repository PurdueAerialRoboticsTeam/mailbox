use std::collections::BinaryHeap;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

// 定义消息类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    ImageMap,
    TargetFound,
    Error,
    Logs,
}

// 获取优先级
impl MessageType {
    fn get_priority(&self) -> u8 {
        match self {
            MessageType::ImageMap => 1,
            MessageType::TargetFound => 2,
            MessageType::Error => 2,
            MessageType::Logs => 3,
        }
    }
}

// 定义消息结构
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]  // ✅ 修复缺少 `Eq` 和 `PartialEq`
pub struct Message {
    pub msg_type: MessageType,
    pub data: String,
}

// 定义队列元素
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QueueItem {
    priority: u8,
    message: Message,
}

// 让 Rust 知道如何排序优先级
impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// 实现 Mailbox 结构
pub struct Mailbox {
    queue: BinaryHeap<QueueItem>,
}

impl Mailbox {
    pub fn new() -> Self {
        Mailbox {
            queue: BinaryHeap::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        let priority = message.msg_type.get_priority();
        self.queue.push(QueueItem { priority, message });
    }

    pub fn get_next_message(&mut self) -> Option<Message> {
        self.queue.pop().map(|item| item.message)
    }
}
