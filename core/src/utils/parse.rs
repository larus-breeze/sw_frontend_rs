use crate::CoreError;

pub trait ParseSlice {
    fn from_slice(slice: &[u8]) -> Result<Self, CoreError>
    where
        Self: Sized;
}

impl ParseSlice for u32 {
    fn from_slice(slice: &[u8]) -> Result<Self, CoreError> {
        if slice.len() == 0 {
            return Err(CoreError::ParseError);
        }

        let mut u: u32 = 0;
        for c in slice {
            let add = match c {
                b'0'..=b'9' => *c as u32 - 48,
                _ => return Err(CoreError::ParseError),
            };
            if u > (u32::max_value() - add) / 10 {
                return Err(CoreError::ParseError);
            }
            u = u * 10 + add;
        }
        Ok(u)
    }
}

impl ParseSlice for i32 {
    fn from_slice(slice: &[u8]) -> Result<Self, CoreError> {
        if slice.len() == 0 {
            return Err(CoreError::ParseError);
        }

        let (is_negative, slice) = if slice[0] == b'-' {
            (true, &slice[1..])
        } else {
            (false, slice)
        };

        let mut u: i32 = 0;
        for c in slice {
            let add = match c {
                b'0'..=b'9' => *c as i32 - 48,
                _ => return Err(CoreError::ParseError),
            };
            if u > (i32::max_value() - add) / 10 {
                return Err(CoreError::ParseError);
            }
            u = u * 10 + add;
        }
        if is_negative {
            Ok(-u)
        } else {
            Ok(u)
        }
    }
}

impl ParseSlice for f32 {
    fn from_slice(slice: &[u8]) -> Result<Self, CoreError> {
        if slice.len() == 0 {
            return Err(CoreError::ParseError);
        }

        let (is_negative, slice) = if slice[0] == b'-' {
            (true, &slice[1..])
        } else {
            (false, slice)
        };

        let mut divisor = 1i32;
        let mut divident = 0i32;
        let mut before_comma = 0i32;
        let mut first_part = true;
        let mut u = 0i32;

        for c in slice {
            match c {
                b'0'..=b'9' => {
                    let add = *c as i32 - 48;
                    if u > (i32::max_value() - add) / 10 {
                        return Err(CoreError::ParseError);
                    }
                    u = u * 10 + add;

                    if first_part {
                        before_comma = u;
                    } else {
                        divident = u;
                        divisor *= 10;
                    }
                }
                b'.' => {
                    first_part = false;
                    u = 0
                }
                _ => return Err(CoreError::ParseError),
            }
        }

        let r = before_comma as f32 + divident as f32 / divisor as f32;
        if is_negative {
            Ok(-r)
        } else {
            Ok(r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParseSlice;

    #[test]
    fn parse_u32() {
        assert_eq!(u32::from_slice(b"0"), Ok(0));
        assert_eq!(u32::from_slice(b"4711"), Ok(4711));
        assert_eq!(u32::from_slice(b"4294967295"), Ok(u32::max_value()));
        assert_eq!(
            u32::from_slice(b"4294967296"),
            Err(crate::CoreError::ParseError)
        );
        assert_eq!(u32::from_slice(b"a"), Err(crate::CoreError::ParseError));
    }

    #[test]
    fn parse_i32() {
        assert_eq!(i32::from_slice(b"0"), Ok(0));
        assert_eq!(i32::from_slice(b"4711"), Ok(4711));
        assert_eq!(i32::from_slice(b"-4711"), Ok(-4711));
        assert_eq!(i32::from_slice(b"2147483647"), Ok(i32::max_value()));
        assert_eq!(i32::from_slice(b"-2147483647"), Ok(-i32::max_value()));
        assert_eq!(
            i32::from_slice(b"4294967296"),
            Err(crate::CoreError::ParseError)
        );
        assert_eq!(i32::from_slice(b"a"), Err(crate::CoreError::ParseError));
    }

    #[test]
    fn parse_f32() {
        assert_eq!(f32::from_slice(b"0"), Ok(0.0));
        assert_eq!(f32::from_slice(b"4711"), Ok(4711.0));
        assert_eq!(f32::from_slice(b"3.14"), Ok(3.14));
        assert_eq!(f32::from_slice(b"-3.14"), Ok(-3.14));
        assert_eq!(f32::from_slice(b"1.23456"), Ok(1.23456));
        assert_eq!(f32::from_slice(b"-1.23456"), Ok(-1.23456));
        assert_eq!(
            f32::from_slice(b"2147483647.0"),
            Ok(i32::max_value() as f32)
        );
        assert_eq!(
            f32::from_slice(b"-2147483647.0"),
            Ok(-i32::max_value() as f32)
        );
        assert_eq!(
            f32::from_slice(b"4294967296.0"),
            Err(crate::CoreError::ParseError)
        );
        assert_eq!(f32::from_slice(b"a"), Err(crate::CoreError::ParseError));
    }
}
