use darling::FromField;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::{env, fs, path::PathBuf};

use syn::{
    Data, DeriveInput, Expr, ExprLit, Lit, LitStr, Meta, Token, parse::Parse, parse::ParseStream, parse_macro_input,
};

struct GithubReleaseAssetBytesInput {
    release_url: LitStr,
    asset_name: Option<LitStr>,
}

impl Parse for GithubReleaseAssetBytesInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let release_url = input.parse()?;
        let asset_name = if input.is_empty() {
            None
        } else {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        };
        if !input.is_empty() {
            return Err(input.error("expected a release URL and optional asset filename"));
        }
        Ok(Self { release_url, asset_name })
    }
}

#[derive(serde::Deserialize)]
struct GithubRelease {
    assets: Vec<GithubAsset>,
}

#[derive(serde::Deserialize)]
struct GithubAsset {
    name: String,
    size: u64,
}

const ARCH_ALT_MAPPINGS: &[(&str, &str)] = &[("arm", "arm"), ("aarch64", "arm64"), ("x86_64", "x64")];

fn arch_alt_for(target_arch: &str) -> Option<&'static str> {
    ARCH_ALT_MAPPINGS.iter().find_map(|&(target, alternate)| (target == target_arch).then_some(alternate))
}

/// Expands to the alternate asset architecture name for the current target.
#[proc_macro]
pub fn arch_alt(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        return syn::Error::new(Span::call_site(), "`arch_alt!` takes no arguments").to_compile_error().into();
    }

    let arms = ARCH_ALT_MAPPINGS.iter().map(|&(target_arch, alternate)| {
        let target_arch = LitStr::new(target_arch, Span::call_site());
        let alternate = LitStr::new(alternate, Span::call_site());
        quote! {
            #[cfg(target_arch = #target_arch)]
            { #alternate }
        }
    });
    let supported_targets = ARCH_ALT_MAPPINGS.iter().map(|&(target_arch, _)| {
        let target_arch = LitStr::new(target_arch, Span::call_site());
        quote!(target_arch = #target_arch)
    });

    quote! {
        {
            #(#arms)*
            #[cfg(not(any(#(#supported_targets),*)))]
            {
                compile_error!("unsupported target architecture for `arch_alt!`");
                ""
            }
        }
    }
    .into()
}

/// Expands to the byte size of a named GitHub release asset, or the total
/// byte size of all assets when no filename is provided.
///
/// ```ignore
/// github_release_asset_bytes!(
///     "https://github.com/OWNER/REPOSITORY/releases/tag/TAG",
///     "asset-name.zip"
/// );
/// ```
///
/// The release API is queried while the crate is compiled. Results are cached
/// under `target/server-z-macros/github-release-asset-bytes`, keyed by the
/// exact URL. Set `GITHUB_TOKEN` in the build environment to raise the GitHub
/// API rate limit.
#[proc_macro]
pub fn github_release_asset_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GithubReleaseAssetBytesInput);
    let url = input.release_url.value();
    let asset_name = input.asset_name.map(|name| name.value());

    let result = match asset_name.as_deref() {
        Some(template) if template.contains("{OS}") || template.contains("{ARCH_ALT}") => {
            expand_asset_template(&url, template)
        }
        _ => github_release_asset_bytes_impl(&url, asset_name.as_deref()).map(|size| quote!(#size)),
    };

    match result {
        Ok(tokens) => tokens.into(),
        Err(error) => syn::Error::new(Span::call_site(), error).to_compile_error().into(),
    }
}

fn expand_asset_template(url: &str, template: &str) -> Result<proc_macro2::TokenStream, String> {
    const TARGETS: &[(&str, &str)] = &[
        ("macos", "x86_64"),
        ("macos", "aarch64"),
        ("linux", "x86_64"),
        ("linux", "aarch64"),
        ("linux", "arm"),
        ("windows", "x86_64"),
        ("windows", "aarch64"),
    ];

    let mut sizes = Vec::with_capacity(TARGETS.len());
    let mut missing = false;
    for &(os, target_arch) in TARGETS {
        let arch_alt = arch_alt_for(target_arch).expect("target architecture must have an alternate mapping");
        let asset_name = template.replace("{OS}", os).replace("{ARCH_ALT}", arch_alt);
        if read_cached_size(url, Some(&asset_name)).is_none() {
            missing = true;
        }
        sizes.push(asset_name);
    }

    let release = if missing { Some(fetch_release(url)?) } else { None };
    let mut arms = Vec::with_capacity(TARGETS.len());
    for ((os, arch), asset_name) in TARGETS.iter().zip(sizes) {
        let arch_alt = arch_alt_for(arch).expect("target architecture must have an alternate mapping");
        let size = match read_cached_size(url, Some(&asset_name)) {
            Some(size) => size,
            None => {
                let release = release.as_ref().expect("release fetched for missing asset");
                let resolved_name = template.replace("{OS}", os).replace("{ARCH_ALT}", arch_alt);
                let asset = release
                    .assets
                    .iter()
                    .find(|asset| asset.name == resolved_name)
                    .ok_or_else(|| format!("GitHub release does not contain an asset named `{resolved_name}`"))?;
                write_cached_size(url, Some(&asset_name), u128::from(asset.size));
                u128::from(asset.size)
            }
        };
        let literal = proc_macro2::Literal::u128_unsuffixed(size);
        let os = syn::LitStr::new(os, Span::call_site());
        let arch = syn::LitStr::new(arch, Span::call_site());
        arms.push(quote! {
            #[cfg(all(target_os = #os, target_arch = #arch))]
            { #literal }
        });
    }

    Ok(quote! {
        {
            #(#arms)*
            #[cfg(not(any(
                all(target_os = "macos", target_arch = "x86_64"),
                all(target_os = "macos", target_arch = "aarch64"),
                all(target_os = "linux", target_arch = "x86_64"),
                all(target_os = "linux", target_arch = "aarch64"),
                all(target_os = "linux", target_arch = "arm"),
                all(target_os = "windows", target_arch = "x86_64"),
                all(target_os = "windows", target_arch = "aarch64"),
            )))]
            { compile_error!("unsupported target for GitHub asset template"); 0u128 }
        }
    })
}

