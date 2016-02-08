use std::slice;

#[inline]
pub fn option_as_slice<T>(opt: &Option<T>) -> &[T]
{
    match *opt {
        Some(ref val) => unsafe{ slice::from_raw_parts(val, 1) },
        None => &[],
    }
}
