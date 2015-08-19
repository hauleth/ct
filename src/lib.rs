use std::ops::*;
use std::mem::size_of;

#[derive(Clone, Copy, Debug)]
pub struct Ct<T>(T);

macro_rules! implement {
    (Eq for $($t:ty),*) => {
        $(
            impl Ct<$t> {
                fn const_ne(a: $t, b: $t) -> $t {
                    (a.wrapping_sub(b) | b.wrapping_sub(a)) >> (size_of::<$t>() * 8 - 1)
                }
                fn const_eq(a: $t, b: $t) -> $t { 1 ^ Self::const_ne(a, b) }
            }

            impl PartialEq for Ct<$t> {
                fn eq(&self, other: &Self) -> bool { Self::const_eq(self.0, other.0) == 1 }
                fn ne(&self, other: &Self) -> bool { Self::const_ne(self.0, other.0) == 1 }
            }
            impl Eq for Ct<$t> {}

            impl PartialEq<$t> for Ct<$t> {
                fn eq(&self, other: &$t) -> bool { Self::const_eq(self.0, *other) == 1 }
                fn ne(&self, other: &$t) -> bool { Self::const_ne(self.0, *other) == 1 }
            }
         )*
    };
    (binary $op:ident for Ct<$t:ident> with $fun:ident) => {
        impl<$t> $op for Ct<$t> where $t: $op {
            type Output = Ct<$t::Output>;

            fn $fun(self, other: Self) -> Self::Output { Ct($t::$fun(self.0, other.0)) }
        }
    };
    (unary $op:ident for Ct<$t:ident> with $fun:ident) => {
        impl<$t> $op for Ct<$t> where $t: $op {
            type Output = Ct<$t::Output>;

            fn $fun(self) -> Self::Output { Ct($t::$fun(self.0)) }
        }
    };
}

implement!(Eq for u8, u16, u32, u64, usize, i8, i16, i32, i64);

implement!(binary Add for Ct<T> with add);
implement!(binary Sub for Ct<T> with sub);
implement!(binary Mul for Ct<T> with mul);
implement!(binary Div for Ct<T> with div);

implement!(binary BitAnd for Ct<T> with bitand);
implement!(binary BitOr  for Ct<T> with bitor);
implement!(binary BitXor for Ct<T> with bitxor);

implement!(unary Not for Ct<T> with not);
implement!(unary Neg for Ct<T> with neg);

#[cfg(test)]
mod tests {
    use super::Ct;

    #[test]
    fn test_partial_eq() {
        let a = Ct(0u32);
        let b = Ct(1u32);

        assert_eq!(a, a);
        assert_eq!(b, b);
        assert!(a != b);
    }
}
