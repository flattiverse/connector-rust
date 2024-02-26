use crate::{GameError, GameErrorKind};

#[inline]
pub fn check_name_or_err_32<S: AsRef<str>>(name: S) -> Result<S, GameError> {
    check_name_or_err(name, 32)
}

#[inline]
pub fn check_name_or_err_64<S: AsRef<str>>(name: S) -> Result<S, GameError> {
    check_name_or_err(name, 64)
}

pub fn check_name_or_err<S: AsRef<str>>(name: S, max_len: usize) -> Result<S, GameError> {
    if name.as_ref().len() <= max_len && check_name(name.as_ref()) {
        Ok(name)
    } else {
        Err(GameErrorKind::ParameterNotWithinSpecification.into())
    }
}

pub fn check_name(name: &str) -> bool {
    if name.len() < 2 || name.len() > 32 {
        return false;
    }

    if name.trim().len() != name.len() {
        return false;
    }

    name.chars().all(|c| {
        matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '.' | '_' | '-')
            || matches!(c as u32, 192..=214 | 216..=246 | 248..=687)
    })
}
