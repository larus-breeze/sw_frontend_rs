#[macro_export]
#[allow(unused_macros)]
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! basic_ops {
    ($($t:ty)*) => ($(

        impl Add for $t {
            type Output = $t;

            #[inline]
            fn add(self, other: $t) -> $t { Self(self.0 + other.0) }
        }
        $crate::forward_ref_binop! { impl Add, add for $t, $t }

        impl AddAssign for $t {

            #[inline]
            fn add_assign(&mut self, other: $t) { self.0 += other.0; }
        }

        impl Sub for $t {
            type Output = $t;

            #[inline]
            fn sub(self, other: $t) -> $t { Self(self.0 - other.0) }
        }
        $crate::forward_ref_binop! { impl Sub, sub for $t, $t }

        impl SubAssign for $t {

            #[inline]
            fn sub_assign(&mut self, other: $t) { self.0 -= other.0; }
        }

        impl Mul<Float> for $t {
            type Output = $t;

            #[inline]
            fn mul(self, other: Float) -> $t { Self(self.0 * other) }
        }

        impl Div<Float> for $t {
            type Output = $t;

            #[inline]
            fn div(self, other: Float) -> $t { Self(self.0 / other) }
        }

        impl Div for $t {
            type Output = Float;

            #[inline]
            fn div(self, other: $t) -> Float { self.0 / other.0 }
        }

        impl Neg for $t {
            type Output = $t;

            #[inline]
            fn neg(self) -> Self::Output { Self(-self.0) }
        }

        impl PartialOrd for $t {

            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> { Some(self.cmp(other)) }
        }

        impl PartialEq for $t {

            #[inline]
            fn eq(&self, other: &$t) -> bool { self.0 == other.0 }
        }

        impl Ord for $t {

            #[inline]
            fn cmp(&self, other: &$t) -> Ordering {
                if self.0 < other.0 { Ordering::Less }
                else if self.0 == other.0 { Ordering::Equal }
                else { Ordering::Greater }
            }
        }

        impl Eq for $t {}
    )*)
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! assert_float_eq {
    ($a:expr, $b:expr) => {
        assert!(($a - $b).abs() < $a.abs() * 0.001, "{} =! {}", $a, $b)
    };
}

#[macro_export]
macro_rules! tformat {
    ($cap:expr, $($tt:tt)*) => {{
        let mut s = heapless::String::<$cap>::new();
        #[allow(unreachable_code)]
        match tfmt::uwrite!(&mut s, $($tt)*) {
            Ok(_) => Ok(s),
            Err(_) => Err($crate::CoreError::ConversionError),
        }
    }};
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! spsc_queue {
    ($queue_type:ty) => {{
        let queue: *mut $queue_type = {
            static mut Q: $queue_type = Queue::new();
            &raw mut Q
        };
        unsafe { &mut *queue }.split()
    }};
}
