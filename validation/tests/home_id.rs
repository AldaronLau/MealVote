use std::assert_matches;

use validation::home_id::{Error, HomeId};

#[test]
fn home_id() {
    assert_matches!(HomeId::new(0), Err(Error::Zero));
    assert_matches!(HomeId::new(1), Ok(_));
    assert_matches!(HomeId::new(1_024), Ok(_));
    assert_matches!(HomeId::new(1_025), Err(Error::TooLarge(1_025)));
}
