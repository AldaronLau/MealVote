use ranch::RangedNonZeroU16;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Error {
    Zero,
    TooLarge(u16),
}

/// Home ID (more than 0, less than 1\_024)
#[derive(
    Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize, serde::Serialize,
)]
#[repr(transparent)]
pub struct HomeId(RangedNonZeroU16<1, HOME_ID_MAX_VALUE>);

impl HomeId {
    pub fn new(value: u16) -> Result<Self> {
        Ok(Self(
            RangedNonZeroU16::with_u16(value)
                .map_err(|_| Error::TooLarge(value))?
                .ok_or(Error::Zero)?,
        ))
    }
}

const HOME_ID_MAX_VALUE: u16 = 1_024;
