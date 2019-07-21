use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

use base64;
use base64::DecodeError;

#[derive(Debug, Clone, Copy)]
pub struct Seed {
    pub raw_value: [u8; 32]
}

impl Display for Seed {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", base64::encode(&self.raw_value))
    }
}

impl FromStr for Seed {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = base64::decode(s)?;
        if bytes.len() != 32 {
            Result::Err(DecodeError::InvalidLength)
        } else {
            let mut raw_value = [0 as u8; 32];
            raw_value.copy_from_slice(&bytes);
            Result::Ok(Seed { raw_value })
        }
    }
}