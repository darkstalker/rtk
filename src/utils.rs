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
