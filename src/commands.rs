pub enum Command {
    SetAttitude,
}

impl Command {
    pub fn from(command_number: u8) -> Result<Self, ()> {
        match command_number {
            0x01 => Ok(Self::SetAttitude),
            _ => Err(())
        }
    }
}
