use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(IB2CMetaSignals)]
pub fn derive_ib2c_meta_signals(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let generics = input.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut methods = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                if let Some(field_ident) = &field.ident {
                    let name = field_ident.to_string();
                    match name.as_str() {
                        "stimulation" => {
                            methods.push(quote! {
                                fn stimulation(&mut self) -> &mut ReceivePort<MetaSignal> {
                                    &mut self.stimulation
                                }
                            });
                        }
                        "inhibition" => {
                            methods.push(quote! {
                                fn inhibition(&mut self) -> &mut ReceivePort<MetaSignal> {
                                    &mut self.inhibition
                                }
                            });
                        }
                        "activity" => {
                            methods.push(quote! {
                                fn activity(&mut self) -> &mut SendPort<MetaSignal> {
                                    &mut self.activity
                                }
                            });
                        }
                        "target_rating" => {
                            methods.push(quote! {
                                fn target_rating(&mut self) -> &mut SendPort<MetaSignal> {
                                    &mut self.target_rating
                                }
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl #impl_generics IB2CMetaSignals for #ident #ty_generics
        #where_clause
        {
            #(#methods)*
        }
    };

    expanded.into()
}