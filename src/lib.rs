use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn compile_fail(attr: TokenStream, stream: TokenStream) -> TokenStream {
    let mut args = parse_macro_input!(attr as AttributeArgs);
    let off = match args.pop() {
        Some(NestedMeta::Meta(Meta::NameValue(m))) if m.path.is_ident("off") => {
            if let Lit::Bool(bool) = m.lit {
                // TODO: ensure args is now empty
                bool.value
            } else {
                // TODO: return an error indicating that attributes are malformed
                false
            }
        }
        Some(_) => {
            // TODO: return an error indicating that attributes are malformed
            false
        }
        None => false,
    };

    let fun = parse_macro_input!(stream as ItemFn);
    let mut tokens = proc_macro2::TokenStream::new();
    fun.block.to_tokens(&mut tokens);
    let output = if off {
        quote! {
            #fun
        }
    } else {
        let name = fun.sig.ident;
        let code = tokens.to_string();
        let doc = format!(
            "
        ```compile_fail
        {}
        ```
            ",
            code
        );
        quote! {
            #[test]
            #[doc = #doc]
            fn #name() {}
        }
    };

    output.into()
}
