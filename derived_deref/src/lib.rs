extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct, Ident, Generics, Field, Fields, Index, Type, punctuated::Punctuated, token::Comma};
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(Deref, attributes(deref))]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let name = item_struct.ident;
    let generics = item_struct.generics;

    match extract_field_parameters(item_struct.fields, "Deref") {
        Ok((field_name, field_type, is_mut_reference)) => impl_deref(name, generics, field_name, Some(field_type), is_mut_reference),
        Err(error) => error,
    }
}

#[proc_macro_derive(DerefMut, attributes(deref))]
pub fn derive_deref_mut(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let name = item_struct.ident;
    let generics = item_struct.generics;

    match extract_field_parameters(item_struct.fields, "DerefMut") {
        Ok((field_name, _, is_mut_reference)) => impl_deref(name, generics, field_name, None, is_mut_reference),
        Err(error) => error,
    }
}

fn get_field(fields: Punctuated<Field, Comma>) -> Result<(usize, Field), TokenStream> {
    let attribute_name = "deref";
    let error = || quote! { compile_error!("`#[deref]` is required for one field"); }.into();
    
    let has_one_field = fields.len() == 1;
    let mut fields_iter = fields.into_iter().fuse().enumerate();
    
    if has_one_field {
        fields_iter.next().ok_or_else(error)
    } else {
        let mut fields_iter = fields_iter.filter(|(_, field)| {
            field.attrs.iter().any(|attribute| {
                attribute.meta
                    .require_path_only()
                    .is_ok_and(|path| path.is_ident(attribute_name))
            })
        });

        fields_iter.next().filter(|_| {
            fields_iter
                .next()
                .is_none()
        })
        .ok_or_else(error)
    }
}

fn extract_field_parameters(fields: Fields, trait_name: &str) -> Result<(TokenStream2, Type, Option<bool>), TokenStream> {
    match fields {
        Fields::Named(fields) => {
            let (_, field) = get_field(fields.named)?;
            let field_name = field.ident.unwrap();
            let (field_type, is_mut_reference) = match field.ty {
                Type::Reference(reference_type) => (*reference_type.elem, Some(reference_type.mutability.is_some())),
                field_type => (field_type, None),
            };

            Ok((quote! { #field_name }, field_type, is_mut_reference))
        },
        Fields::Unnamed(fields) => {
            let (field_index, field) = get_field(fields.unnamed)?;
            let field_index = Index::from(field_index);
            let (field_type, is_mut_reference) = match field.ty {
                Type::Reference(reference_type) => (*reference_type.elem, Some(reference_type.mutability.is_some())),
                field_type => (field_type, None),
            };

            Ok((quote! { #field_index }, field_type, is_mut_reference))
        },
        Fields::Unit => {
            let error = &format!("unable to implement `{}` trait for struct of no fields", trait_name)[..];

            Err(quote! { compile_error!(#error); }.into())
        }
    }
}

fn impl_deref(
    struct_name: Ident,
    struct_generics: Generics,
    field_name: TokenStream2,
    field_type: Option<Type>,
    is_mut_reference: Option<bool>,
) -> TokenStream 
{
    let (impl_generics, type_generics, where_clause) = struct_generics.split_for_impl();

    match field_type {
        Some(field_type) => {
            let reference = is_mut_reference.map_or_else(|| Some(quote!(&)), |_| None);
            
            quote! {
                impl #impl_generics core::ops::Deref for #struct_name #type_generics #where_clause {
                    type Target = #field_type;

                    fn deref(&self) -> &Self::Target {
                        #reference self.#field_name
                    }
                }
            }
        },
        None => {
            let reference = match is_mut_reference {
                Some(true) => None,
                Some(false) => return quote! { compile_error!("`#[deref_target]` is unable to be of an immutable reference"); }.into(),
                None => Some(quote!(&mut)),
            };
            
            quote! {
                impl #impl_generics core::ops::DerefMut for #struct_name #type_generics #where_clause {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        #reference self.#field_name
                    }
                }
            }
        },
    }
    .into()
}
