extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(MaxSerializedLen)]
pub fn impl_max_serialized_len(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // TODO
    let len = 5;

    let token_stream2 = quote! {
        impl #impl_generics MaxSerializedLen for #name #ty_generics #where_clause {
            const MAX_SERIALIZED_LEN: usize = #len;
        }
    };

    token_stream2.into()
}
