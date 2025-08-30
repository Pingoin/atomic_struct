use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn atomic_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let struct_vis = &input.vis;

    let struct_attrs = &input.attrs;
    let mut atomic_fields = Vec::new();
    let mut getters_setters = Vec::new();
    let mut constructor_params = Vec::new();
    let mut constructor_inits = Vec::new();

    if let Fields::Named(fields) = &input.fields {
        for field in &fields.named {
            let fname = field.ident.as_ref().unwrap();
            let fty = &field.ty;
            let attrs = &field.attrs;
            let vis = &field.vis;
            let doc_attrs: Vec<_> = field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("doc"))
                .collect();

            // neues Feld mit Arc<Mutex>
            atomic_fields.push(quote! {
                #(#attrs)*
                #fname: atomic_struct_core::AtomicMember<#fty>
            });

            // Konstruktor-Parameter
            constructor_params.push(quote! {
                #fname: #fty
            });

            // Konstruktor-Init
            constructor_inits.push(quote! {
                #fname: atomic_struct_core::AtomicMember::new(#fname)
            });

            // Getter
            let get_name = format_ident!("get_{}", fname);
            getters_setters.push(quote! {
                #(#doc_attrs)*
                #vis async fn #get_name(&self) -> #fty {
                    self.#fname.get().await
                }
            });

            // Setter
            let set_name = format_ident!("set_{}", fname);
            getters_setters.push(quote! {
                #(#doc_attrs)*
                #vis async fn #set_name(&self, new_val: #fty) {
                    self.#fname.set(new_val).await
                }
            });
        }
    }

    let expanded = quote! {
        #(#struct_attrs)*
        #struct_vis struct #struct_name {
            #(#atomic_fields),*
        }

        impl #struct_name {
            #struct_vis fn new(#(#constructor_params),*) -> Self {
                Self {
                    #(#constructor_inits),*
                }
            }

            #(#getters_setters)*
        }
    };

    expanded.into()
}
