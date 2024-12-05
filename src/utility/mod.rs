
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




pub struct WowKeyInfo {
    keyName: String,
    windowDecimal: u8,
    windowHexadecimal: u16,
    pressInteger: u16,
    releaseInteger: u16,
}


pub struct EnumWowKey {
    list: Vec<WowKeyInfo>,
}

impl EnumWowKey {

    pub fn get_key_info(&self, key: &str) -> Option<&WowKeyInfo> {
        self.list.iter().find(|&x| x.keyName == key)
    }

    pub fn get_key_info_by_decimal(&self, decimal: u8) -> Option<&WowKeyInfo> {
        self.list.iter().find(|&x| x.windowDecimal == decimal)
    }

    pub fn get_key_info_by_hexadecimal(&self, hexadecimal: u16) -> Option<&WowKeyInfo> {
        self.list.iter().find(|&x| x.windowHexadecimal == hexadecimal)
    }

    pub fn get_key_info_by_press(&self, press: u16) -> Option<&WowKeyInfo> {
        self.list.iter().find(|&x| x.pressInteger == press)
    }

    pub fn get_key_info_by_release(&self, release: u16) -> Option<&WowKeyInfo> {
        self.list.iter().find(|&x| x.releaseInteger == release)
    }



    fn new() -> Self {
        EnumWowKey {
            list: vec![
                WowKeyInfo { keyName: "Backspace".to_string(), windowDecimal: 8, windowHexadecimal: 0x08, pressInteger: 1008, releaseInteger: 2008 },
                WowKeyInfo { keyName: "Tab".to_string(), windowDecimal: 9, windowHexadecimal: 0x09, pressInteger: 1009, releaseInteger: 2009 },
                WowKeyInfo { keyName: "Clear".to_string(), windowDecimal: 12, windowHexadecimal: 0x0C, pressInteger: 1012, releaseInteger: 2012 },
                WowKeyInfo { keyName: "Enter".to_string(), windowDecimal: 13, windowHexadecimal: 0x0D, pressInteger: 1013, releaseInteger: 2013 },
                WowKeyInfo { keyName: "Shift".to_string(), windowDecimal: 16, windowHexadecimal: 0x10, pressInteger: 1016, releaseInteger: 2016 },
                WowKeyInfo { keyName: "Ctrl".to_string(), windowDecimal: 17, windowHexadecimal: 0x11, pressInteger: 1017, releaseInteger: 2017 },
                WowKeyInfo { keyName: "Alt".to_string(), windowDecimal: 18, windowHexadecimal: 0x12, pressInteger: 1018, releaseInteger: 2018 },
                WowKeyInfo { keyName: "Pause".to_string(), windowDecimal: 19, windowHexadecimal: 0x13, pressInteger: 1019, releaseInteger: 2019 },
                WowKeyInfo { keyName: "CapsLock".to_string(), windowDecimal: 20, windowHexadecimal: 0x14, pressInteger: 1020, releaseInteger: 2020 },
                WowKeyInfo { keyName: "Esc".to_string(), windowDecimal: 27, windowHexadecimal: 0x1B, pressInteger: 1027, releaseInteger: 2027 },
                WowKeyInfo { keyName: "Escape".to_string(), windowDecimal: 27, windowHexadecimal: 0x1B, pressInteger: 1027, releaseInteger: 2027 },
                WowKeyInfo { keyName: "Space".to_string(), windowDecimal: 32, windowHexadecimal: 0x20, pressInteger: 1032, releaseInteger: 2032 },
                WowKeyInfo { keyName: "PageUp".to_string(), windowDecimal: 33, windowHexadecimal: 0x21, pressInteger: 1033, releaseInteger: 2033 },
                WowKeyInfo { keyName: "PageDown".to_string(), windowDecimal: 34, windowHexadecimal: 0x22, pressInteger: 1034, releaseInteger: 2034 },
                WowKeyInfo { keyName: "End".to_string(), windowDecimal: 35, windowHexadecimal: 0x23, pressInteger: 1035, releaseInteger: 2035 },
                WowKeyInfo { keyName: "Home".to_string(), windowDecimal: 36, windowHexadecimal: 0x24, pressInteger: 1036, releaseInteger: 2036 },
                WowKeyInfo { keyName: "LeftArrow".to_string(), windowDecimal: 37, windowHexadecimal: 0x25, pressInteger: 1037, releaseInteger: 2037 },
                WowKeyInfo { keyName: "Left".to_string(), windowDecimal: 37, windowHexadecimal: 0x25, pressInteger: 1037, releaseInteger: 2037 },
                WowKeyInfo { keyName: "UpArrow".to_string(), windowDecimal: 38, windowHexadecimal: 0x26, pressInteger: 1038, releaseInteger: 2038 },
                WowKeyInfo { keyName: "Up".to_string(), windowDecimal: 38, windowHexadecimal: 0x26, pressInteger: 1038, releaseInteger: 2038 },
                WowKeyInfo { keyName: "RightArrow".to_string(), windowDecimal: 39, windowHexadecimal: 0x27, pressInteger: 1039, releaseInteger: 2039 },
                WowKeyInfo { keyName: "Right".to_string(), windowDecimal: 39, windowHexadecimal: 0x27, pressInteger: 1039, releaseInteger: 2039 },
                WowKeyInfo { keyName: "DownArrow".to_string(), windowDecimal: 40, windowHexadecimal: 0x28, pressInteger: 1040, releaseInteger: 2040 },
                WowKeyInfo { keyName: "Down".to_string(), windowDecimal: 40, windowHexadecimal: 0x28, pressInteger: 1040, releaseInteger: 2040 },
                WowKeyInfo { keyName: "Select".to_string(), windowDecimal: 41, windowHexadecimal: 0x29, pressInteger: 1041, releaseInteger: 2041 },
                WowKeyInfo { keyName: "Print".to_string(), windowDecimal: 42, windowHexadecimal: 0x2A, pressInteger: 1042, releaseInteger: 2042 },
                WowKeyInfo { keyName: "Execute".to_string(), windowDecimal: 43, windowHexadecimal: 0x2B, pressInteger: 1043, releaseInteger: 2043 },
                WowKeyInfo { keyName: "PrintScreen".to_string(), windowDecimal: 44, windowHexadecimal: 0x2C, pressInteger: 1044, releaseInteger: 2044 },
                WowKeyInfo { keyName: "Insert".to_string(), windowDecimal: 45, windowHexadecimal: 0x2D, pressInteger: 1045, releaseInteger: 2045 },
                WowKeyInfo { keyName: "Delete".to_string(), windowDecimal: 46, windowHexadecimal: 0x2E, pressInteger: 1046, releaseInteger: 2046 },
                WowKeyInfo { keyName: "0".to_string(), windowDecimal: 48, windowHexadecimal: 0x30, pressInteger: 1048, releaseInteger: 2048 },
                WowKeyInfo { keyName: "1".to_string(), windowDecimal: 49, windowHexadecimal: 0x31, pressInteger: 1049, releaseInteger: 2049 },
                WowKeyInfo { keyName: "2".to_string(), windowDecimal: 50, windowHexadecimal: 0x32, pressInteger: 1050, releaseInteger: 2050 },
                WowKeyInfo { keyName: "3".to_string(), windowDecimal: 51, windowHexadecimal: 0x33, pressInteger: 1051, releaseInteger: 2051 },
                WowKeyInfo { keyName: "4".to_string(), windowDecimal: 52, windowHexadecimal: 0x34, pressInteger: 1052, releaseInteger: 2052 },
                WowKeyInfo { keyName: "5".to_string(), windowDecimal: 53, windowHexadecimal: 0x35, pressInteger: 1053, releaseInteger: 2053 },
                WowKeyInfo { keyName: "6".to_string(), windowDecimal: 54, windowHexadecimal: 0x36, pressInteger: 1054, releaseInteger: 2054 },
                WowKeyInfo { keyName: "7".to_string(), windowDecimal: 55, windowHexadecimal: 0x37, pressInteger: 1055, releaseInteger: 2055 },
                WowKeyInfo { keyName: "8".to_string(), windowDecimal: 56, windowHexadecimal: 0x38, pressInteger: 1056, releaseInteger: 2056 },
                WowKeyInfo { keyName: "9".to_string(), windowDecimal: 57, windowHexadecimal: 0x39, pressInteger: 1057, releaseInteger: 2057 },
                WowKeyInfo { keyName: "A".to_string(), windowDecimal: 65, windowHexadecimal: 0x41, pressInteger: 1065, releaseInteger: 2065 },
                WowKeyInfo { keyName: "B".to_string(), windowDecimal: 66, windowHexadecimal: 0x42, pressInteger: 1066, releaseInteger: 2066 },
                WowKeyInfo { keyName: "C".to_string(), windowDecimal: 67, windowHexadecimal: 0x43, pressInteger: 1067, releaseInteger: 2067 },
                WowKeyInfo { keyName: "D".to_string(), windowDecimal: 68, windowHexadecimal: 0x44, pressInteger: 1068, releaseInteger: 2068 },
                WowKeyInfo { keyName: "E".to_string(), windowDecimal: 69, windowHexadecimal: 0x45, pressInteger: 1069, releaseInteger: 2069 },
                WowKeyInfo { keyName: "F".to_string(), windowDecimal: 70, windowHexadecimal: 0x46, pressInteger: 1070, releaseInteger: 2070 },
                WowKeyInfo { keyName: "G".to_string(), windowDecimal: 71, windowHexadecimal: 0x47, pressInteger: 1071, releaseInteger: 2071 },
                WowKeyInfo { keyName: "H".to_string(), windowDecimal: 72, windowHexadecimal: 0x48, pressInteger: 1072, releaseInteger: 2072 },
                WowKeyInfo { keyName: "I".to_string(), windowDecimal: 73, windowHexadecimal: 0x49, pressInteger: 1073, releaseInteger: 2073 },
                WowKeyInfo { keyName: "J".to_string(), windowDecimal: 74, windowHexadecimal: 0x4A, pressInteger: 1074, releaseInteger: 2074 },
                WowKeyInfo { keyName: "K".to_string(), windowDecimal: 75, windowHexadecimal: 0x4B, pressInteger: 1075, releaseInteger: 2075 },
                WowKeyInfo { keyName: "L".to_string(), windowDecimal: 76, windowHexadecimal: 0x4C, pressInteger: 1076, releaseInteger: 2076 },
                WowKeyInfo { keyName: "M".to_string(), windowDecimal: 77, windowHexadecimal: 0x4D, pressInteger: 1077, releaseInteger: 2077 },
                WowKeyInfo { keyName: "N".to_string(), windowDecimal: 78, windowHexadecimal: 0x4E, pressInteger: 1078, releaseInteger: 2078 },
                WowKeyInfo { keyName: "O".to_string(), windowDecimal: 79, windowHexadecimal: 0x4F, pressInteger: 1079, releaseInteger: 2079 },
                WowKeyInfo { keyName: "P".to_string(), windowDecimal: 80, windowHexadecimal: 0x50, pressInteger: 1080, releaseInteger: 2080 },
                WowKeyInfo { keyName: "Q".to_string(), windowDecimal: 81, windowHexadecimal: 0x51, pressInteger: 1081, releaseInteger: 2081 },
                WowKeyInfo { keyName: "R".to_string(), windowDecimal: 82, windowHexadecimal: 0x52, pressInteger: 1082, releaseInteger: 2082 },
                WowKeyInfo { keyName: "S".to_string(), windowDecimal: 83, windowHexadecimal: 0x53, pressInteger: 1083, releaseInteger: 2083 },
                WowKeyInfo { keyName: "T".to_string(), windowDecimal: 84, windowHexadecimal: 0x54, pressInteger: 1084, releaseInteger: 2084 },
                WowKeyInfo { keyName: "U".to_string(), windowDecimal: 85, windowHexadecimal: 0x55, pressInteger: 1085, releaseInteger: 2085 },
                WowKeyInfo { keyName: "V".to_string(), windowDecimal: 86, windowHexadecimal: 0x56, pressInteger: 1086, releaseInteger: 2086 },
                WowKeyInfo { keyName: "W".to_string(), windowDecimal: 87, windowHexadecimal: 0x57, pressInteger: 1087, releaseInteger: 2087 },
                WowKeyInfo { keyName: "X".to_string(), windowDecimal: 88, windowHexadecimal: 0x58, pressInteger: 1088, releaseInteger: 2088 },
                WowKeyInfo { keyName: "Y".to_string(), windowDecimal: 89, windowHexadecimal: 0x59, pressInteger: 1089, releaseInteger: 2089 },
                WowKeyInfo { keyName: "Z".to_string(), windowDecimal: 90, windowHexadecimal: 0x5A, pressInteger: 1090, releaseInteger: 2090 },
                WowKeyInfo { keyName: "LeftWindow".to_string(), windowDecimal: 91, windowHexadecimal: 0x5B, pressInteger: 1091, releaseInteger: 2091 },
                WowKeyInfo { keyName: "RightWindow".to_string(), windowDecimal: 92, windowHexadecimal: 0x5C, pressInteger: 1092, releaseInteger: 2092 },
                WowKeyInfo { keyName: "Applications".to_string(), windowDecimal: 93, windowHexadecimal: 0x5D, pressInteger: 1093, releaseInteger: 2093 },
                WowKeyInfo { keyName: "Sleep".to_string(), windowDecimal: 95, windowHexadecimal: 0x5F, pressInteger: 1095, releaseInteger: 2095 },
                WowKeyInfo { keyName: "NumLock".to_string(), windowDecimal: 144, windowHexadecimal: 0x90, pressInteger: 1144, releaseInteger: 2144 },
                WowKeyInfo { keyName: "ScrollLock".to_string(), windowDecimal: 145, windowHexadecimal: 0x91, pressInteger: 1145, releaseInteger: 2145 },
            
        
        
                WowKeyInfo { keyName: "Numpad0".to_string(), windowDecimal: 96, windowHexadecimal: 0x60, pressInteger: 1096, releaseInteger: 2096 },
                WowKeyInfo { keyName: "Numpad1".to_string(), windowDecimal: 97, windowHexadecimal: 0x61, pressInteger: 1097, releaseInteger: 2097 },
                WowKeyInfo { keyName: "Numpad2".to_string(), windowDecimal: 98, windowHexadecimal: 0x62, pressInteger: 1098, releaseInteger: 2098 },
                WowKeyInfo { keyName: "Numpad3".to_string(), windowDecimal: 99, windowHexadecimal: 0x63, pressInteger: 1099, releaseInteger: 2099 },
                WowKeyInfo { keyName: "Numpad4".to_string(), windowDecimal: 100, windowHexadecimal: 0x64, pressInteger: 1100, releaseInteger: 2100 },
                WowKeyInfo { keyName: "Numpad5".to_string(), windowDecimal: 101, windowHexadecimal: 0x65, pressInteger: 1101, releaseInteger: 2101 },
                WowKeyInfo { keyName: "Numpad6".to_string(), windowDecimal: 102, windowHexadecimal: 0x66, pressInteger: 1102, releaseInteger: 2102 },
                WowKeyInfo { keyName: "Numpad7".to_string(), windowDecimal: 103, windowHexadecimal: 0x67, pressInteger: 1103, releaseInteger: 2103 },
                WowKeyInfo { keyName: "Numpad8".to_string(), windowDecimal: 104, windowHexadecimal: 0x68, pressInteger: 1104, releaseInteger: 2104 },
                WowKeyInfo { keyName: "Numpad9".to_string(), windowDecimal: 105, windowHexadecimal: 0x69, pressInteger: 1105, releaseInteger: 2105 },
                WowKeyInfo { keyName: "Multiply".to_string(), windowDecimal: 106, windowHexadecimal: 0x6A, pressInteger: 1106, releaseInteger: 2106 },
                WowKeyInfo { keyName: "Add".to_string(), windowDecimal: 107, windowHexadecimal: 0x6B, pressInteger: 1107, releaseInteger: 2107 },
                WowKeyInfo { keyName: "Separator".to_string(), windowDecimal: 108, windowHexadecimal: 0x6C, pressInteger: 1108, releaseInteger: 2108 },
                WowKeyInfo { keyName: "Subtract".to_string(), windowDecimal: 109, windowHexadecimal: 0x6D, pressInteger: 1109, releaseInteger: 2109 },
                WowKeyInfo { keyName: "Decimal".to_string(), windowDecimal: 110, windowHexadecimal: 0x6E, pressInteger: 1110, releaseInteger: 2110 },
                WowKeyInfo { keyName: "Divide".to_string(), windowDecimal: 111, windowHexadecimal: 0x6F, pressInteger: 1111, releaseInteger: 2111 },
                WowKeyInfo { keyName: "F1".to_string(), windowDecimal: 112, windowHexadecimal: 0x70, pressInteger: 1112, releaseInteger: 2112 },
                WowKeyInfo { keyName: "F2".to_string(), windowDecimal: 113, windowHexadecimal: 0x71, pressInteger: 1113, releaseInteger: 2113 },
                WowKeyInfo { keyName: "F3".to_string(), windowDecimal: 114, windowHexadecimal: 0x72, pressInteger: 1114, releaseInteger: 2114 },
                WowKeyInfo { keyName: "F4".to_string(), windowDecimal: 115, windowHexadecimal: 0x73, pressInteger: 1115, releaseInteger: 2115 },
                WowKeyInfo { keyName: "F5".to_string(), windowDecimal: 116, windowHexadecimal: 0x74, pressInteger: 1116, releaseInteger: 2116 },
                WowKeyInfo { keyName: "F6".to_string(), windowDecimal: 117, windowHexadecimal: 0x75, pressInteger: 1117, releaseInteger: 2117 },
                WowKeyInfo { keyName: "F7".to_string(), windowDecimal: 118, windowHexadecimal: 0x76, pressInteger: 1118, releaseInteger: 2118 },
                WowKeyInfo { keyName: "F8".to_string(), windowDecimal: 119, windowHexadecimal: 0x77, pressInteger: 1119, releaseInteger: 2119 },
                WowKeyInfo { keyName: "F9".to_string(), windowDecimal: 120, windowHexadecimal: 0x78, pressInteger: 1120, releaseInteger: 2120 },
                WowKeyInfo { keyName: "F10".to_string(), windowDecimal: 121, windowHexadecimal: 0x79, pressInteger: 1121, releaseInteger: 2121 },
                WowKeyInfo { keyName: "F11".to_string(), windowDecimal: 122, windowHexadecimal: 0x7A, pressInteger: 1122, releaseInteger: 2122 },
                WowKeyInfo { keyName: "F12".to_string(), windowDecimal: 123, windowHexadecimal: 0x7B, pressInteger: 1123, releaseInteger: 2123 },
                WowKeyInfo { keyName: "F13".to_string(), windowDecimal: 124, windowHexadecimal: 0x7C, pressInteger: 1124, releaseInteger: 2124 },
                WowKeyInfo { keyName: "F14".to_string(), windowDecimal: 125, windowHexadecimal: 0x7D, pressInteger: 1125, releaseInteger: 2125 },
                WowKeyInfo { keyName: "F15".to_string(), windowDecimal: 126, windowHexadecimal: 0x7E, pressInteger: 1126, releaseInteger: 2126 },
                WowKeyInfo { keyName: "F16".to_string(), windowDecimal: 127, windowHexadecimal: 0x7F, pressInteger: 1127, releaseInteger: 2127 },
                WowKeyInfo { keyName: "F17".to_string(), windowDecimal: 128, windowHexadecimal: 0x80, pressInteger: 1128, releaseInteger: 2128 },
                WowKeyInfo { keyName: "F18".to_string(), windowDecimal: 129, windowHexadecimal: 0x81, pressInteger: 1129, releaseInteger: 2129 },
                WowKeyInfo { keyName: "F19".to_string(), windowDecimal: 130, windowHexadecimal: 0x82, pressInteger: 1130, releaseInteger: 2130 },
                WowKeyInfo { keyName: "F20".to_string(), windowDecimal: 131, windowHexadecimal: 0x83, pressInteger: 1131, releaseInteger: 2131 },
                WowKeyInfo { keyName: "F21".to_string(), windowDecimal: 132, windowHexadecimal: 0x84, pressInteger: 1132, releaseInteger: 2132 },
                WowKeyInfo { keyName: "F22".to_string(), windowDecimal: 133, windowHexadecimal: 0x85, pressInteger: 1133, releaseInteger: 2133 },
                WowKeyInfo { keyName: "F23".to_string(), windowDecimal: 134, windowHexadecimal: 0x86, pressInteger: 1134, releaseInteger: 2134 },
                WowKeyInfo { keyName: "F24".to_string(), windowDecimal: 135, windowHexadecimal: 0x87, pressInteger: 1135, releaseInteger: 2135 },
                WowKeyInfo { keyName: "NumLock".to_string(), windowDecimal: 144, windowHexadecimal: 0x90, pressInteger: 1144, releaseInteger: 2144 },
                WowKeyInfo { keyName: "ScrollLock".to_string(), windowDecimal: 145, windowHexadecimal: 0x91, pressInteger: 1145, releaseInteger: 2145 },
                WowKeyInfo { keyName: "LeftShift".to_string(), windowDecimal: 160, windowHexadecimal: 0xA0, pressInteger: 1160, releaseInteger: 2160 },
                WowKeyInfo { keyName: "RightShift".to_string(), windowDecimal: 161, windowHexadecimal: 0xA1, pressInteger: 1161, releaseInteger: 2161 },
                WowKeyInfo { keyName: "LeftControl".to_string(), windowDecimal: 162, windowHexadecimal: 0xA2, pressInteger: 1162, releaseInteger: 2162 },
                WowKeyInfo { keyName: "RightControl".to_string(), windowDecimal: 163, windowHexadecimal: 0xA3, pressInteger: 1163, releaseInteger: 2163 },
                WowKeyInfo { keyName: "LeftAlt".to_string(), windowDecimal: 164, windowHexadecimal: 0xA4, pressInteger: 1164, releaseInteger: 2164 },
                WowKeyInfo { keyName: "RightAlt".to_string(), windowDecimal: 165, windowHexadecimal: 0xA5, pressInteger: 1165, releaseInteger: 2165 },
                WowKeyInfo { keyName: "LeftMenu".to_string(), windowDecimal: 164, windowHexadecimal: 0xA4, pressInteger: 1164, releaseInteger: 2164 },
                WowKeyInfo { keyName: "RightMenu".to_string(), windowDecimal: 165, windowHexadecimal: 0xA5, pressInteger: 1165, releaseInteger: 2165 },
                WowKeyInfo { keyName: "BrowserBack".to_string(), windowDecimal: 166, windowHexadecimal: 0xA6, pressInteger: 1166, releaseInteger: 2166 },
                WowKeyInfo { keyName: "BrowserForward".to_string(), windowDecimal: 167, windowHexadecimal: 0xA7, pressInteger: 1167, releaseInteger: 2167 },
                WowKeyInfo { keyName: "BrowserRefresh".to_string(), windowDecimal: 168, windowHexadecimal: 0xA8, pressInteger: 1168, releaseInteger: 2168 },
                WowKeyInfo { keyName: "BrowserStop".to_string(), windowDecimal: 169, windowHexadecimal: 0xA9, pressInteger: 1169, releaseInteger: 2169 },
                WowKeyInfo { keyName: "BrowserSearch".to_string(), windowDecimal: 170, windowHexadecimal: 0xAA, pressInteger: 1170, releaseInteger: 2170 },
                WowKeyInfo { keyName: "BrowserFavorites".to_string(), windowDecimal: 171, windowHexadecimal: 0xAB, pressInteger: 1171, releaseInteger: 2171 },
                WowKeyInfo { keyName: "BrowserHome".to_string(), windowDecimal: 172, windowHexadecimal: 0xAC, pressInteger: 1172, releaseInteger: 2172 },
                WowKeyInfo { keyName: "VolumeMute".to_string(), windowDecimal: 173, windowHexadecimal: 0xAD, pressInteger: 1173, releaseInteger: 2173 },
                WowKeyInfo { keyName: "VolumeDown".to_string(), windowDecimal: 174, windowHexadecimal: 0xAE, pressInteger: 1174, releaseInteger: 2174 },
                WowKeyInfo { keyName: "VolumeUp".to_string(), windowDecimal: 175, windowHexadecimal: 0xAF, pressInteger: 1175, releaseInteger: 2175 },
                WowKeyInfo { keyName: "MediaNextTrack".to_string(), windowDecimal: 176, windowHexadecimal: 0xB0, pressInteger: 1176, releaseInteger: 2176 },
                WowKeyInfo { keyName: "MediaPreviousTrack".to_string(), windowDecimal: 177, windowHexadecimal: 0xB1, pressInteger: 1177, releaseInteger: 2177 },
                WowKeyInfo { keyName: "MediaStop".to_string(), windowDecimal: 178, windowHexadecimal: 0xB2, pressInteger: 1178, releaseInteger: 2178 },
                WowKeyInfo { keyName: "MediaPlay".to_string(), windowDecimal: 179, windowHexadecimal: 0xB3, pressInteger: 1179, releaseInteger: 2179 },
                WowKeyInfo { keyName: "LaunchMail".to_string(), windowDecimal: 180, windowHexadecimal: 0xB4, pressInteger: 1180, releaseInteger: 2180 },
                WowKeyInfo { keyName: "LaunchMediaSelect".to_string(), windowDecimal: 181, windowHexadecimal: 0xB5, pressInteger: 1181, releaseInteger: 2181 },
                WowKeyInfo { keyName: "LaunchApp1".to_string(), windowDecimal: 182, windowHexadecimal: 0xB6, pressInteger: 1182, releaseInteger: 2182 },
                WowKeyInfo { keyName: "LaunchApp2".to_string(), windowDecimal: 183, windowHexadecimal: 0xB7, pressInteger: 1183, releaseInteger: 2183 },
                WowKeyInfo { keyName: "OEM1".to_string(), windowDecimal: 186, windowHexadecimal: 0xBA, pressInteger: 1186, releaseInteger: 2186 },
                WowKeyInfo { keyName: "OEMPlus".to_string(), windowDecimal: 187, windowHexadecimal: 0xBB, pressInteger: 1187, releaseInteger: 2187 },
                WowKeyInfo { keyName: "OEMComma".to_string(), windowDecimal: 188, windowHexadecimal: 0xBC, pressInteger: 1188, releaseInteger: 2188 },
                WowKeyInfo { keyName: "OEMMinus".to_string(), windowDecimal: 189, windowHexadecimal: 0xBD, pressInteger: 1189, releaseInteger: 2189 },
                WowKeyInfo { keyName: "OEMPeriod".to_string(), windowDecimal: 190, windowHexadecimal: 0xBE, pressInteger: 1190, releaseInteger: 2190 },
                WowKeyInfo { keyName: "OEM2".to_string(), windowDecimal: 191, windowHexadecimal: 0xBF, pressInteger: 1191, releaseInteger: 2191 },
                WowKeyInfo { keyName: "OEM3".to_string(), windowDecimal: 192, windowHexadecimal: 0xC0, pressInteger: 1192, releaseInteger: 2192 },
                WowKeyInfo { keyName: "OEM4".to_string(), windowDecimal: 219, windowHexadecimal: 0xDB, pressInteger: 1219, releaseInteger: 2219 },
                WowKeyInfo { keyName: "OEM5".to_string(), windowDecimal: 220, windowHexadecimal: 0xDC, pressInteger: 1220, releaseInteger: 2220 },
                WowKeyInfo { keyName: "OEM6".to_string(), windowDecimal: 221, windowHexadecimal: 0xDD, pressInteger: 1221, releaseInteger: 2221 },
                WowKeyInfo { keyName: "OEM7".to_string(), windowDecimal: 222, windowHexadecimal: 0xDE, pressInteger: 1222, releaseInteger: 2222 },
                WowKeyInfo { keyName: "OEM8".to_string(), windowDecimal: 223, windowHexadecimal: 0xDF, pressInteger: 1223, releaseInteger: 2223 },
                WowKeyInfo { keyName: "OEM102".to_string(), windowDecimal: 226, windowHexadecimal: 0xE2, pressInteger: 1226, releaseInteger: 2226 },
                WowKeyInfo { keyName: "ProcessKey".to_string(), windowDecimal: 229, windowHexadecimal: 0xE5, pressInteger: 1229, releaseInteger: 2229 },
                WowKeyInfo { keyName: "Packet".to_string(), windowDecimal: 231, windowHexadecimal: 0xE7, pressInteger: 1231, releaseInteger: 2231 },
                WowKeyInfo { keyName: "Attn".to_string(), windowDecimal: 246, windowHexadecimal: 0xF6, pressInteger: 1246, releaseInteger: 2246 },
                WowKeyInfo { keyName: "CrSel".to_string(), windowDecimal: 247, windowHexadecimal: 0xF7, pressInteger: 1247, releaseInteger: 2247 },
                WowKeyInfo { keyName: "ExSel".to_string(), windowDecimal: 248, windowHexadecimal: 0xF8, pressInteger: 1248, releaseInteger: 2248 },
                WowKeyInfo { keyName: "EraseEOF".to_string(), windowDecimal: 249, windowHexadecimal: 0xF9, pressInteger: 1249, releaseInteger: 2249 },
                WowKeyInfo { keyName: "Play".to_string(), windowDecimal: 250, windowHexadecimal: 0xFA, pressInteger: 1250, releaseInteger: 2250 },
                WowKeyInfo { keyName: "Zoom".to_string(), windowDecimal: 251, windowHexadecimal: 0xFB, pressInteger: 1251, releaseInteger: 2251 },
                WowKeyInfo { keyName: "PA1".to_string(), windowDecimal: 253, windowHexadecimal: 0xFD, pressInteger: 1253, releaseInteger: 2253 }
            ],
        }
    }
}


