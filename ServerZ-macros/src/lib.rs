use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, DeriveInput, Expr, ExprLit, Lit, LitStr, Meta, parse_macro_input};

/// Derive macro that generates `to_class()` and `Display` for enums with `#[class("...")]` attributes.
///
/// Each variant must have a `#[class = "..."]` attribute specifying its Tailwind class.
///
/// # Example
///
/// ```ignore
/// #[derive(TailwindClass)]
/// enum DividerColor {
///     #[default]
///     #[class = "divider-neutral"]
///     Neutral,
///     #[class = "divider-primary"]
///     Primary,
/// }
/// ```
///
/// Generates:
/// - `const fn to_class(&self) -> &'static str` returning the literal class string
/// - `impl Display` that delegates to `to_class()`
#[proc_macro_derive(TailwindClass, attributes(class))]
pub fn derive_tailwind_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("`TailwindClass` can only be derived for enums"),
    };

    // Extract class strings from #[class = "..."] attributes
    let class_strings: Vec<_> = variants
        .iter()
        .map(|variant| {
            let class_value = variant
                .attrs
                .iter()
                .filter_map(|attr| {
                    if attr.path().is_ident("class") {
                        if let Meta::NameValue(meta) = &attr.meta {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(lit_str),
                                ..
                            }) = &meta.value
                            {
                                return Some(lit_str.value());
                            }
                        }
                    }
                    None
                })
                .next();

            class_value.unwrap_or_else(|| {
                panic!(
                    "Variant `{}::{}` is missing `#[class = \"...\"]` attribute",
                    name, variant.ident
                )
            })
        })
        .collect();

    let variant_ids: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    // Build match arms with literal class strings
    let match_arms = variant_ids
        .iter()
        .zip(class_strings.iter())
        .map(|(variant, class)| {
            let class_lit = LitStr::new(class, Span::call_site());
            quote! {
                Self::#variant => #class_lit,
            }
        });

    let expanded = quote! {
        impl #name {
            /// Returns the Tailwind class string for this variant.
            pub const fn to_class(&self) -> &'static str {
                match self {
                    #(#match_arms)*
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_class())
            }
        }
    };

    TokenStream::from(expanded)
}
