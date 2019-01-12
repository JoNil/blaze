use std::env;
use std::error::Error;
use std::mem;
use std::net::UdpSocket;
use std::io::ErrorKind;
use structview::{u32_le, View};

const MESSAGE_MAGIC: u32 = 0x768DD123;
const MESSAGE_COMMAND_LEFT: u32 = 1;
const MESSAGE_COMMAND_RIGHT: u32 = 2;
const MESSAGE_COMMAND_UP: u32 = 3;
const MESSAGE_COMMAND_DOWN: u32 = 4;
const MESSAGE_COMMAND_ACTION: u32 = 5;

#[derive(Clone, Copy, View)]
#[repr(C)]
struct Message {
    magic: u32_le,
    command: u32_le,
    up_down: u32_le,
}

pub struct Input {
    socket: UdpSocket,

    pub left_down: bool,
    pub right_down: bool,
    pub up_down: bool,
    pub down_down: bool,
    pub action_down: bool,
}

impl Input {
    pub fn new() -> Result<Input, Box<dyn Error>> {
        let socket = UdpSocket::bind("0.0.0.0:15171")?;
        socket.set_nonblocking(true)?;

        Ok(Input {
            socket: socket,
            left_down: false,
            right_down: false,
            up_down: false,
            down_down: false,
            action_down: false,
        })
    }

    pub fn poll(&mut self) {
        let mut buf = [0; mem::size_of::<Message>()];

        loop {
            let res = self.socket.recv(&mut buf);

            match res {
                Ok(amt) => {
                    if amt == mem::size_of::<Message>() {
                        if let Ok(message) = Message::view(&buf) {
                            if message.magic.to_int() == MESSAGE_MAGIC {
                                match message.command.to_int() {
                                    MESSAGE_COMMAND_LEFT => {
                                        self.left_down = message.up_down.to_int() == 1;
                                    }
                                    MESSAGE_COMMAND_RIGHT => {
                                        self.right_down = message.up_down.to_int() == 1;
                                    }
                                    MESSAGE_COMMAND_UP => {
                                        self.up_down = message.up_down.to_int() == 1;
                                    }
                                    MESSAGE_COMMAND_DOWN => {
                                        self.down_down = message.up_down.to_int() == 1;
                                    }
                                    MESSAGE_COMMAND_ACTION => {
                                        self.action_down = message.up_down.to_int() == 1;
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => {
                    break;
                }
                Err(ref err) => {
                    panic!("{}", err);
                }
            }
        }
    }
}
