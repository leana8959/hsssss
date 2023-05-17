// Telnet protocol characters
const IAC: u8 = 255; // "Interpret As Command"
const DONT: u8 = 254;
const DO: u8 = 253;
const WONT: u8 = 252;
const WILL: u8 = 251;
const NULL: u8 = 0;

const SB: u8 = 250; // Subnegotiation Begin;
const SE: u8 = 240; // Subnegotiation End;
const NOP: u8 = 241; // No Operation;
const DM: u8 = 242; // Data Mark;
const BRK: u8 = 243; // Break;
const IP: u8 = 244; // Interrupt process;
const AO: u8 = 245; // Abort output;
const AYT: u8 = 246; // Are You There;
const EC: u8 = 247; // Erase Character;
const EL: u8 = 248; // Erase Line;
const GA: u8 = 249; // Go Ahead;

// telnet options
const NAWS: u8 = 31;
const TM: u8 = 6;

#[derive(Default)]
pub struct TelnetParser {
    width: u8,
    height: u8,
    response: Vec<u8>,
    exit_now: bool,
}

impl TelnetParser {
    pub fn new() -> Self {
        TelnetParser {
            response: Vec::with_capacity(1024),
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.response.clear();
    }

    pub fn read_codes(&mut self, codes: &[u8]) {
        match codes {
            [] => (),

            // Responsd to terminal size request
            [IAC, WILL, NAWS, rest @ ..] => {
                self.response.append(&mut vec![IAC, DO, NAWS]);
                self.read_codes(rest);
            }
            // Parse terminal size
            [IAC, SB, NAWS, 0, width, 0, height, IAC, SE, rest @ ..] => {
                self.width = *width;
                self.height = *height;
                self.read_codes(rest);
            }

            // Override the ^C behaviour
            [IAC, IP, IAC, DO, TM, rest @ ..] => {
                self.exit_now = true;
                self.response.append(&mut vec![IAC, IP, IAC, WONT, TM]);
                self.read_codes(rest);
            }

            [IAC, DO, option, rest @ ..] => {
                self.response.append(&mut vec![IAC, WONT, *option]);
                self.read_codes(rest);
            }
            [IAC, WILL, option, rest @ ..] => {
                self.response.append(&mut vec![IAC, WONT, *option]);
                self.read_codes(rest);
            }
            _ => {
                eprintln!("Unimplemented sequence: {:?}", codes);
            }
        }
    }

    pub fn respond(&self) -> &[u8] {
        &self.response[..]
    }

    pub fn exit_now(&self) -> bool {
        self.exit_now
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn width(&self) -> u8 {
        self.width
    }
}
