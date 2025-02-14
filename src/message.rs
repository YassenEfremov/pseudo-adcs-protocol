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
//     0x02 SAT (Set Attitude)
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

// pub enum MessagePayload {
//     RES,
//     TEL([u8; 6]),
//     SAT([u8; 6]),
//     NAS,
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

// pub struct Message {
//     // pub header: MessageHeader,
//     // pub payload: Option<MessagePayload>,
//     pub x_h: u8,
//     pub x_l: u8,
//     pub y_h: u8,
//     pub y_l: u8,
//     pub z_h: u8,
//     pub z_l: u8,
// }

// impl Message {
//     // pub fn with_payload(payload: MessagePayload) -> Self {
//     //     Self {
//     //         header: match payload {
//     //             MessagePayload::RES => 0x00,
//     //             MessagePayload::TEL(_) => 0x01,
//     //             MessagePayload::SAT(_) => 0x02,
//     //             MessagePayload::NAS => 0x03,
//     //         },
//     //         payload: payload
//     //     }
//     // }

//     // pub fn new_res() -> Self {
//     //     Self {
//     //         header: MessageHeader { data_type: 0x00 },
//     //         payload: None
//     //     }
//     // }

//     // pub fn new_tel(payload: MessagePayload) -> Self {
//     //     Self {
//     //         header: MessageHeader { data_type: 0x01 },
//     //         payload: Some(payload)
//     //     }
//     // }

//     // pub fn new_sat(payload: MessagePayload) -> Self {
//     //     Self {
//     //         header: MessageHeader { data_type: 0x02 },
//     //         payload: Some(payload)
//     //     }
//     // }

//     // pub fn new_nas() -> Self {
//     //     Self {
//     //         header: MessageHeader { data_type: 0x03 },
//     //         payload: None
//     //     }
//     // }

// 	// pub fn from_(x: u16, y: u16, z: u16) -> Self {
// 	// 	Self {

// 	// 	}
// 	// }

//     pub fn from_fixed(buf: &[u8; size_of::<Self>()]) -> Self {
//         Self {
//             x_h: buf[0], x_l: buf[1],
//             y_h: buf[2], y_l: buf[3],
//             z_h: buf[4], z_l: buf[5]
//         }
//     }

//     // pub fn from_vec(buf: &Vec<u8>) -> Result<Self, ()> {
//     //     if buf.len() < size_of::<Self>() {
//     //         Err(())
//     //     } else {
//     //         Ok(Self {
//     //             x_h: buf[0], x_l: buf[1],
//     //             y_h: buf[2], y_l: buf[3],
//     //             z_h: buf[4], z_l: buf[5]
//     //         })
//     //     }
//     // }

//     // pub fn from(buf: [u8; size_of::<Self>()]) -> Self {
//     //     Self {
//     //         x_h: buf[0], x_l: buf[1],
//     //         y_h: buf[2], y_l: buf[3],
//     //         z_h: buf[4], z_l: buf[5]
//     //     }
//     // }

//     pub fn get_x(&self) -> i16 {
//         (((self.x_h as u16) << 8) | (self.x_l as u16)) as i16
//     }

//     pub fn get_y(&self) -> i16 {
//         (((self.y_h as u16) << 8) | (self.y_l as u16)) as i16
//     }

//     pub fn get_z(&self) -> i16 {
//         (((self.z_h as u16) << 8) | (self.z_l as u16)) as i16
//     }

//     // pub fn to_vec(&self) -> Vec<u8> {
//     //     vec![
//     //         self.x_h, self.x_l,
//     //         self.y_h, self.y_l,
//     //         self.z_h, self.z_l,
//     //     ]
//     // }

//     // pub fn to_string(&self) -> String {
//     //     format!("x: {}, y: {}, z: {}", self.get_x(), self.get_y(), self.get_z())
//     // }

//     pub fn as_bytes(&self) -> [u8; size_of::<Self>()] {
//         return [
//             self.x_h, self.x_l,
//             self.y_h, self.y_l,
//             self.z_h, self.z_l,
//         ];
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_new() {
    //     assert_eq!(
	// 		MyFrame::new(),
	// 		MyFrame {
	// 			x_h: 0, x_l: 0,
	// 			y_h: 0, y_l: 0,
	// 			z_h: 0, z_l: 0
	// 		}
	// 	);
    // }

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
