pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Error {
    /// Longer than 254 octets
    TooLong(usize),
    /// Control character, whitespace, or invalid ASCII punctuation
    InvalidText(char),
    /// No '@' in the email address
    MissingSeparator,
    /// More than one '@' in the email address
    ExtraSeparator(usize),
    /// The domain is empty (address ends with '@')
    DomainEmpty,
    /// A domain label is empty
    DomainLabelEmpty,
    /// A domain label is longer than 63 octets
    DomainLabelTooLong(usize),
    /// Invalid hyphen at start of domain label
    DomainLabelHyphenAtStart,
    /// Invalid hyphen at end of domain label
    DomainLabelHyphenAtEnd,
    /// Local (non-public) email address
    DomainTldMissing,
    /// Invalid single-character top-level domain
    DomainTldTooShort(char),
    /// Non alphabetic ASCII character in TLD
    DomainTldInvalidText(char),
    /// The recipient is empty (address starts with '@')
    LocalEmpty,
    /// A local label is empty
    LocalLabelEmpty,
    /// Local part of the email address is longer than 64 octets
    LocalTooLong(usize),
}

/// User email
///
/// This validation is meant to catch only real public emails, not the full set
/// of RFC-compliant email addresses.
///
///  - Less than or equal to 254 octets
///  - Greater than or equal to 6 octets `a@a.aa`
///    - Domain is alphanumeric (plus '-', labels split by '.'), each label max
///      63 octets
///    - TLD is at least 2 octets (1 octet TLD might be usable in the future,
///      but currently not)
///    - TLD is either alphabetic or non-ascii non-control-codes non-whitespace
///    - Single '@'
///  - Local-part is at most 64 octets (no additional limit for domain)
///  - '.' only occurs between two alphanumeric / special characters
///
/// Special characters (including non-ascii non-control-codes non-whitespace):
///
/// ```text
/// ! ? & # $ % ' ` + - * / = ~ ^ _ { } |
/// ```
///
/// Notably, this rejects:
///
///  - Single-octet TLD email address
///  - Local area network email address
///  - Emails containing comments `()`
///  - Emails containing IPs `[192.168.0.1]`
///  - Emails containing quotes `"me"@gmail.com`
#[derive(
    Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize, serde::Serialize,
)]
#[repr(transparent)]
pub struct UserEmail(Box<str>);

impl UserEmail {
    pub fn new(name: String) -> Result<Self> {
        // Check length is doesn't exceed max before going through each char
        {
            let octet_len = name.len();

            (octet_len <= USER_EMAIL_MAX_LEN)
                .then_some(())
                .ok_or(Error::TooLong(octet_len))?;
        }

        let mut invalid_char = None;

        for c in name.chars() {
            let is_invalid = if c.is_ascii() {
                let is_valid = c.is_ascii_alphanumeric()
                    || "!?&#$%'`+-*/=~^_{}|@.".contains(c);

                !is_valid
            } else {
                c.is_whitespace() || c.is_control()
            };

            invalid_char = invalid_char.or_else(|| is_invalid.then_some(c));
        }

        let mut parts = name.split('@');
        let local_part = parts.next().unwrap();
        let domain_part = parts.next().ok_or(Error::MissingSeparator)?;
        let extra = parts.count();

        (extra == 0)
            .then_some(())
            .ok_or(Error::ExtraSeparator(extra))?;
        (local_part.len() <= 64)
            .then_some(())
            .ok_or(Error::LocalTooLong(local_part.len()))?;
        (!local_part.is_empty())
            .then_some(())
            .ok_or(Error::LocalEmpty)?;
        (!local_part.split('.').any(|x| x.is_empty()))
            .then_some(())
            .ok_or(Error::LocalLabelEmpty)?;

        let mut non_empty_label = true;
        let mut too_large_label = None;
        let mut tld = None;
        let mut no_hyphen_at_start = true;
        let mut no_hyphen_at_end = true;
        let labels = domain_part
            .rsplit('.')
            .map(|label| {
                if label.is_empty() {
                    non_empty_label = false;
                }

                if label.len() > 63 {
                    too_large_label = Some(label.len());
                }

                if label.starts_with('-') {
                    no_hyphen_at_start = false;
                }

                if label.ends_with('-') {
                    no_hyphen_at_end = false;
                }

                tld = tld.or(Some(label));
            })
            .count();
        let tld = (labels != 1 || non_empty_label)
            .then_some(tld)
            .flatten()
            .ok_or(Error::DomainEmpty)?;
        let mut invalid_tld_char = None;

        for c in tld.chars() {
            invalid_tld_char = invalid_tld_char
                .or((c.is_ascii() && !c.is_ascii_alphabetic()).then_some(c));
        }

        invalid_tld_char
            .map(|c| Err(Error::DomainTldInvalidText(c)))
            .unwrap_or(Ok(()))?;
        non_empty_label
            .then_some(())
            .ok_or(Error::DomainLabelEmpty)?;
        no_hyphen_at_start
            .then_some(())
            .ok_or(Error::DomainLabelHyphenAtStart)?;
        no_hyphen_at_end
            .then_some(())
            .ok_or(Error::DomainLabelHyphenAtEnd)?;
        too_large_label
            .map(|l| Err(Error::DomainLabelTooLong(l)))
            .unwrap_or(Ok(()))?;
        (labels >= 2).then_some(()).ok_or(Error::DomainTldMissing)?;
        (tld.len() >= 2)
            .then_some(())
            .ok_or(Error::DomainTldTooShort(tld.chars().nth(0).unwrap()))?;
        invalid_char
            .map(|c| Err(Error::InvalidText(c)))
            .unwrap_or(Ok(()))?;
        Ok(Self(name.into()))
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

const USER_EMAIL_MAX_LEN: usize = 254;
