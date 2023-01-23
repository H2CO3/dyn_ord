//! Traits for dynamically-typed equality comparison and ordering.

use core::cmp::Ordering;
use core::any::Any;

/// A trait for comparing dynamically-typed values for equality.
///
/// After coercing your values to a trait object of type `DynEq`,
/// you can directly compare references (and smart pointers) to
/// instances via the usual `==` or `!=` operators.
///
/// Trait objects created from different concrete underlying types
/// are considered not equal. Trait objects created from the same
/// underlying concrete type are compared using `PartialEq`.
///
/// ```
/// # use dyn_ord::DynEq;
/// let x: &dyn DynEq = &42;
/// let y: &dyn DynEq = &String::from("qux");
/// let z: &dyn DynEq = &String::from("baz");
///
/// assert!(*x == *x);
/// assert!(*x != *y);
/// assert!(*x != *z);
///
/// assert!(*y != *x);
/// assert!(*y == *y);
/// assert!(*y != *z);
///
/// assert!(*z != *x);
/// assert!(*z != *y);
/// assert!(*z == *z);
/// ```
pub trait DynEq: Any {
    #[doc(hidden)]
    fn as_any(&self) -> &dyn Any;

    #[doc(hidden)]
    fn as_dyn_eq_helper(&self) -> &dyn DynEqHelper;

    #[doc(hidden)]
    fn dyn_eq(&self, other: &dyn DynEqHelper) -> bool;
}

#[doc(hidden)]
pub trait DynEqHelper {
    fn static_eq(&self, other: &dyn DynEq) -> bool;
}

impl<T: Any + PartialEq> DynEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_dyn_eq_helper(&self) -> &dyn DynEqHelper {
        self
    }

    fn dyn_eq(&self, other: &dyn DynEqHelper) -> bool {
        other.static_eq(self)
    }
}

#[doc(hidden)]
impl<T: Any + PartialEq> DynEqHelper for T {
    fn static_eq(&self, other: &dyn DynEq) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }
}

impl PartialEq for dyn DynEq {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_dyn_eq_helper())
    }
}

/// A trait for comparing dynamically-typed values for ordering.
///
/// After coercing your values to a trait object of type `DynOrd`,
/// you can directly compare references (and smart pointers) to
/// instances via the usual `<`, `<=`, `>`, `>=`, `==` or `!=`.
///
/// Trait objects created from different concrete underlying types
/// are considered not comparable. Trait objects created from the
/// same underlying concrete type are compared using `PartialOrd`.
///
/// ```
/// # use core::cmp::Ordering;
/// # use std::rc::Rc;
/// # use dyn_ord::DynOrd;
/// let x: Rc<dyn DynOrd> = Rc::new(1337);
/// let y: Rc<dyn DynOrd> = Rc::new(String::from("qux"));
/// let z: Rc<dyn DynOrd> = Rc::new(String::from("baz"));
///
/// assert_eq!(y.partial_cmp(&z), Some(Ordering::Greater));
/// assert_eq!(z.partial_cmp(&y), Some(Ordering::Less));
/// assert_eq!(y.partial_cmp(&y), Some(Ordering::Equal));
/// assert_eq!(x.partial_cmp(&y), None);
/// ```
pub trait DynOrd: DynEq {
    #[doc(hidden)]
    fn as_dyn_ord_helper(&self) -> &dyn DynOrdHelper;

    #[doc(hidden)]
    fn dyn_ord(&self, other: &dyn DynOrdHelper) -> Option<Ordering>;
}

#[doc(hidden)]
pub trait DynOrdHelper {
    fn static_ord(&self, other: &dyn DynOrd) -> Option<Ordering>;
}

impl<T: Any + PartialOrd> DynOrd for T {
    fn as_dyn_ord_helper(&self) -> &dyn DynOrdHelper {
        self
    }

    fn dyn_ord(&self, other: &dyn DynOrdHelper) -> Option<Ordering> {
        other.static_ord(self)
    }
}

#[doc(hidden)]
impl<T: Any + PartialOrd> DynOrdHelper for T {
    fn static_ord(&self, other: &dyn DynOrd) -> Option<Ordering> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            other.partial_cmp(self) // note the reversed order
        } else {
            None
        }
    }
}

impl PartialEq for dyn DynOrd {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for dyn DynOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dyn_ord(other.as_dyn_ord_helper())
    }
}
