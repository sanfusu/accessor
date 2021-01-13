extern crate proc_macro;

use proc_macro::TokenStream;

/// # example
/// ```no_compile
/// #[derive(Field)]
/// enum A {
///     Field1(u32),
///     Field2(u8),
///     Field3(u16),
/// }
/// ```
#[proc_macro_derive(Field, attributes(mutable))]
pub fn field(_item: TokenStream) -> TokenStream {
    todo! {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
