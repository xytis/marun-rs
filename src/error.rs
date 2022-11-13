use std::error;

pub(crate) enum UnmarshalError {
    Parse(dyn error::Error + 'static),
    UnexpectedToken(dyn error::Error + 'static),
    Other(dyn error::Error + 'static),
    TODO,
}

