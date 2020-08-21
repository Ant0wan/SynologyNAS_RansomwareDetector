use std::fs::File;
use std::io;

/* Maximum suspicious action limit */
pub const BAN_LIMIT: u16 = 50;

pub mod sms {
    pub fn send() {}
}

pub mod email {

    pub fn send() -> Result<std::fs::File, std::io::Error> {
        let mut f = std::fs::File::open("email.txt")?;
        Ok(f)
    }
}
