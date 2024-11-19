use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::DeriveInput;

pub(crate) fn do_derive_auto_process_before_save(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let has_update_time = if let Struct(data) = &input.data {
        data.fields.iter()
            .any(|f| f.ident.as_ref().map_or(false, |ident| ident == "update_time"))
    } else { false };

    let has_create_time = if let Struct(data) = &input.data {
        data.fields.iter()
            .any(|f| f.ident.as_ref().map_or(false, |ident| ident == "create_time"))
    } else { false };


    let create_time_fn = if has_create_time {
        quote! {
            fn create_time(mut self) -> Self {
                self.create_time = Some(chrono::Utc::now().naive_utc());
                self.update_time = Some(chrono::Utc::now().naive_utc());
                self
            }
        }
    } else {
        quote! {
            fn create_time(mut self) -> Self {
                self
            }
        }
    };

    let update_time_fn = if has_update_time {
        quote! {
            fn update_time(mut self) -> Self {
                self.update_time = Some(chrono::Utc::now().naive_utc());
                self
            }
        }
    } else {
        quote! {
            fn update_time(mut self) -> Self {
                self
            }
        }
    };

    let stream = quote! {
        impl common::db_config::auto_trait::AutoOperation for #name<'_> {
            #create_time_fn
            #update_time_fn
        }
    };

    TokenStream::from(stream)
}