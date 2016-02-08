use std::slice;

#[inline]
pub fn option_as_slice<T>(opt: &Option<T>) -> &[T]
{
    match *opt {
        Some(ref val) => unsafe{ slice::from_raw_parts(val, 1) },
        None => &[],
    }
}

macro_rules! debug_fmt
{
    ($tyname:ident, $($field:ident),*) => {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            f.debug_struct(stringify!($tyname))
                $(
                .field(stringify!($field), &self.$field)
                )*
                .finish()
        }
    }
}
