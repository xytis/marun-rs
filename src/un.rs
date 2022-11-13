use crate::error::UnmarshalError;

pub trait Unmarshal<'un, E> {
    fn unmarshal<U>(unmarshaller: &mut U) -> Result<Self, UnmarshalError>
        where
            U: Unmarshaller<'un, E>,
            Self: Sized;
}

pub trait Unmarshaller<'un, E> {
    fn take(&mut self) -> Result<E, UnmarshalError>;
}