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

/// Derive macro that turns a struct's JSON schema (via `schemars::JsonSchema`)
/// into a `<Struct>Validation` enum with one variant per field.
///
/// The struct must also derive `schemars::JsonSchema`. The generated enum has:
///
/// - `name(&self) -> &'static str`: the schema property name of the field
/// - `ALL: &'static [Self]`: every variant, in declaration order
/// - `attrs(&self, cx: &Cx) -> Attributes`: HTML input attributes derived
///   from the field's schema, ready to pass as `attrs:` to the `input`
///   component
///
/// Attributes emitted: `id`, `name`, `type`, `required`, `minlength`,
/// `maxlength`, `pattern`, `min`, and `max`. `type` is inferred from the
/// schema's `format` and `type` (`email`, `url`, `date`, `time`,
/// `datetime-local`, `number`, otherwise `text`).
///
/// Fields honour `#[serde(rename = "...")]`, `#[schemars(rename = "...")]`
/// and are omitted for `#[serde(skip)]`.
///
/// # Example
///
/// ```ignore
/// #[derive(JsonSchema, FormValidation)]
/// struct OwnerForm {
///     #[schemars(length(min = 3, max = 30))]
///     username: String,
/// }
///
/// input(attrs: OwnerFormValidation::Username.attrs(cx))
/// ```
#[proc_macro_derive(FormValidation)]
pub fn derive_form_validation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_form_validation(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_form_validation(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    use syn::ext::IdentExt;

    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "`FormValidation` does not support generic structs",
        ));
    }
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            syn::Fields::Named(named) => &named.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "`FormValidation` requires a struct with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "`FormValidation` can only be derived for structs",
            ));
        }
    };

    let struct_name = &input.ident;
    let vis = &input.vis;
    let enum_name = syn::Ident::new(&format!("{struct_name}Validation"), struct_name.span());

    let mut variants = Vec::new();
    let mut names = Vec::new();
    for field in fields {
        let ident = field.ident.as_ref().expect("named field");
        let mut property = ident.unraw().to_string();
        let mut skip = false;
        for attr in &field.attrs {
            if attr.path().is_ident("serde") || attr.path().is_ident("schemars") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("rename") {
                        let lit: LitStr = meta.value()?.parse()?;
                        property = lit.value();
                    } else if meta.path.is_ident("skip") {
                        skip = true;
                    } else if meta.input.peek(syn::Token![=]) {
                        let _: Expr = meta.value()?.parse()?;
                    } else if meta.input.peek(syn::token::Paren) {
                        let content;
                        syn::parenthesized!(content in meta.input);
                        let _: proc_macro2::TokenStream = content.parse()?;
                    }
                    Ok(())
                })?;
            }
        }
        if skip {
            continue;
        }
        let variant = syn::Ident::new(&to_pascal_case(&ident.unraw().to_string()), ident.span());
        variants.push(variant);
        names.push(LitStr::new(&property, ident.span()));
    }

    Ok(quote! {
        /// One variant per field of the source struct, generated by
        /// `#[derive(FormValidation)]`.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #vis enum #enum_name {
            #(#variants,)*
        }

        impl #enum_name {
            /// Every field, in declaration order.
            pub const ALL: &'static [Self] = &[#(Self::#variants,)*];

            /// The field's property name in the JSON schema.
            pub const fn name(&self) -> &'static str {
                match self {
                    #(Self::#variants => #names,)*
                }
            }

            /// HTML input attributes derived from the field's JSON schema.
            pub fn attrs(&self, cx: &::topcoat::context::Cx) -> ::topcoat::view::Attributes {
                static SCHEMA: ::std::sync::OnceLock<::schemars::Schema> =
                    ::std::sync::OnceLock::new();
                let schema = SCHEMA
                    .get_or_init(|| ::schemars::schema_for!(#struct_name))
                    .as_value();

                let name = self.name();
                let mut attrs = ::topcoat::view::Attributes::new();
                attrs.insert(cx, "id", name.to_owned());
                attrs.insert(cx, "name", name.to_owned());

                let required = schema
                    .get("required")
                    .and_then(|r| r.as_array())
                    .is_some_and(|r| r.iter().any(|v| v.as_str() == Some(name)));
                if required {
                    attrs.insert(cx, "required", true);
                }

                let Some(prop) = schema.get("properties").and_then(|p| p.get(name)) else {
                    attrs.insert(cx, "type", "text");
                    return attrs;
                };

                // `Option<T>` fields use `"type": ["string", "null"]`.
                let ty = match prop.get("type") {
                    Some(t) if t.is_string() => t.as_str(),
                    Some(t) => t
                        .as_array()
                        .and_then(|a| a.iter().filter_map(|v| v.as_str()).find(|s| *s != "null")),
                    None => None,
                };
                let format = prop.get("format").and_then(|f| f.as_str());
                let input_type = match (format, ty) {
                    (Some("email"), _) => "email",
                    (Some("uri" | "url"), _) => "url",
                    (Some("date"), _) => "date",
                    (Some("time"), _) => "time",
                    (Some("date-time"), _) => "datetime-local",
                    (_, Some("integer" | "number")) => "number",
                    _ => "text",
                };
                attrs.insert(cx, "type", input_type);

                if let Some(v) = prop.get("minLength") {
                    attrs.insert(cx, "minlength", v.to_string());
                }
                if let Some(v) = prop.get("maxLength") {
                    attrs.insert(cx, "maxlength", v.to_string());
                }
                if let Some(v) = prop.get("pattern").and_then(|v| v.as_str()) {
                    attrs.insert(cx, "pattern", v.to_owned());
                }
                if let Some(v) = prop.get("minimum") {
                    attrs.insert(cx, "min", v.to_string());
                }
                if let Some(v) = prop.get("maximum") {
                    attrs.insert(cx, "max", v.to_string());
                }
                attrs
            }
        }
    })
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars
                .next()
                .map(|c| c.to_uppercase().chain(chars).collect::<String>())
                .unwrap_or_default()
        })
        .collect()
}
