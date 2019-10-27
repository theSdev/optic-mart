use crate::user::UserRegisteredData;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Event {
	UserRegistered(UserRegisteredData),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum ExpectedVersion {
	StreamShouldExistButEmpty = 0,
	StreamShouldNotExist = -1,
	WriteShouldAlwaysSucceed = -2,
	StreamShouldExist = -4,
}