use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, space0},
    error::{ErrorKind, ParseError},
    AsChar, Err, IResult, InputLength, InputTake, InputTakeAtPosition, Parser,
};

///
/// Parse a list of items, separated by a separator, with at least two items
/// copy of [nom::multi::separated_list1] from nom, but requires at least two items
///
pub fn separated_list2<I, O, O2, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    move |mut i: I| {
        let mut res = Vec::new();

        // Parse the first element
        match f.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }

        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(Err::Error(_)) if res.len() > 1 => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(Err::Error(E::from_error_kind(i1, ErrorKind::SeparatedList)));
                    }

                    match f.parse(i1.clone()) {
                        Err(Err::Error(_)) if res.len() > 1 => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}

///
/// custom space-parser with toggle-able multiline support
///
pub(super) fn ourspace0<I, E>(multiline: bool) -> impl FnMut(I) -> IResult<I, I, E>
where
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    E: ParseError<I>,
{
    move |input| {
        if multiline {
            multispace0(input)
        } else {
            space0(input)
        }
    }
}

pub(super) fn space0_and_separator<I, S>(
    separator: S,
    multiline: bool,
) -> impl Fn(I) -> IResult<I, ()>
where
    I: nom::Compare<S>,
    I: InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    I: InputTake,
    S: InputLength + Clone + Copy,
{
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(separator)(input)?;
        Ok((input, ()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{bytes::complete::tag, IResult};

    #[test]
    fn test_list_parser_1() {
        let input = "abc";

        let result: IResult<&'static str, Vec<&'static str>> =
            separated_list2(tag(","), tag("abc"))(input);

        let Err(error) = result else {
            unreachable!();
        };

        assert_eq!(
            Err::Error(ParseError::from_error_kind("", ErrorKind::Tag)),
            error
        );
    }

    #[test]
    fn test_list_parser_2() {
        let input = "abc,abc";

        let result: IResult<&'static str, Vec<&'static str>> =
            separated_list2(tag(","), tag("abc"))(input);

        let Ok((remaining, parsed)) = result else {
            unreachable!();
        };

        assert!(remaining.is_empty());
        assert_eq!(vec!["abc", "abc"], parsed);
    }

    #[test]
    fn test_list_parser_3() {
        let input = "abc,abc,abc";

        let result: IResult<&'static str, Vec<&'static str>> =
            separated_list2(tag(","), tag("abc"))(input);

        let Ok((remaining, parsed)) = result else {
            unreachable!();
        };

        assert!(remaining.is_empty());
        assert_eq!(vec!["abc", "abc", "abc"], parsed);
    }
}
