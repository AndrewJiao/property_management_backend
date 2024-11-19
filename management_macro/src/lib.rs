use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod inner;


#[proc_macro_derive(AutoOperation)]
pub fn derive_save_change_after_process(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    inner::do_derive_auto_process_before_save(input)
}

