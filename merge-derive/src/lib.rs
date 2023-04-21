use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident};
use darling::{ast, util, FromDeriveInput, FromField};

#[derive(Debug, FromField)]
#[darling(attributes(merge_field))]
struct MergeField {
    ident: Option<Ident>,
    strategy: Option<String>,
    #[darling(default)]
    skip: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(merge_field), supports(struct_any))]
struct MergeTarget {
    ident: Ident,
    data: ast::Data<util::Ignored, MergeField>,
}

type FieldGenFn = fn(MergeField, proc_macro2::TokenStream) -> Option<proc_macro2::TokenStream>;
type TraitGenFn = fn(Ident, Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream;

fn gen_field_lines(
    fields: Option<ast::Fields<MergeField>>,
    gen_field: FieldGenFn,
) -> Vec<proc_macro2::TokenStream> {
    let mut res = vec![];
    if let Some(fields) = fields {
        for (idx, field) in fields.into_iter().enumerate() {
            let field_token = match &field.ident {
                Some(field_name) => quote! { #field_name },
                None => {
                    let idx = syn::Index::from(idx);
                    quote! { #idx }
                }
            };
            if let Some(field) = gen_field(field, field_token) {
                res.push(field);
            }
        }
    }
    res
}

fn do_derive(
    input: TokenStream,
    gen_field: FieldGenFn,
    gen_trait: TraitGenFn,
) -> TokenStream {
    let parsed = parse_macro_input!(input);
    match MergeTarget::from_derive_input(&parsed) {
        Err(e) => e.write_errors(),
        Ok(target) => {
            let target_ident = target.ident;
            let fields = gen_field_lines(target.data.take_struct(), gen_field);
            gen_trait(target_ident, fields)
        }
    }.into()
}

#[proc_macro_derive(MergeMut, attributes(merge_field))]
pub fn derive_merge_mut(input: TokenStream) -> TokenStream {
    do_derive(
        input,
        |field, field_token| {
            match field.strategy {
                _ if field.skip => None,
                None => Some(quote! {
                    self.#field_token.merge_mut(&other.#field_token)?;
                }),
                Some(strategy) => {
                    let strategy_fn = Ident::new(&strategy, Span::call_site());
                    Some(quote! {
                        #strategy_fn(&mut self.#field_token, &other.#field_token); // TODO: bug here?
                    })
                }
            }
        },
        |target, fields| {
            quote! {
                impl MergeMut for #target {
                    fn merge_mut(&mut self, other: &Self) -> Result<(), Box<dyn std::error::Error>> {
                        #(#fields)*
                        Ok(())
                    }
                }
            }
        })
}

#[proc_macro_derive(Merge, attributes(merge_field))]
pub fn derive_merge(input: TokenStream) -> TokenStream {
    do_derive(
        input,
        |field, field_token| {
            match field.strategy {
                _ if field.skip => Some(quote! {
                    #field_token: self.#field_token.clone(),
                }),
                None => Some(quote! {
                    #field_token: self.#field_token.merge(&other.#field_token)?,
                }),
                Some(strategy) => {
                    let strategy_fn = Ident::new(&strategy, Span::call_site());
                    Some(quote! {
                        #field_token: #strategy_fn(&self.#field_token, &other.#field_token)?,
                    })
                }
            }
        },
        |target, fields| {
            quote! {
                impl Merge for #target {
                    fn merge(&self, other: &Self) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
                        Ok(Self {
                            #(#fields)*
                        })
                    }
                }
            }
        },
    )
}
