use std::borrow::Cow;
use std::fmt;
use std::fmt::{Error, Formatter};
// use crate::error::Error;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::error::UnmarshalError;
use crate::un::{Unmarshal, Unmarshaller};


// struct MyString(String);
//
// impl<'un> Unmarshal<'un, Event<'un>> for MyString {
//     fn unmarshal<U>(unmarshaller: &mut U) -> Result<Self, Error>
//         where
//             U: Unmarshaller<'un, Event<'un>>,
//     {
//         loop {
//             match unmarshaller.take() {
//                 Ok(Event::Start(_)) => (),
//                 Ok(Event::Text(text)) => break Ok(MyString(text.unescape().unwrap().to_string())),
//                 Ok(Event::End(_)) => (),
//                 Err(e) => break Err(e),
//                 _ => break Err(Error::default()),
//             };
//         }
//     }
// }


struct XML<'a> {
    stream: Reader<&'a [u8]>,
}

impl<'a> XML<'a> {
    fn from_str(src: &'a str) -> Self {
        XML {
            stream: Reader::from_str(src)
        }
    }
}

impl<'un> Unmarshaller<'un, Event<'un>> for XML<'un>
{
    fn take(&mut self) -> Result<Event<'un>, UnmarshalError> {
        match self.stream.read_event() {
            Ok(e) => Ok(e),
            Err(e) => ,
        }
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
    where
        T: Unmarshal<'a, Event<'a>>,
{
    let mut unmarshaller = XML::from_str(s);
    let t = T::unmarshal(&mut unmarshaller)?;
    Ok(t)
}


pub trait Visitor<'un, E> {
    type Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;

    fn visit(self, event: E) -> Result<Self::Value, Error>;
}
//
// impl<'un> Visitor<'un, Event> for XmlEventVisitor {
//     type Value = i32;
//
//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("an integer between -2^31 and 2^31")
//     }
//
//     fn visit<E>(self, event: Event) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//     {
//         match event {
//             Event::Start(_) => {}
//             Event::End(_) => {}
//             Event::Empty(_) => {}
//             Event::Text(_) => {}
//             Event::Comment(_) => {}
//             Event::CData(_) => {}
//             Event::Decl(_) => {}
//             Event::PI(_) => {}
//             Event::DocType(_) => {}
//             Event::Eof => {}
//         }
//         todo!()
//     }
// }

impl<'un> Unmarshal<'un, Event<'un>> for Cow<'un, str> {
    fn unmarshal<U>(unmarshaller: &mut U) -> Result<Self, Error> where U: Unmarshaller<'un, Event<'un>>, Self: Sized {
        loop {
            match unmarshaller.take() {
                Ok(Event::Text(text)) => break Ok(text
                    .unescape()
                    .unwrap()
                ),
                _ => break Err(Error::default()),
            }
        }
    }
}

fn def<T>(e:T) -> Error {
    return Error::default();
}

fn parse_i8(src: Cow<str>) -> Result<i8, Error> {
    return src.parse::<i8>().map_err(def);
}

impl<'un> Unmarshal<'un, Event<'un>> for i8 {
    fn unmarshal<U>(unmarshaller: &mut U) -> Result<Self, Error> where U: Unmarshaller<'un, Event<'un>>, Self: Sized {
        loop {
            match unmarshaller.take() {
                Ok(Event::Text(text)) => return match text.unescape() {
                    Ok(cow) => parse_i8(cow),
                    Err(e) => break Err(Error::default()), // TODO ?
                },
                _ => break Err(Error::default()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use super::*;

    #[test]
    fn it_unmarshalls() {
        let src = "<root>value</root>";
        let dst: MyString = from_str(&src).unwrap();
        assert_eq!(dst.0, "value");
    }

    #[test]
    fn unmarshal_primitive_cow_str() {
        let src = "value";
        let dst: Cow<str> = from_str(&src).unwrap();
        assert_eq!(dst, "value");
    }

    #[test]
    fn unmarshal_primitive_cow_i8() {
        let src = "-127";
        let dst: i8 = from_str(&src).unwrap();
        assert_eq!(dst, -127);
    }
}