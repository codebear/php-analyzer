use std::{
    iter::{Copied, Enumerate},
    ops::{RangeFrom, RangeTo},
    slice::Iter, ffi::OsString, os::unix::prelude::OsStrExt,
};

use nom::{FindSubstring, InputIter, InputLength, InputTake, Slice, UnspecializedInput};
use tree_sitter::{Point, Range};

#[derive(Clone, Debug)]
pub struct PHPDocInput<'a>(pub &'a [u8], pub Range);

impl<'a> PHPDocInput<'a> {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Slice<std::ops::Range<usize>> for PHPDocInput<'a> {
    fn slice(&self, range: std::ops::Range<usize>) -> Self {
        let mut out_range = self.1.clone();

        if range.start > 0 {
            // Adjust for what we have skipped
            out_range.start_byte += range.start;
            let skipped_buf = self.0.slice(0..range.start);
            adjust_point_based_on_buffer(&mut out_range.start_point, skipped_buf);
        } else {
            // Start-point did not move
        }
        out_range.end_point.column = out_range.start_point.column;
        out_range.end_point.row = out_range.start_point.row;

        let len = range.end - range.start;

        let buffer = self.0.slice(range);
        adjust_point_based_on_buffer(&mut out_range.end_point, buffer);

        out_range.end_byte = out_range.start_byte + len;
        // FIXME adjust end_point as well
        //         out_range.end_point.column

        Self(buffer, out_range)
    }
}

impl<'a> Slice<RangeTo<usize>> for PHPDocInput<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        let mut out_range = self.1.clone();
        out_range.end_byte = out_range.start_byte + range.end;
        let buffer = self.0.slice(range);
        out_range.end_point.row = out_range.start_point.row;
        out_range.end_point.column = out_range.end_point.column;
        adjust_point_based_on_buffer(&mut out_range.end_point, buffer);
        // FIXME adjust end_point
        Self(buffer, out_range)
    }
}

// FIXME Don't use these, this needs external libs
pub trait RFind<T> {
    fn rfind(&self, item: T) -> Option<usize>;
}

pub trait LFind<T> {
    fn lfind(&self, item: T) -> Option<usize>;
}
impl RFind<u8> for &[u8] {
    fn rfind(&self, item: u8) -> Option<usize> {
        let mut pos: usize = self.len();
        for b in self.iter().rev() {
            pos -= 1;
            if *b == item {
                return Some(pos);
            }
        }
        None
    }
}

impl LFind<u8> for &[u8] {
    fn lfind(&self, item: u8) -> Option<usize> {
        let mut pos: usize = 0;
        for b in self.iter() {
            if *b == item {
                return Some(pos);
            }
            pos += 1;
        }
        None
    }
}

pub fn fake_range(buffer: &OsString) -> Range {
    let start_point = Point { row: 0, column: 0 };
    let mut end_point = start_point.clone();
    let bytes = buffer.as_bytes();
    adjust_point_based_on_buffer(&mut end_point, bytes);

    Range {
        start_byte: 0,
        end_byte: buffer.len(),
        start_point,
        end_point,
    }
}

pub fn adjust_point_based_on_buffer(point: &mut Point, buffer: &[u8]) {
    let len = buffer.len();

    if let Some(pos) = buffer.rfind(b'\n') {
        //eprintln!("POINT ORIG: {:#?}", point);
        //eprintln!("BUFFER: {:?} has newlines...", OsStr::from_bytes(buffer));
        let last_line_len = len - pos;
        point.column = last_line_len;

        point.row += 1;

        let mut start = 0;
        let last_newline = pos;
        while let Some(pos) = (&buffer[start..last_newline]).lfind(b'\n') {
            point.row += 1;
            start += pos + 1;
        }
        //eprintln!("POINT POST: {:#?}", point);
        //panic!();
        // void
    } else {
        point.column += buffer.len();
        // void
    }
}

impl<'a> Slice<RangeFrom<usize>> for PHPDocInput<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        let mut out_range = self.1.clone();
        out_range.start_byte = self.1.start_byte + range.start;
        let skipped = &self.0[0..range.start];
        let buffer = self.0.slice(range);
        adjust_point_based_on_buffer(&mut out_range.start_point, skipped);
        // FIXME adjust start_point
        Self(buffer, out_range)
    }
}

