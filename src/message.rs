use core::mem::size_of;

// message format:
//
//   message header:
//
//      0
//      0 1 2 3 4 5 6 7
//     +-+-+-+-+-+-+-+-+
//     |   data type   |
//     +-+-+-+-+-+-+-+-+
//
//   message types:
//
//     0x00 RES (REServed)
//       idk why, just kinda like it that way
//
//     0x01 TEL (TELemetry)
//       A regular telemetry message.
//       The device constantly sends this type of message.
//       Payload sent after the header:
//
//        0                   1
//        0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
//       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//       |  X high bits  |  X low bits   |
//       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//       |  Y high bits  |  Y low bits   |
//       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//       |  Y high bits  |  Z low bits   |
//       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
//     0x02 SAT (Set ATtitude)
//       A message requesting new attitude of the device.
//       The desktop software sends this type of message.
//       Payload sent after the header:
//
//        0                   1
//        0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
//       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//       |  X high bits  |  X low bits   |
//       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//       |  Y high bits  |  Y low bits   |
//       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//       |  Y high bits  |  Z low bits   |
//       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
//     0x03 NAS (New Attitude Successful)
//       A message indicating that the previously requested attitude has been
//       successfully achieved.
//       The device sends this type of message only after it has received a
//       corresponding SAT message.
//       There is no payload sent after the header.
//

// pub struct MessageHeader {
//     data_type: u8,
// }

pub struct TEL {
    pub x_h: u8,
    pub x_l: u8,
    pub y_h: u8,
    pub y_l: u8,
    pub z_h: u8,
    pub z_l: u8,
}

impl TEL {
    pub fn from_fixed(buf: &[u8; size_of::<Self>()]) -> Self {
        Self {
            x_h: buf[0], x_l: buf[1],
            y_h: buf[2], y_l: buf[3],
            z_h: buf[4], z_l: buf[5]
        }
    }

    pub fn get_x(&self) -> i16 {
        i16::from_be_bytes([self.x_h, self.x_l])
    }

    pub fn get_y(&self) -> i16 {
        i16::from_be_bytes([self.y_h, self.y_l])
    }

    pub fn get_z(&self) -> i16 {
        i16::from_be_bytes([self.z_h, self.z_l])
    }
}

pub struct SAT {
    pub x_h: u8,
    pub x_l: u8,
    pub y_h: u8,
    pub y_l: u8,
    pub z_h: u8,
    pub z_l: u8,
}

impl SAT {
    pub fn from_fixed(buf: &[u8; size_of::<Self>()]) -> Self {
        Self {
            x_h: buf[0], x_l: buf[1],
            y_h: buf[2], y_l: buf[3],
            z_h: buf[4], z_l: buf[5]
        }
    }

    pub fn get_x(&self) -> i16 {
        i16::from_be_bytes([self.x_h, self.x_l])
    }

    pub fn get_y(&self) -> i16 {
        i16::from_be_bytes([self.y_h, self.y_l])
    }

    pub fn get_z(&self) -> i16 {
        i16::from_be_bytes([self.z_h, self.z_l])
    }
}

pub enum MessagePayload {
    RES,
    TEL(TEL),
    SAT(SAT),
    NAS,
}

enum PayloadBuffer {
    TEL([u8; 6]),
    SAT([u8; 6]),
}

pub struct Message {
    // pub header: MessageHeader,
    header_buffer: [u8; 1],
    header_tail: usize,
    payload_started: bool,
    tel_payload_buffer: [u8; size_of::<TEL>()],
    sat_payload_buffer: [u8; size_of::<SAT>()],
    payload_tail: usize,
    pub payload: Option<MessagePayload>,
    // pub x_h: u8,
    // pub x_l: u8,
    // pub y_h: u8,
    // pub y_l: u8,
    // pub z_h: u8,
    // pub z_l: u8,
}

pub enum PushState {
    Continue,
    Done,
    Err,
}

impl Message {
    pub fn new() -> Self {
        Message {
            header_buffer: [0x00],
            header_tail: 0,
            payload_started: false,
            tel_payload_buffer: [0x00; 6],
            sat_payload_buffer: [0x00; 6],
            payload_tail: 0,
            payload: None
        }
    }

