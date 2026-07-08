use ranch::RangedNonZeroU16;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Error {
    Zero,
    TooLarge(u16),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MealId(RangedNonZeroU16<1, MEAL_ID_MAX_VALUE>);

impl MealId {
    pub fn new(value: u16) -> Result<Self> {
        Ok(Self(
            RangedNonZeroU16::with_u16(value)
                .map_err(|_| Error::TooLarge(value))?
                .ok_or(Error::Zero)?,
        ))
    }
}

const MEAL_ID_MAX_VALUE: u16 = 1_200;