impl<'a> InputTake for PHPDocInput<'a> {
    fn take(&self, count: usize) -> Self {
        let mut out_range = self.1.clone();
        let buffer = self.0.take(count);
        out_range.end_byte = out_range.start_byte + buffer.len();
        out_range.end_point.column = out_range.start_point.column;
        out_range.end_point.row = out_range.end_point.row;
        adjust_point_based_on_buffer(&mut out_range.end_point, buffer);
        Self(buffer, out_range)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let mut out_range_suffix = self.1.clone();
        let mut out_range_prefix = self.1.clone();
        //         todo!();
        let (buffer_suffix, buffer_prefix) = self.0.take_split(count);

        // realign range for prefix
        out_range_prefix.end_point.row = out_range_prefix.start_point.row;
        out_range_prefix.end_point.column = out_range_prefix.start_point.column;
        adjust_point_based_on_buffer(&mut out_range_prefix.end_point, buffer_prefix);
        out_range_prefix.end_byte = out_range_prefix.start_byte + buffer_prefix.len();

        // realign range for suffix
        out_range_suffix.start_point.row = out_range_prefix.end_point.row;
        out_range_suffix.start_point.column = out_range_prefix.end_point.column;
        //        adjust_point_based_on_buffer(&mut out_range_a.end_point, buffer_a);
        out_range_suffix.start_byte = out_range_prefix.end_byte;

        (
            Self(buffer_suffix, out_range_suffix),
            Self(buffer_prefix, out_range_prefix),
        )
    }
}

impl<'a> InputLength for PHPDocInput<'a> {
    fn input_len(&self) -> usize {
        self.0.len()
    }
}
/*
impl<'a, 'b> Compare<&'b [u8]> for PHPDocInput<'a> {
    fn compare(&self, t: &'b [u8]) -> nom::CompareResult {
        self.0.compare(t)
    }

    fn compare_no_case(&self, t: &'b [u8]) -> nom::CompareResult {
        self.0.compare_no_case(t)
    }
}
*/
/*
impl<'a> Compare<&'static str> for PHPDocInput<'a> {
    fn compare(&self, t: &'static str) -> nom::CompareResult {
        self.0.compare(t)
    }

    fn compare_no_case(&self, t: &'static str) -> nom::CompareResult {
        self.0.compare(t)
    }
}*/

impl<'a> UnspecializedInput for PHPDocInput<'a> {}
// InputLength + InputIter + InputTake + Clone + UnspecializedInput
/*
impl<'a> InputTakeAtPosition for PHPDocInput<'a> {
    type Item = u8;

    fn split_at_position<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.position(predicate) {
            Some(n) => {
                let (a, b) = self.0.take_split(n);
                Ok((Self(a, self.1), Self(b, self.1)))
            },
            None => todo!(),
        }

    }

    fn split_at_position1<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.position(predicate) {
            Some(0) => Err(Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => Err(Err::Incomplete(Needed::new(1))),
          }
    }

    fn split_at_position_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: nom::error::ParseError<Self>
    {
        /*let x: IResult<(&[u8], &[u8]), nom::Err<nom::error::ParseError<&[u8]> + Sized>> = self.0.split_at_position_complete(move |ch| {
            // let x:  = self.0.split_at_position_complete::<>(move |ch| {
           /* match predicate(ch) {
                true => todo!(),
                false => todo!(),
            }*/
            todo!();
        });*/
        todo!();
     /*   match self.0.split_at_position_complete::<P, nom::error::ParseError<&[u8]>>(predicate) {
            Ok((a, b)) => {
                let ra = self.1.clone();
                let rb = self.1.clone();
                Ok((Self(a, ra), Self(b, rb)))
            }
            Err(e) => {
                Err(e)
            }
        }*/
    }

    fn split_at_position1_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        todo!();
       /* match self.0.split_at_position1_complete::<P,E>(predicate, e) {
            Ok((a, b)) => {
                let ra = self.1.clone();
                let rb = self.1.clone();
                Ok((Self(a, ra), Self(b, rb)))
            },
            Err(e) => {
                todo!();
            }
        }*/
    }
}*/

impl<'a> InputIter for PHPDocInput<'a> {
    type Item = u8;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = Copied<Iter<'a, u8>>;

    fn iter_indices(&self) -> Self::Iter {
        self.0.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.0.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.0.slice_index(count)
    }
}

impl<'a> FindSubstring<&str> for PHPDocInput<'a> {
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.0.find_substring(substr)
    }
}
