pub trait DigitF {
	fn digit_count(&self) -> u32;
}

impl DigitF for i64 {
	fn digit_count(&self) -> u32 {
		if self == &0 { return 1; }
		let mut dc: u32 = 0;
		if self.is_negative() {
			loop {
				if i128::pow(10, dc)*-1 < *self as i128 {
					break;
				}
				else {
					dc += 1;
				}
			}
		} else {
			loop {
				if i128::pow(10, dc) > *self as i128 {
					break;
				}
				else {
					dc += 1;
				}
			}
		}
		dc
	}
}

impl DigitF for i32 {
	fn digit_count(&self) -> u32 {
		if self == &0 { return 1; }
		let mut dc: u32 = 0;
		if self.is_negative() {
			loop {
				if i64::pow(10, dc)*-1 < *self as i64 {
					break;
				}
				else {
					dc += 1;
				}
			}
		} else {
			loop {
				if i64::pow(10, dc) > *self as i64 {
					break;
				}
				else {
					dc += 1;
				}
			}
		}
		dc
	}
}

impl DigitF for i16 {
	fn digit_count(&self) -> u32 {
		if self == &0 { return 1; }
		let mut dc: u32 = 0;
		if self.is_negative() {
			loop {
				if i32::pow(10, dc)*-1 < *self as i32 {
					break;
				}
				else {
					dc += 1;
				}
			}
		} else {
			loop {
				if i32::pow(10, dc) > *self as i32 {
					break;
				}
				else {
					dc += 1;
				}
			}
		}
		dc
	}
}

impl DigitF for i8 {
	fn digit_count(&self) -> u32 {
		if self == &0 { return 1; }
		let mut dc: u32 = 0;
		if self.is_negative() {
			loop {
				if i16::pow(10, dc)*-1 < *self as i16 {
					break;
				}
				else {
					dc += 1;
				}
			}
		} else {
			loop {
				if i16::pow(10, dc) > *self as i16 {
					break;
				}
				else {
					dc += 1;
				}
			}
		}
		dc
	}
}

fn main() {}
