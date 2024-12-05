
//! # Eloi - WowInteger 
//! Code to send integers to a target player using UDP.  
//! [https://github.com/EloiStree/2024_08_29_ScratchToWarcraft](https://github.com/EloiStree/2024_08_29_ScratchToWarcraft)  
//! The aim is to learn to code by playing.    
//! It is set by default to be played with World of Warcraft.  
//! May the code be with you :)  




    



use std::net::UdpSocket;
use rand::Rng;

pub struct WowIntegerTarget {
    // Player IPv4 address to target
    ip: String,
    // Player port to target
    port: u16,
    // Player index to control
    index: i32,
}

impl WowIntegerTarget {
    //! Creates a new player with the given IP, port, and index.
    pub fn new(ip: &str, port: u16, index: i32) -> WowIntegerTarget {
        WowIntegerTarget {
            ip: ip.to_string(),
            port,
            index,
        }
    }
}

pub trait IntegerUdpSender {
    /// Sends an integer to the target player using UDP.
    fn send_integer_to_target(&self, value: i32) -> std::io::Result<()>;

    /// Sends an integer to a target player at a specific index.
    fn send_integer_to_target_at_index(&self, index: i32, value: i32) -> std::io::Result<()>;

    /// Sends an integer to all players using UDP.
    fn send_integer_to_all(&self, value: i32) -> std::io::Result<()>;
}

impl IntegerUdpSender for WowIntegerTarget {
    /// Sends an integer to the target player using UDP.
    fn send_integer_to_target(&self, value: i32) -> std::io::Result<()> {
        self.send_integer_to_target_at_index(self.index, value)
    }

    /// Sends an integer to the target player at a specific index using UDP.
    fn send_integer_to_target_at_index(&self, index: i32, value: i32) -> std::io::Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        let addr = format!("{}:{}", self.ip, self.port);
        let mut buf = [0u8; 8];
        buf[..4].copy_from_slice(&index.to_le_bytes());
        buf[4..].copy_from_slice(&value.to_le_bytes());
        socket.send_to(&buf, addr)?;
        Ok(())
    }

    /// Sends an integer to all players using UDP.
    fn send_integer_to_all(&self, value: i32) -> std::io::Result<()> {
        // Assuming you want to send to multiple players, modify as needed.
        // Currently sending to a single player (use multicast or broadcast for more).
        self.send_integer_to_target(value)
    }
}

/// Returns a random float between -1.0 and 1.0.
pub fn get_random_float() -> f32 {
    (rand::random::<f32>() - 0.5) * 2.0
}

pub fn get_random_integer() -> i32 {
    rand::random::<i32>()
}

pub fn get_random_integer_between(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..max)
}

pub fn get_random_integer_from_list(list: &Vec<i32>) -> i32 {
    if list.is_empty() {
        return 0;
    } else {
        let index = rand::thread_rng().gen_range(0..list.len());
        return list[index];
    }
}

