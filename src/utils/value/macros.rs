#![allow(unused_macros)]

macro_rules! forward_val_val_binop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl $imp<$res> for $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $res) -> $res {
                // forward to val-ref
                $imp::$method(self, &other)
            }
        }
    };
}

macro_rules! forward_ref_val_binop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl $imp<$res> for &$res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $res) -> $res {
                // forward to ref-ref
                $imp::$method(self, &other)
            }
        }
    };
}

macro_rules! forward_val_ref_binop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl $imp<&$res> for $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: &$res) -> $res {
                // forward to ref-ref
                $imp::$method(&self, other)
            }
        }
    };
}

macro_rules! forward_scalar_val_val_binop_to_ref_val {
    (impl $imp:ident<$scalar:ty> for $res:ty, $method:ident) => {
        impl $imp<$scalar> for $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $scalar) -> $res {
                $imp::$method(&self, other)
            }
        }

        impl $imp<$res> for $scalar {
            type Output = $res;

            #[inline]
            fn $method(self, other: $res) -> $res {
                $imp::$method(self, &other)
            }
        }
    };
}

macro_rules! forward_scalar_ref_val_binop_to_ref_ref {
    (impl $imp:ident<$scalar:ty> for $res:ty, $method:ident) => {
        impl $imp<$scalar> for &$res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $scalar) -> $res {
                $imp::$method(self, &other)
            }
        }

        impl $imp<$res> for &$scalar {
            type Output = $res;

            #[inline]
            fn $method(self, other: $res) -> $res {
                $imp::$method(self, &other)
            }
        }
    };
}

macro_rules! forward_scalar_val_ref_binop_to_ref_val {
    (impl $imp:ident<$scalar:ty> for $res:ty, $method:ident) => {
        impl $imp<&$scalar> for $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: &$scalar) -> $res {
                $imp::$method(&self, other)
            }
        }

        impl $imp<&$res> for $scalar {
            type Output = $res;

            #[inline]
            fn $method(self, other: &$res) -> $res {
                $imp::$method(&self, other)
            }
        }
    };
}

macro_rules! forward_scalar_ref_ref_binop_commutative {
    (impl $imp:ident<$scalar:ty> for $res:ty, $method:ident) => {
        impl $imp<&$res> for &$scalar {
            type Output = $res;

            #[inline]
            fn $method(self, other: &$res) -> $res {
                $imp::$method(other, self)
            }
        }
    };
}

macro_rules! forward_into_bigint_scalar_ref_val_binop_to_ref_ref {
    (impl $imp:ident<$scalar:ty> for $res:ty, $method:ident) => {
        impl $imp<$scalar> for &$res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $scalar) -> $res {
                $imp::$method(self, &BigInt::from(other))
            }
        }

        impl $imp<&$res> for $scalar {
            type Output = $res;

            #[inline]
            fn $method(self, other: &$res) -> $res {
                $imp::$method(&BigInt::from(self), other)
            }
        }
    };
}
