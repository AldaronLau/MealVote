use std::assert_matches;

use validation::meal_name::{Error, MealName};

#[test]
fn meal_name() {
    assert_matches!(MealName::new("Changas".to_string()), Ok(_));
    assert_matches!(MealName::new("asdf".repeat(50)), Err(Error::TooLong(_)));
    assert_matches!(MealName::new("asdf".repeat(256)), Err(Error::TooLarge));
    assert_matches!(
        MealName::new("\0".to_string()),
        Err(Error::InvalidText('\0')),
    );
    assert_matches!(MealName::new(String::new()), Err(Error::Empty));
}
