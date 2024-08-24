use std::{
    cell::Cell,
    ffi::{OsStr, OsString},
    os::unix::ffi::OsStrExt,
    sync::atomic::{AtomicBool, Ordering},
};

use nom::{bytes::complete::take_while1, IResult};

use crate::symbols::Name;

///
/// GENERAL TYPE TOKENS
///

///
/// Generic type-word, like `array` or `int`, or a classname
/// Result is assign a [Name] type, currently wrapping an OsString.
///
pub(super) fn simple_type_name(input: &[u8]) -> IResult<&[u8], Name> {
    let second = AtomicBool::new(false);
    let (input, result) = take_while1(move |x: u8| {
        let sec = second.load(Ordering::Relaxed);
        second.store(true, Ordering::Relaxed);
        x == b'_'
            || if sec {
                // We allow dash in type names, to allow for `class-string` and similar
                x.is_ascii_alphanumeric() || x == b'-'
            } else {
                x.is_ascii_alphabetic()
            }
    })(input)?;

    Ok((input, Name::from(result)))
}

pub(super) fn php_var_name(input: &[u8]) -> IResult<&[u8], OsString> {
    #[derive(Clone, Copy)]
    enum State {
        First,
        Dollar,
        Alpha,
    }

    let state = Cell::new(State::First);

    let (input, result) = take_while1(move |x: u8| match state.get() {
        State::First if x == b'$' => {
            state.set(State::Dollar);
            true
        }
        State::First => false,
        State::Dollar if x.is_ascii_alphabetic() => {
            state.set(State::Alpha);
            true
        }
        State::Dollar => false,
        State::Alpha => x.is_ascii_alphanumeric(),
    })(input)?;
    Ok((input, OsStr::from_bytes(result).into()))
}
