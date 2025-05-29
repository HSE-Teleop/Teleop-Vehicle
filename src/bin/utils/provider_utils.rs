use std::collections::VecDeque;

pub struct MessageCache {
    pub msg: VecDeque<String>,
}

/// TODO: Maybe work with timestamps to avoid overflow of data.
///     If something goes wrong and the first entry doesnt met his double the cache wont empty.
///     Do a check with the timestamp to delete expired messages...

impl MessageCache {
    pub fn push_message(&mut self, msg: String) {
        // println!("\nPushing messages into the cache: {}\n", msg);
        self.msg.push_back(msg);
    }
    fn pop_message(&mut self) -> Option<String> {
        self.msg.pop_front()
    }
    /// When called from the kuksa thread expects to be a new message (not the current income zenoh message) <br/>
    /// Therefore, <br/>
    ///     returns true when it's considered a new message and <br/>
    ///     returns false when it's considered a double. <br/>
    ///         => When considered a double, the double gets polled from the cache
    pub fn expect_outgoing_message(&mut self, expected: String) -> (bool, Option<String>) {
        if self.msg.is_empty() {
            return (true, None);
        }
        println!("\nMessages in cache: {:?}\n", self.msg);
        if self.msg[0] == expected {
            (false, self.pop_message())
        } else {
            (true, None)
        }
    }
}