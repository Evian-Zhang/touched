use darling::{FromDeriveInput, FromField, ast};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};

#[derive(Debug, FromField)]
#[darling(attributes(touched))]
struct TouchableFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    skip: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_any))]
struct TouchableReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), TouchableFieldReceiver>,
}

impl ToTokens for TouchableReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TouchableReceiver {
            ident,
            generics,
            data,
        } = self;

        let (imp, ty, wher) = generics.split_for_impl();
        let Some(struc) = data.as_ref().take_struct() else {
            tokens
                .extend(darling::Error::custom("Touchable only works with struct").write_errors());
            return;
        };
        let fields = struc.fields;

        let field_list = fields
            .into_iter()
            .enumerate()
            .filter_map(|(index, TouchableFieldReceiver { ident, ty, skip })| {
                if *skip {
                    return None;
                }
                // This works with named or indexed fields, so we'll fall back to the index so we can
                // write the output as a key-value pair.
                let field_ident = ident.as_ref().map(|v| quote!(#v)).unwrap_or_else(|| {
                    let i = syn::Index::from(index);
                    quote!(#i)
                });

                Some(quote! {
                    touched::touching::<#ty>(&self.#field_ident);
                })
            })
            .collect::<Vec<_>>();

        tokens.extend(quote! {
            impl #imp touched::Touchable for #ident #ty #wher {
                fn touch(&self) {
                    #(#field_list)*
                }
            }
        });
    }
}

/// Derive macro for [`Touchable`][Touchable-trait] trait.
///
/// You should make sure each fields of the struct are primitive types,
/// or implement [`Touchable`][Touchable-trait] trait. Otherwise you can
/// use `#[touched(skip)]` to skip touching that field.
///
/// # Usage
///
/// ```rust
/// # struct OtherData;
/// use touched::Touchable;
///
/// #[derive(Touchable)]
/// pub struct MyData {
///     pub a: u8,
///     pub b: [u8; 64],
///     #[touched(skip)] // <-- OtherData does not implement Touchable
///     pub c: OtherData
/// }
/// ```
///
/// [Touchable-trait]: https://docs.rs/touched/latest/touched/trait.Touchable.html
#[proc_macro_derive(Touchable, attributes(touched))]
pub fn my_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let touchable = match TouchableReceiver::from_derive_input(&input) {
        Ok(tokens) => tokens,
        Err(err) => {
            return proc_macro::TokenStream::from(err.write_errors());
        }
    };
    proc_macro::TokenStream::from(quote! { #touchable })
}