    pub fn push_byte(&mut self, byte: u8) -> PushState {

        if self.header_tail < self.header_buffer.len() {
            self.header_buffer[0] = byte;
            self.header_tail += 1;
        }
        match self.header_buffer[0] {
            0x00 => {
                self.header_tail = 0;
                PushState::Done
            }
            0x01 => {
                if self.payload_started {
                    self.tel_payload_buffer[self.payload_tail] = byte;
                    self.payload_tail += 1;

                    if self.payload_tail == self.tel_payload_buffer.len() {
                        // for b in payload_buffer {
                        //     print!("{} ", b);
                        // }
                        // println!("");
                        self.payload = Some(MessagePayload::TEL(
                            TEL::from_fixed(&self.tel_payload_buffer)
                        ));
                        self.header_tail = 0;
                        self.payload_tail = 0;
                        self.payload_started = false;
                        PushState::Done
                    } else {
                        PushState::Continue
                    }
                } else {
                    self.tel_payload_buffer = [0x00; 6];
                    self.payload_started = true;
                    PushState::Continue
                }
            },
            0x02 => {
                if self.payload_started {
                    self.sat_payload_buffer[self.payload_tail] = byte;
                    self.payload_tail += 1;

                    if self.payload_tail == self.sat_payload_buffer.len() {
                        // for b in payload_buffer {
                        //     print!("{} ", b);
                        // }
                        // println!("");
                        self.payload = Some(MessagePayload::SAT(
                            SAT::from_fixed(&self.sat_payload_buffer)
                        ));
                        self.header_tail = 0;
                        self.payload_tail = 0;
                        self.payload_started = false;
                        PushState::Done
                    } else {
                        PushState::Continue
                    }
                } else {
                    self.sat_payload_buffer = [0x00; 6];
                    self.payload_started = true;
                    PushState::Continue
                }
            },
            0x03 => {
                // trigger event?
                // println!("NAS");
                self.header_tail = 0;
                PushState::Done
            },
            _ => {
                self.header_tail = 0;
                PushState::Err
            }
        }
    }

    // pub fn with_payload(payload: MessagePayload) -> Self {
    //     Self {
    //         header: match payload {
    //             MessagePayload::RES => 0x00,
    //             MessagePayload::TEL(_) => 0x01,
    //             MessagePayload::SAT(_) => 0x02,
    //             MessagePayload::NAS => 0x03,
    //         },
    //         payload: payload
    //     }
    // }

    // pub fn new_res() -> Self {
    //     Self {
    //         header: MessageHeader { data_type: 0x00 },
    //         payload: None
    //     }
    // }

    // pub fn new_tel(payload: MessagePayload) -> Self {
    //     Self {
    //         header: MessageHeader { data_type: 0x01 },
    //         payload: Some(payload)
    //     }
    // }

    // pub fn new_sat(payload: MessagePayload) -> Self {
    //     Self {
    //         header: MessageHeader { data_type: 0x02 },
    //         payload: Some(payload)
    //     }
    // }

    // pub fn new_nas() -> Self {
    //     Self {
    //         header: MessageHeader { data_type: 0x03 },
    //         payload: None
    //     }
    // }

	// pub fn from_(x: u16, y: u16, z: u16) -> Self {
	// 	Self {

	// 	}
	// }

    // pub fn from_fixed(buf: &[u8; size_of::<Self>()]) -> Self {
    //     Self {
    //         x_h: buf[0], x_l: buf[1],
    //         y_h: buf[2], y_l: buf[3],
    //         z_h: buf[4], z_l: buf[5]
    //     }
    // }

    // pub fn from_vec(buf: &Vec<u8>) -> Result<Self, ()> {
    //     if buf.len() < size_of::<Self>() {
    //         Err(())
    //     } else {
    //         Ok(Self {
    //             x_h: buf[0], x_l: buf[1],
    //             y_h: buf[2], y_l: buf[3],
    //             z_h: buf[4], z_l: buf[5]
    //         })
    //     }
    // }

    // pub fn from(buf: [u8; size_of::<Self>()]) -> Self {
    //     Self {
    //         x_h: buf[0], x_l: buf[1],
    //         y_h: buf[2], y_l: buf[3],
    //         z_h: buf[4], z_l: buf[5]
    //     }
    // }

    // pub fn to_string(&self) -> String {
    //     format!("x: {}, y: {}, z: {}", self.get_x(), self.get_y(), self.get_z())
    // }

    // pub fn as_bytes(&self) -> [u8; size_of::<Self>()] {
    //     return [
    //         self.x_h, self.x_l,
    //         self.y_h, self.y_l,
    //         self.z_h, self.z_l,
    //     ];
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tel_from_fixed() {
        let tel = TEL::from_fixed(&[
            0x12, 0x34,
            0x56, 0x78,
            0x9a, 0xbc
        ]);
        assert_eq!(tel.x_h, 0x12);
        assert_eq!(tel.x_l, 0x34);
        assert_eq!(tel.y_h, 0x56);
        assert_eq!(tel.y_l, 0x78);
        assert_eq!(tel.z_h, 0x9a);
        assert_eq!(tel.z_l, 0xbc);
    }

    #[test]
    fn test_tel_get_xyz() {
        let tel = TEL {
            x_h: 0x12,
            x_l: 0x34,
            y_h: 0x56,
            y_l: 0x78,
            z_h: 0x9a,
            z_l: 0xbc
        };
        assert_eq!(tel.get_x(), 0x1234);
        assert_eq!(tel.get_y(), 0x5678);
        assert_eq!(tel.get_z(), 0x9abcu16 as i16);
    }

