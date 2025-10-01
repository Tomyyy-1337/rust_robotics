use quote::quote;
use syn::{parse_macro_input, Fields, ItemStruct, Type};

#[proc_macro_derive(PortMethods)]
pub fn module(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = input.ident.clone();

    let generics = input.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = if let Fields::Named(fields_named) = input.fields.clone() {
        fields_named.named
    } else {
        return proc_macro::TokenStream::from(quote! {
            compile_error!("The #[module] macro can only be applied to structs with named fields.");
        });
    };

    let mut receive_port_updates = Vec::new();
    for field in &fields {
        let field_name = field.ident.clone().unwrap();

        if let Type::Path(type_path) = &field.ty {
            if let Some(ident) = type_path.path.segments.last().map(|s| &s.ident) {
                if ident == "ReceivePort" || ident == "ParameterPort" {
                    receive_port_updates.push(quote! {
                        self.#field_name.update();
                    });
                }
            }
        }
    }

    let expanded = quote! {
        impl #impl_generics PortMethods for #struct_name #ty_generics
        #where_clause
        {
            fn update_ports(&mut self) {
                #(#receive_port_updates)*
            }
        }

    };

    proc_macro::TokenStream::from(expanded)
}