pub enum WowWindowKeyInt {
    Backspace = 8,
    Tab = 9,
    Clear = 12,
    Enter = 13,
    Pause = 19,
    CapsLock = 20,
    Escape = 27,
    Space = 32,
    PageUp = 33,
    PageDown = 34,
    End = 35,
    Home = 36,
    LeftArrow = 37,
    UpArrow = 38,
    RightArrow = 39,
    DownArrow = 40,
    Select = 41,
    Print = 42,
    Execute = 43,
    PrintScreen = 44,
    Insert = 45,
    Delete = 46,
    Help = 47,
    LeftWindows = 91,
    RightWindows = 92,
    Applications = 93,
    Sleep = 95,
    Numpad0 = 96,
    Numpad1 = 97,
    Numpad2 = 98,
    Numpad3 = 99,
    Numpad4 = 100,
    Numpad5 = 101,
    Numpad6 = 102,
    Numpad7 = 103,
    Numpad8 = 104,
    Numpad9 = 105,
    Multiply = 106,
    Add = 107,
    Separator = 108,
    Subtract = 109,
    Decimal = 110,
    Divide = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    F13 = 124,
    F14 = 125,
    F15 = 126,
    F16 = 127,
    F17 = 128,
    F18 = 129,
    F19 = 130,
    F20 = 131,
    F21 = 132,
    F22 = 133,
    F23 = 134,
    F24 = 135,
    NumLock = 144,
    ScrollLock = 145,
    LeftShift = 160,
    RightShift = 161,
    LeftControl = 162,
    RightControl = 163,
    LeftAlt = 164,
    RightAlt = 165,
    BrowserBack = 166,
    BrowserForward = 167,
    BrowserRefresh = 168,
    BrowserStop = 169,
    BrowserSearch = 170,
    BrowserFavorites = 171,
    BrowserHome = 172,
    VolumeMute = 173,
    VolumeDown = 174,
    VolumeUp = 175,
    MediaNextTrack = 176,
    MediaPreviousTrack = 177,
    MediaStop = 178,
    MediaPlay = 179,
    LaunchMail = 180,
    LaunchMediaSelect = 181,
    LaunchApp1 = 182,
    LaunchApp2 = 183,
    OEM1 = 186,
    OEMPlus = 187,
    OEMComma = 188,
    OEMMinus = 189,
    OEMPeriod = 190,
    OEM2 = 191,
    OEM3 = 192,
    OEM4 = 219,
    OEM5 = 220,
    OEM6 = 221,
    OEM7 = 222,
    OEM8 = 223,
    OEM102 = 226,
    ProcessKey = 229,
    Packet = 231,
    Attn = 246,
    CrSel = 247,
    ExSel = 248,
    EraseEOF = 249,
    Play = 250,
    Zoom = 251,
    PA1 = 253,
}

