use std::fmt;

#[derive(Debug, Clone)]
pub enum Event {
	Login(crate::user::login::UserLoggedInData),
	UserRegistered(crate::user::register::UserRegisteredData),
}

impl fmt::Display for Event {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

pub enum ExpectedVersion {
	// StreamShouldExistButEmpty = 0,
	StreamShouldNotExist = -1,
	// WriteShouldAlwaysSucceed = -2,
	StreamShouldExist = -4,
}