fn github_release_asset_bytes_impl(url: &str, asset_name: Option<&str>) -> Result<u128, String> {
    if let Some(size) = read_cached_size(url, asset_name) {
        return Ok(size);
    }

    let release = fetch_release(url)?;
    let assets = if let Some(asset_name) = asset_name {
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("GitHub release does not contain an asset named `{asset_name}`"))?;
        std::slice::from_ref(asset)
    } else {
        release.assets.as_slice()
    };
    let size = assets.iter().try_fold(0_u128, |total, asset| {
        total.checked_add(u128::from(asset.size)).ok_or_else(|| "GitHub asset sizes overflowed u128".to_owned())
    })?;

    write_cached_size(url, asset_name, size);
    Ok(size)
}

fn fetch_release(url: &str) -> Result<GithubRelease, String> {
    let path = url
        .strip_prefix("https://github.com/")
        .ok_or_else(|| "expected an https://github.com release URL".to_owned())?
        .trim_end_matches('/');
    let mut parts = path.splitn(4, '/');
    let owner = parts.next().unwrap_or_default();
    let repository = parts.next().unwrap_or_default();
    let releases = parts.next().unwrap_or_default();
    let tag = parts.next().unwrap_or_default();

    if owner.is_empty() || repository.is_empty() || releases != "releases" || !tag.starts_with("tag/") {
        return Err("expected https://github.com/OWNER/REPOSITORY/releases/tag/TAG".to_owned());
    }
    let tag = &tag[4..];
    if tag.is_empty() || tag.contains('?') || tag.contains('#') {
        return Err("the GitHub release URL must include a tag".to_owned());
    }

    let api_url = format!("https://api.github.com/repos/{owner}/{repository}/releases/tags/{tag}");
    let client = reqwest::blocking::Client::builder()
        .user_agent("server-z-build-script")
        .build()
        .map_err(|error| format!("could not create GitHub API client: {error}"))?;
    let mut request = client.get(api_url).header("Accept", "application/vnd.github+json");
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        request = request.bearer_auth(token);
    }

    let response = request.send().map_err(|error| format!("could not query GitHub release: {error}"))?;
    let response =
        response.error_for_status().map_err(|error| format!("GitHub release API returned an error: {error}"))?;
    response.json().map_err(|error| format!("could not parse GitHub release response: {error}"))
}

fn cache_file(url: &str, asset_name: Option<&str>) -> Option<PathBuf> {
    let target_dir = env::var_os("SERVERZ_MACRO_CACHE_DIR")
        .map(PathBuf::from)
        .or_else(|| env::var_os("CARGO_TARGET_DIR").map(PathBuf::from))
        .or_else(|| env::var_os("CARGO_MANIFEST_DIR").map(|dir| PathBuf::from(dir).join("target")))?;

    Some(
        target_dir
            .join("server-z-macros")
            .join("github-release-asset-bytes")
            .join(hex_encode(format!("{url}\0{}", asset_name.unwrap_or("*")).as_bytes()))
            .with_extension("txt"),
    )
}

