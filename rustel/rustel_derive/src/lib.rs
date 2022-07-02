use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_event(&ast)
}

fn impl_event(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        #[automatically_derived]
        impl Event for #name {
            fn event_type(&self) -> EventType {
                EventType::#name
            }

            fn name(&self) -> &str {
                stringify!{ #name }
            }

            fn in_category(&self, category: EventCategory) -> bool {
                (self.category & category).bits() != 0
            }
        }
    };
    gen.into()
}
