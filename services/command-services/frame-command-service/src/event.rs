use std::fmt;

#[derive(Debug, Clone)]
pub enum Event {
	FrameCreated(crate::frame::create::FrameCreatedData),
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
	// StreamShouldExist = -4,
}
