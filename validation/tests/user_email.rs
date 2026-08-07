use std::assert_matches;

use validation::user_email::{Error, UserEmail};

#[test]
fn user_email() {
    assert_matches!(UserEmail::new("example@gmail.com".to_string()), Ok(_));
    assert_matches!(UserEmail::new("Example@gmail-two.com".to_string()), Ok(_));
    assert_matches!(
        UserEmail::new("exa.mple@gmail.two.com".to_string()),
        Ok(_),
    );
    assert_matches!(UserEmail::new("e+label@2.io".to_string()), Ok(_));
    assert_matches!(UserEmail::new("2x-@a--a.2b2.io".to_string()), Ok(_));
    assert_matches!(UserEmail::new("例子@💥.💩".to_string()), Ok(_));
    assert_matches!(UserEmail::new("a".repeat(255)), Err(Error::TooLong(_)));
    assert_matches!(
        UserEmail::new("a@\\.us".to_string()),
        Err(Error::InvalidText('\\')),
    );
    assert_matches!(
        UserEmail::new(String::new()),
        Err(Error::MissingSeparator),
    );
    assert_matches!(
        UserEmail::new("mail@me@example.com".to_string()),
        Err(Error::ExtraSeparator(1)),
    );
    assert_matches!(
        UserEmail::new("@gmail.com".to_string()),
        Err(Error::LocalEmpty),
    );
    assert_matches!(
        UserEmail::new("example@".to_string()),
        Err(Error::DomainEmpty),
    );
    assert_matches!(
        UserEmail::new("me@localhost".to_string()),
        Err(Error::DomainTldMissing),
    );
    assert_matches!(
        UserEmail::new(format!("me@{}.com", "a".repeat(64))),
        Err(Error::DomainLabelTooLong(64)),
    );
    assert_matches!(
        UserEmail::new("yote@yeet.i".to_string()),
        Err(Error::DomainTldTooShort('i')),
    );
    assert_matches!(
        UserEmail::new(".me@gmail.com".to_string()),
        Err(Error::LocalLabelEmpty),
    );
    assert_matches!(
        UserEmail::new("me.@gmail.com".to_string()),
        Err(Error::LocalLabelEmpty),
    );
    assert_matches!(
        UserEmail::new("me@-example.com".to_string()),
        Err(Error::DomainLabelHyphenAtStart),
    );
    assert_matches!(
        UserEmail::new("me@example-.com".to_string()),
        Err(Error::DomainLabelHyphenAtEnd),
    );
    assert_matches!(
        UserEmail::new("me@example.123".to_string()),
        Err(Error::DomainTldInvalidText('1')),
    );
    assert_matches!(
        UserEmail::new("m..e@example.com".to_string()),
        Err(Error::LocalLabelEmpty),
    );
    assert_matches!(
        UserEmail::new("me@example..com".to_string()),
        Err(Error::DomainLabelEmpty),
    );
    assert_matches!(
        UserEmail::new("me@.example.com".to_string()),
        Err(Error::DomainLabelEmpty),
    );
    assert_matches!(
        UserEmail::new("me@example.com.".to_string()),
        Err(Error::DomainLabelEmpty),
    );
    assert_matches!(
        UserEmail::new(format!("{}@example.com", "a".repeat(65))),
        Err(Error::LocalTooLong(65)),
    );
}