fn read_cached_size(url: &str, asset_name: Option<&str>) -> Option<u128> {
    let path = cache_file(url, asset_name)?;
    fs::read_to_string(path).ok()?.trim().parse().ok()
}

fn write_cached_size(url: &str, asset_name: Option<&str>, size: u128) {
    let Some(path) = cache_file(url, asset_name) else {
        return;
    };
    let Some(parent) = path.parent() else {
        return;
    };

    if fs::create_dir_all(parent).is_err() {
        return;
    }

    let temporary_path = path.with_extension(format!("tmp-{}", std::process::id()));
    if fs::write(&temporary_path, size.to_string()).is_ok() {
        let _ = fs::rename(temporary_path, path);
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

#[derive(Debug, FromField)]
#[darling(attributes(form))]
struct FormFieldOptions {
    #[darling(default)]
    value: Option<Expr>,
}

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
                            if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = &meta.value {
                                return Some(lit_str.value());
                            }
                        }
                    }
                    None
                })
                .next();

            class_value.unwrap_or_else(|| {
                panic!("Variant `{}::{}` is missing `#[class = \"...\"]` attribute", name, variant.ident)
            })
        })
        .collect();

    let variant_ids: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    // Build match arms with literal class strings
    let match_arms = variant_ids.iter().zip(class_strings.iter()).map(|(variant, class)| {
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
/// `datetime-local`, `number`, `checkbox`, otherwise `text`). Boolean
/// defaults are emitted as `checked` attributes.
///
/// Fields honour `#[serde(rename = "...")]`, `#[schemars(rename = "...")]`
/// and are omitted for `#[serde(skip)]`.
///
/// A field can define a computed HTML value with `#[form(value = expression)]`.
/// The expression is evaluated when `attrs` is called, for example:
///
/// ```ignore
/// #[form(value = default_cpus().to_string())]
/// cpu_cores: u8,
/// ```
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
#[proc_macro_derive(FormValidation, attributes(form))]
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
        return Err(syn::Error::new_spanned(&input.generics, "`FormValidation` does not support generic structs"));
    }
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            syn::Fields::Named(named) => &named.named,
            _ => {
                return Err(syn::Error::new_spanned(input, "`FormValidation` requires a struct with named fields"));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(input, "`FormValidation` can only be derived for structs"));
        }
    };

    let struct_name = &input.ident;
    let vis = &input.vis;
    let enum_name = syn::Ident::new(&format!("{struct_name}Validation"), struct_name.span());

    let mut variants = Vec::new();
    let mut names = Vec::new();
    let mut values: Vec<Option<Expr>> = Vec::new();
    let mut boolean_fields = Vec::new();
    for field in fields {
        let ident = field.ident.as_ref().expect("named field");
        let mut property = ident.unraw().to_string();
        let mut skip = false;
        let form_options =
            FormFieldOptions::from_field(field).map_err(|error| syn::Error::new_spanned(field, error.to_string()))?;
        let value = form_options.value;
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
        boolean_fields.push(is_bool_type(&field.ty));
        values.push(value);
    }

    let value_arms =
        variants.iter().zip(values.iter()).zip(boolean_fields.iter()).map(|((variant, value), is_boolean)| {
            if let Some(value) = value {
                if *is_boolean {
                    quote! {
                        Self::#variant => {
                            attrs.insert(cx, "value", "true");
                            if #value {
                                attrs.insert(cx, "checked", true);
                            }
                        }
                    }
                } else {
                    quote! {
                        Self::#variant => {
                            attrs.insert(cx, "value", #value);
                        }
                    }
                }
            } else {
                quote! { Self::#variant => {} }
            }
        });

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

                if let Some(prop) = schema.get("properties").and_then(|p| p.get(name)) {
                    // `Option<T>` fields use "type": ["string", "null"].
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
                    (_, Some("boolean")) => "checkbox",
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
                } else {
                    attrs.insert(cx, "type", "text");
                }

                match self {
                    #(#value_arms,)*
                }
                attrs
            }
        }
    })
}

fn is_bool_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(path) = ty else {
        return false;
    };
    path.path.segments.last().is_some_and(|segment| segment.ident == "bool")
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map(|c| c.to_uppercase().chain(chars).collect::<String>()).unwrap_or_default()
        })
        .collect()
}
