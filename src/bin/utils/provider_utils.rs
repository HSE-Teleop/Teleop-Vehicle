use std::cmp::PartialEq;
use std::collections::VecDeque;

pub struct MessageCache {
    pub message: VecDeque<Message>,
}
#[derive(Debug)]
pub struct Message {
    pub msg: String,
    pub signal: String,
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.msg == other.msg && self.signal == other.signal
    }
}

impl Message {
    pub fn new(msg: String, signal: String) -> Message {
        Message { msg, signal }
    }
}

/// TODO: Maybe work with timestamps to avoid overflow of data.
///     If something goes wrong and the first entry doesnt met his double the cache wont empty.
///     Do a check with the timestamp to delete expired messages...

impl MessageCache {
    pub fn push_message(&mut self, msg: String, signal: String) {
        // println!("\nPushing messages into the cache: {}\n", msg);
        let new_message: Message = Message {msg, signal};           // Shorthand initializer instead of: {msg: msg, signal: signal}
        self.message.push_back(new_message);
    }
    fn pop_message(&mut self) -> Option<Message> {
        self.message.pop_front()
    }
    /// When called from the kuksa thread expects to be a new message (not the current income zenoh message) <br/>
    /// Therefore, <br/>
    ///     returns true when it's considered a new message and <br/>
    ///     returns false when it's considered a double. <br/>
    ///         => When considered a double, the double gets polled from the cache
    pub fn expect_outgoing_message(&mut self, expected: Message) -> (bool, Option<Message>) {
        if self.message.is_empty() {
            return (true, None);
        }
        println!("\nMessages in cache: {:?}\n", self.message);
        if self.message[0] == expected {
            (false, self.pop_message())
        } else {
            (true, None)
        }
    }
}