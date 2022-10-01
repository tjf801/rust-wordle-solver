#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord)]
pub enum GuessTotal {
	Number(u16),
	Infinity,
}
impl GuessTotal {
	pub fn is_infinite(&self) -> bool {
		match self {
			GuessTotal::Number(_) => false,
			GuessTotal::Infinity => true,
		}
	}
}
impl From<u16> for GuessTotal {
	fn from(n: u16) -> Self {
		GuessTotal::Number(n)
	}
}
impl std::fmt::Display for GuessTotal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			GuessTotal::Number(n) => write!(f, "{}", n),
			GuessTotal::Infinity => write!(f, "∞"),
		}
	}
}
impl std::ops::Add for GuessTotal {
	type Output = Self;
	
	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(GuessTotal::Number(a), GuessTotal::Number(b)) => GuessTotal::Number(a + b),
			_ => GuessTotal::Infinity,
		}
	}
}
impl std::ops::Add<u16> for GuessTotal {
	type Output = Self;
	
	fn add(self, rhs: u16) -> Self::Output {
		match self {
			GuessTotal::Number(a) => GuessTotal::Number(a + rhs),
			_ => GuessTotal::Infinity,
		}
	}
}
impl<T> std::ops::AddAssign<T> for GuessTotal where GuessTotal: std::ops::Add<T, Output=GuessTotal> {
	fn add_assign(&mut self, rhs: T) {
		*self = *self + rhs;
	}
}
impl std::ops::Sub for GuessTotal {
	type Output = Self;
	
	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(GuessTotal::Number(a), GuessTotal::Number(b)) => GuessTotal::Number(a - b),
			(_,  GuessTotal::Infinity) => panic!("can't subtract infinity"),
			_ => GuessTotal::Infinity,
		}
	}
}
impl std::ops::Sub<u16> for GuessTotal {
	type Output = Self;
	
	fn sub(self, rhs: u16) -> Self::Output {
		match self {
			GuessTotal::Number(a) => GuessTotal::Number(a - rhs),
			_ => GuessTotal::Infinity,
		}
	}
}
impl<T> std::ops::SubAssign<T> for GuessTotal where GuessTotal: std::ops::Sub<T, Output=GuessTotal> {
	fn sub_assign(&mut self, rhs: T) {
		*self = *self - rhs;
	}
}
impl std::cmp::PartialOrd for GuessTotal {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match (self, other) {
			(GuessTotal::Number(a), GuessTotal::Number(b)) => a.partial_cmp(b),
			(GuessTotal::Infinity, GuessTotal::Infinity) => Some(std::cmp::Ordering::Equal),
			(GuessTotal::Infinity, _) => Some(std::cmp::Ordering::Greater),
			(_, GuessTotal::Infinity) => Some(std::cmp::Ordering::Less),
		}
	}
}
