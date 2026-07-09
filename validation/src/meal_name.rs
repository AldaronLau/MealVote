pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Error {
    Empty,
    TooLong(usize),
    InvalidText(char),
}

/// Limited character count meal name
#[derive(
    Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize, serde::Serialize,
)]
pub struct MealName(Box<str>);

impl MealName {
    pub fn new(name: String) -> Result<Self> {
        let mut invalid_char = None;
        let char_count = name
            .chars()
            .map(|c| {
                invalid_char =
                    invalid_char.or_else(|| c.is_control().then_some(c))
            })
            .count();

        (char_count > 0).then_some(()).ok_or(Error::Empty)?;
        (char_count <= MEAL_NAME_MAX_CHARS)
            .then_some(())
            .ok_or(Error::TooLong(char_count))?;
        invalid_char
            .map(|c| Err(Error::InvalidText(c)))
            .unwrap_or(Ok(()))?;
        Ok(Self(name.into()))
    }
}

impl AsRef<str> for MealName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

const MEAL_NAME_MAX_CHARS: usize = 100;
