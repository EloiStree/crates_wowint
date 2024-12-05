// Input : https://github.com/EloiStree/2024_08_29_ScratchToWarcraft

use std::io;
use std::thread;
use std::time::Duration;
use rand::seq::SliceRandom;
use wowint::utility::{EnumWowKey, WowWindowKeyInt};
use wowint::utility::{
    WowIntegerTarget,
    IntegerUdpSender,
    get_random_integer_between,
    get_random_integer_from_list
};



fn main() -> io::Result<()> {
    // Create a new WowIntegerPlayer instance with a dummy IP, port, and index
    let player = WowIntegerTarget::new("192.168.1.37", 7073, 2);

    // Send a fixed integer to the target player (using default index)
    println!("Sending a fixed integer to target player...");
    player.send_integer_to_target(42)?;

    let exit_condition: bool =false;

    // Create a list of integers
    let rotateLeft: i32 = (WowWindowKeyInt::Numpad4 as i32) + 1000;
    let rotateRight: i32 = (WowWindowKeyInt::Numpad6 as i32) + 1000;
    let moveFroward: i32 = (WowWindowKeyInt::Numpad8 as i32) + 1000;
    let moveBackwardd: i32 = (WowWindowKeyInt::Numpad2 as i32) + 1000;
    let enter: i32 =(WowWindowKeyInt::Enter as i32) + 1000;
    let moveUp: i32 = (WowWindowKeyInt::Numpad2 as i32) + 1000;
    let moveDown: i32 = (WowWindowKeyInt::Numpad0 as i32) + 1000;
    let moveLeft: i32 = (WowWindowKeyInt::Numpad1 as i32) + 1000;
    let moveRight: i32 = (WowWindowKeyInt::Numpad3 as i32) + 1000;
    let arrow_list_key: Vec<i32> = vec![ 
        rotateLeft,
        rotateRight,
        moveFroward,
        moveBackwardd,
        enter,
        moveUp,
        moveDown,
        moveLeft,
        moveRight
    ];




    while !exit_condition {
        let range_int_press = get_random_integer_between(1048, 1090);
        let range_int_release   = range_int_press+1000;

        println!("Sending integer: {}", range_int_press);
        player.send_integer_to_target(range_int_press)?;
        thread::sleep(Duration::from_secs(1)); 
        println!("Sending integer: {}", range_int_release);
        player.send_integer_to_target(range_int_release)?;
        thread::sleep(Duration::from_secs(1)); 

        let random_arrow_input_press = get_random_integer_from_list(&arrow_list_key);
        let random_arrow_input_release = random_arrow_input_press+1000;
        println!("Sending integer arrow: {}", random_arrow_input_press);
        player.send_integer_to_target(random_arrow_input_press)?;
        thread::sleep(Duration::from_secs(1));
        println!("Sending integer arrow: {}", random_arrow_input_release);
        player.send_integer_to_target(random_arrow_input_release)?;
        thread::sleep(Duration::from_secs(1));

    }

    Ok(())
}
