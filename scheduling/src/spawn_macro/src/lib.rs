use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, ItemFn, Stmt, Pat, Local, Block,
    visit_mut::VisitMut,
};

struct SpawnInjector;

impl VisitMut for SpawnInjector {
    fn visit_block_mut(&mut self, block: &mut Block) {
        syn::visit_mut::visit_block_mut(self, block);
        let mut injections = Vec::new();

        for stmt in &block.stmts {
            if let Stmt::Local(Local { pat, init: Some(init), .. }) = stmt {
                if let Pat::Ident(ident) = pat {
                    let var_name = &ident.ident;
                    let init_expr = &init.expr.to_token_stream().to_string();

                    if init_expr.contains("ModuleBuilder") {
                        injections.push(syn::parse_quote! {
                            builder.add_module(#var_name);
                        });
                    } else if init_expr.contains("GroupBuilder") {
                        injections.push(syn::parse_quote! {
                            builder.add_group(#var_name);
                        });
                    }
                }
            }
        }

        block.stmts.extend(injections);
    }
}

/// Automatically injects `builder.add_module(...)` or `builder.add_group(...)`
/// calls for each `ModuleBuilder` or `GroupBuilder` variable declared in the
/// annotated function.
#[proc_macro_attribute]
pub fn spawns(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let mut injector = SpawnInjector;
    injector.visit_item_fn_mut(&mut input);

    TokenStream::from(quote! { #input })
}
