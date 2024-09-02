use crate::network::InvalidArgumentKind;
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
        Err(GameErrorKind::InvalidArgument {
            reason: InvalidArgumentKind::NameConstraint,
            parameter: "name".to_string(),
        }
        .into())
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

pub fn check_message_or_err<S: AsRef<str>>(message: S) -> Result<S, GameError> {
    if check_message(message.as_ref()) {
        Ok(message)
    } else {
        Err(GameErrorKind::InvalidArgument {
            reason: InvalidArgumentKind::ChatConstraint,
            parameter: "chat_message".to_string(),
        }
        .into())
    }
}

pub fn check_message(message: &str) -> bool {
    if message.len() < 1 || message.len() > 512 {
        false
    } else if message.trim().len() != message.len() {
        false
    } else {
        message.chars().all(|c| {
            matches!(
                c,
                '€' | '‚'
                    | '„'
                    | '…'
                    | '‰'
                    | '‹'
                    | '›'
                    | '™'
                    | '•'
                    | '¢'
                    | '£'
                    | '¡'
                    | '¤'
                    | '¥'
                    | '©'
                    | '®'
                    | '±'
                    | '²'
                    | '³'
                    | 'µ'
                    | '¿'
                    | '«'
                    | '»'
            ) || matches!(c as u32, 32..=126 | 192..=214 | 216..=246 | 248..=687)
        })
    }
}
