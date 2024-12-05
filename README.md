
# Wow Integer Workshop Rust

**Warning:**  
⚠️ I love Rust, but I’m not a Rust developer and have just started learning it. ⚠️  
🛑 So, this code is most likely not great (probably rubbish). Proceed with caution! 🛑  

This code allows the use of the code convention **ScratchToWow**:  
https://github.com/EloiStree/2024_08_29_ScratchToWarcraft  

The idea is to learn coding by playing games through input injection.  
You can play any game with it, but it is designed specifically to learn by playing *World of Warcraft*.  

There are several versions of this code: **C#**, **Python**, **Unity3D**, etc.  
The goal is to keep these repositories simple, as they serve as starting points for learning in workshops.  

The main idea is that keyboards and gamepads are represented as integers.

There are different targets:  
- **The main default one:**  
  - Uses PostMessage to inject fake keystrokes into a specific application.
- **The less intrusive version:**  
  - Uses CType Keyboard to simulate keys natively on Windows.
- **The hardware solutions:**  
  - **Pico W**: Simulates keyboard, mouse, and gamepad via USB.  
  - **ESP32**: Simulates keyboard, mouse, and gamepad via Bluetooth.  
  - **XESP32**: Simulates XInput via Bluetooth.  
  - **XInput Arduino**: Simulates Xbox Input via USB.  



Copy: `be_eloistree_wowinteger = "2024.12.5"`  
As Git: `be_eloistree_wowinteger = {git="https://github.com/EloiStree/2024_12_05_WowIntegerWorkshopRust.git"}`  