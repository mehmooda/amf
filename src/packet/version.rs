use std::convert::TryFrom;

pub enum Version {
    AMF0,
    AMF3,
}

impl From<VersionError> for nom::Err<()> {
    fn from(_: VersionError) -> nom::Err<()> {
        nom::Err::Failure(())
    }
}

pub struct VersionError;

impl TryFrom<u16> for Version {
    type Error = VersionError;

    fn try_from(value: u16) -> Result<Version, VersionError> {
        match value {
            0 => Ok(Version::AMF0),
            3 => Ok(Version::AMF3),
            _ => Err(VersionError),
        }
    }
}