impl WowWindowKeyInt {
    

    // Method to convert a key to its string name
    fn key_name(&self) -> &'static str {
        match self {
            WowWindowKeyInt::Backspace => "Backspace",
            WowWindowKeyInt::Tab => "Tab",
            WowWindowKeyInt::Clear => "Clear",
            WowWindowKeyInt::Enter => "Enter",
            WowWindowKeyInt::Pause => "Pause",
            WowWindowKeyInt::CapsLock => "CapsLock",
            WowWindowKeyInt::Escape => "Escape",
            WowWindowKeyInt::Space => "Space",
            WowWindowKeyInt::PageUp => "PageUp",
            WowWindowKeyInt::PageDown => "PageDown",
            WowWindowKeyInt::End => "End",
            WowWindowKeyInt::Home => "Home",
            WowWindowKeyInt::LeftArrow => "LeftArrow",
            WowWindowKeyInt::UpArrow => "UpArrow",
            WowWindowKeyInt::RightArrow => "RightArrow",
            WowWindowKeyInt::DownArrow => "DownArrow",
            WowWindowKeyInt::Select => "Select",
            WowWindowKeyInt::Print => "Print",
            WowWindowKeyInt::Execute => "Execute",
            WowWindowKeyInt::PrintScreen => "PrintScreen",
            WowWindowKeyInt::Insert => "Insert",
            WowWindowKeyInt::Delete => "Delete",
            WowWindowKeyInt::Help => "Help",
            WowWindowKeyInt::LeftWindows => "LeftWindows",
            WowWindowKeyInt::RightWindows => "RightWindows",
            WowWindowKeyInt::Applications => "Applications",
            WowWindowKeyInt::Sleep => "Sleep",
            WowWindowKeyInt::Numpad0 => "Numpad0",
            WowWindowKeyInt::Numpad1 => "Numpad1",
            WowWindowKeyInt::Numpad2 => "Numpad2",
            WowWindowKeyInt::Numpad3 => "Numpad3",
            WowWindowKeyInt::Numpad4 => "Numpad4",
            WowWindowKeyInt::Numpad5 => "Numpad5",
            WowWindowKeyInt::Numpad6 => "Numpad6",
            WowWindowKeyInt::Numpad7 => "Numpad7",
            WowWindowKeyInt::Numpad8 => "Numpad8",
            WowWindowKeyInt::Numpad9 => "Numpad9",
            WowWindowKeyInt::Multiply => "Multiply",
            WowWindowKeyInt::Add => "Add",
            WowWindowKeyInt::Separator => "Separator",
            WowWindowKeyInt::Subtract => "Subtract",
            WowWindowKeyInt::Decimal => "Decimal",
            WowWindowKeyInt::Divide => "Divide",
            WowWindowKeyInt::F1 => "F1",
            WowWindowKeyInt::F2 => "F2",
            WowWindowKeyInt::F3 => "F3",
            WowWindowKeyInt::F4 => "F4",
            WowWindowKeyInt::F5 => "F5",
            WowWindowKeyInt::F6 => "F6",
            WowWindowKeyInt::F7 => "F7",
            WowWindowKeyInt::F8 => "F8",
            WowWindowKeyInt::F9 => "F9",
            WowWindowKeyInt::F10 => "F10",
            WowWindowKeyInt::F11 => "F11",
            WowWindowKeyInt::F12 => "F12",
            WowWindowKeyInt::F13 => "F13",
            WowWindowKeyInt::F14 => "F14",
            WowWindowKeyInt::F15 => "F15",
            WowWindowKeyInt::F16 => "F16",
            WowWindowKeyInt::F17 => "F17",
            WowWindowKeyInt::F18 => "F18",
            WowWindowKeyInt::F19 => "F19",
            WowWindowKeyInt::F20 => "F20",
            WowWindowKeyInt::F21 => "F21",
            WowWindowKeyInt::F22 => "F22",
            WowWindowKeyInt::F23 => "F23",
            WowWindowKeyInt::F24 => "F24",
            WowWindowKeyInt::NumLock => "NumLock",
            WowWindowKeyInt::ScrollLock => "ScrollLock",
            WowWindowKeyInt::LeftShift => "LeftShift",
            WowWindowKeyInt::RightShift => "RightShift",
            WowWindowKeyInt::LeftControl => "LeftControl",
            WowWindowKeyInt::RightControl => "RightControl",
            WowWindowKeyInt::LeftAlt => "LeftAlt",
            WowWindowKeyInt::RightAlt => "RightAlt",
            WowWindowKeyInt::LeftAlt => "LeftMenu",
            WowWindowKeyInt::RightAlt => "RightMenu",
            WowWindowKeyInt::BrowserBack => "BrowserBack",
            WowWindowKeyInt::BrowserForward => "BrowserForward",
            WowWindowKeyInt::BrowserRefresh => "BrowserRefresh",
            WowWindowKeyInt::BrowserStop => "BrowserStop",
            WowWindowKeyInt::BrowserSearch => "BrowserSearch",
            WowWindowKeyInt::BrowserFavorites => "BrowserFavorites",
            WowWindowKeyInt::BrowserHome => "BrowserHome",
            WowWindowKeyInt::VolumeMute => "VolumeMute",
            WowWindowKeyInt::VolumeDown => "VolumeDown",
            WowWindowKeyInt::VolumeUp => "VolumeUp",
            WowWindowKeyInt::MediaNextTrack => "MediaNextTrack",
            WowWindowKeyInt::MediaPreviousTrack => "MediaPreviousTrack",
            WowWindowKeyInt::MediaStop => "MediaStop",
            WowWindowKeyInt::MediaPlay => "MediaPlay",
            WowWindowKeyInt::LaunchMail => "LaunchMail",
            WowWindowKeyInt::LaunchMediaSelect => "LaunchMediaSelect",
            WowWindowKeyInt::LaunchApp1 => "LaunchApp1",
            WowWindowKeyInt::LaunchApp2 => "LaunchApp2",
            WowWindowKeyInt::OEM1 => "OEM1",
            WowWindowKeyInt::OEMPlus => "OEMPlus",
            WowWindowKeyInt::OEMComma => "OEMComma",
            WowWindowKeyInt::OEMMinus => "OEMMinus",
            WowWindowKeyInt::OEMPeriod => "OEMPeriod",
            WowWindowKeyInt::OEM2 => "OEM2",
            WowWindowKeyInt::OEM3 => "OEM3",
            WowWindowKeyInt::OEM4 => "OEM4",
            WowWindowKeyInt::OEM5 => "OEM5",
            WowWindowKeyInt::OEM6 => "OEM6",
            WowWindowKeyInt::OEM7 => "OEM7",
            WowWindowKeyInt::OEM8 => "OEM8",
            WowWindowKeyInt::OEM102 => "OEM102",
            WowWindowKeyInt::ProcessKey => "ProcessKey",
            WowWindowKeyInt::Packet => "Packet",
            WowWindowKeyInt::Attn => "Attn",
            WowWindowKeyInt::CrSel => "CrSel",
            WowWindowKeyInt::ExSel => "ExSel",
            WowWindowKeyInt::EraseEOF => "EraseEOF",
            WowWindowKeyInt::Play => "Play",
            WowWindowKeyInt::Zoom => "Zoom",
            WowWindowKeyInt::PA1 => "PA1",
        }
    }
}