    #[test]
    fn test_sat_from_fixed() {
        let sat = SAT::from_fixed(&[
            0x12, 0x34,
            0x56, 0x78,
            0x9a, 0xbc
        ]);
        assert_eq!(sat.x_h, 0x12);
        assert_eq!(sat.x_l, 0x34);
        assert_eq!(sat.y_h, 0x56);
        assert_eq!(sat.y_l, 0x78);
        assert_eq!(sat.z_h, 0x9a);
        assert_eq!(sat.z_l, 0xbc);
    }

    #[test]
    fn test_sat_get_xyz() {
        let sat = SAT {
            x_h: 0x12,
            x_l: 0x34,
            y_h: 0x56,
            y_l: 0x78,
            z_h: 0x9a,
            z_l: 0xbc
        };
        assert_eq!(sat.get_x(), 0x1234);
        assert_eq!(sat.get_y(), 0x5678);
        assert_eq!(sat.get_z(), 0x9abcu16 as i16);
    }

    #[test]
    fn test_message_new() {
        let message = Message::new();
        assert_eq!(message.header_buffer, [0x00; 1]);
        assert_eq!(message.header_tail, 0);
        assert_eq!(message.payload_started, false);
        assert_eq!(message.tel_payload_buffer, [0x00; 6]);
        assert_eq!(message.sat_payload_buffer, [0x00; 6]);
        assert_eq!(message.payload_tail, 0);
        assert!(matches!(message.payload, None));
    }

    #[test]
    fn test_push_byte_tel() {
        let mut message = Message::new();
        assert!(matches!(message.push_byte(0x01), PushState::Continue)); // TEL
        assert!(matches!(message.push_byte(0x12), PushState::Continue)); // x_h
        assert!(matches!(message.push_byte(0x34), PushState::Continue)); // x_l
        assert!(matches!(message.push_byte(0x56), PushState::Continue)); // y_h
        assert!(matches!(message.push_byte(0x78), PushState::Continue)); // y_l
        assert!(matches!(message.push_byte(0x9a), PushState::Continue)); // z_h
        assert!(matches!(message.push_byte(0xbc), PushState::Done));     // z_l
        assert!(matches!(message.payload, Some(MessagePayload::TEL(TEL {
            x_h: 0x12, x_l: 0x34,
            y_h: 0x56, y_l: 0x78,
            z_h: 0x9a, z_l: 0xbc
        }))));
    }

    #[test]
    fn test_push_byte_sat() {
        let mut message = Message::new();
        assert!(matches!(message.push_byte(0x02), PushState::Continue)); // SAT
        assert!(matches!(message.push_byte(0x12), PushState::Continue)); // x_h
        assert!(matches!(message.push_byte(0x34), PushState::Continue)); // x_l
        assert!(matches!(message.push_byte(0x56), PushState::Continue)); // y_h
        assert!(matches!(message.push_byte(0x78), PushState::Continue)); // y_l
        assert!(matches!(message.push_byte(0x9a), PushState::Continue)); // z_h
        assert!(matches!(message.push_byte(0xbc), PushState::Done));     // z_l
        assert!(matches!(message.payload, Some(MessagePayload::SAT(SAT {
            x_h: 0x12, x_l: 0x34,
            y_h: 0x56, y_l: 0x78,
            z_h: 0x9a, z_l: 0xbc
        }))));
    }

    #[test]
    fn test_push_byte_nas() {
        let mut message = Message::new();
        assert!(matches!(message.push_byte(0x03), PushState::Done)); // NAS
        assert!(matches!(message.payload, None));
    }

    #[test]
    fn test_push_byte_error() {
        let mut message = Message::new();
        assert!(matches!(message.push_byte(0x04), PushState::Err));
    }

    // #[test]
    // fn test_from_fixed() {
    //     let test_frame = Message::from_fixed(
    //         &[0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]
    //     );
    //     assert_eq!(test_frame.x_h, 0x12);
    //     assert_eq!(test_frame.x_l, 0x34);
    //     assert_eq!(test_frame.y_h, 0x56);
    //     assert_eq!(test_frame.y_l, 0x78);
    //     assert_eq!(test_frame.z_h, 0x9a);
    //     assert_eq!(test_frame.z_l, 0xbc);
    // }

	// #[test]
	// fn test_as_bytes() {
	// 	let test_frame = Message {
	// 		x_h: 0x12, x_l: 0x34,
	// 		y_h: 0x56, y_l: 0x78,
	// 		z_h: 0x9a, z_l: 0xbc
	// 	};
	// 	assert_eq!(test_frame.as_bytes(), [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]);
	// }
}
