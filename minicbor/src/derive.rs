#[cfg(feature = "alloc")]
#[doc(hidden)]
#[inline]
pub fn cow_borrowed<'a, T>(t: &'a T) -> alloc::borrow::Cow<'a, T>
where
    T: alloc::borrow::ToOwned + ?Sized,
{
    alloc::borrow::Cow::Borrowed(t)
}

#[cfg(not(feature = "alloc"))]
#[doc(hidden)]
#[inline]
pub fn cow_borrowed<T>(t: T) -> T {
    t
}

#[cfg(feature = "alloc")]
#[doc(hidden)]
#[inline]
pub fn tag_mismatch(tag: crate::data::Tag, expected: u64) -> crate::decode::Error {
    crate::decode::Error::tag_mismatch(tag)
        .with_message(alloc::format!("expected tag {}", expected))
}

#[cfg(not(feature = "alloc"))]
#[doc(hidden)]
#[inline]
pub fn tag_mismatch(tag: crate::data::Tag, _expected: u64) -> crate::decode::Error {
    crate::decode::Error::tag_mismatch(tag)
}
