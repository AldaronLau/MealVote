use std::assert_matches;

use validation::meal_id::{Error, MealId};

#[test]
fn meal_id() {
    assert_matches!(MealId::new(0), Err(Error::Zero));
    assert_matches!(MealId::new(1), Ok(_));
    assert_matches!(MealId::new(1_200), Ok(_));
    assert_matches!(MealId::new(1_201), Err(Error::TooLarge(1_201)));
}
