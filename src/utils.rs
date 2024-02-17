use crate::{GameError, GameErrorKind};

pub fn check_name_or_err(name: impl Into<String>) -> Result<String, GameError> {
    let name = name.into();
    if check_name(&name) {
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
