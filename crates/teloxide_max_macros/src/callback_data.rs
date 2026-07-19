use crate::{error::compile_error_at, Result};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, Lit, Meta};

/// Parsed enum-level attributes for `#[callback_data(...)]`.
struct CallbackDataEnumAttrs {
    prefix: String,
    separator: char,
}

/// Parsed variant-level attributes for `#[callback_data(suffix = "...")]`.
struct CallbackDataVariantAttrs {
    suffix: String,
}

/// Represents a parsed variant of the CallbackData enum.
struct CallbackDataVariant {
    ident: syn::Ident,
    suffix: String,
    fields: Vec<(syn::Ident, syn::Type)>,
}

fn parse_enum_attrs(attrs: &[syn::Attribute]) -> Result<CallbackDataEnumAttrs> {
    let mut prefix = None;
    let mut separator = ':';

    for attr in attrs {
        if !attr.path().is_ident("callback_data") {
            continue;
        }

        let meta = &attr.meta;
        if let Meta::List(meta_list) = meta {
            let _nested = meta_list.parse_nested_meta(|meta| {
                if meta.path.is_ident("prefix") {
                    let value = meta.value()?;
                    let lit: Lit = value.parse()?;
                    if let Lit::Str(s) = lit {
                        prefix = Some(s.value());
                    } else {
                        return Err(meta.error("expected string literal"));
                    }
                } else if meta.path.is_ident("separator") {
                    let value = meta.value()?;
                    let lit: Lit = value.parse()?;
                    if let Lit::Char(c) = lit {
                        separator = c.value();
                    } else if let Lit::Str(s) = lit {
                        let chars: Vec<char> = s.value().chars().collect();
                        if chars.len() == 1 {
                            separator = chars[0];
                        } else {
                            return Err(meta.error("separator must be a single character"));
                        }
                    } else {
                        return Err(meta.error("expected char or string literal"));
                    }
                }
                Ok(())
            })?;
        }
    }

    let prefix = prefix.ok_or_else(|| {
        compile_error_at(
            "#[callback_data(prefix = \"...\")] attribute is required",
            proc_macro2::Span::call_site(),
        )
    })?;

    Ok(CallbackDataEnumAttrs { prefix, separator })
}

fn parse_variant_attrs(attrs: &[syn::Attribute]) -> Result<CallbackDataVariantAttrs> {
    let mut suffix = None;

    for attr in attrs {
        if !attr.path().is_ident("callback_data") {
            continue;
        }

        let meta = &attr.meta;
        if let Meta::List(meta_list) = meta {
            let _ = meta_list.parse_nested_meta(|meta| {
                if meta.path.is_ident("suffix") {
                    let value = meta.value()?;
                    let lit: Lit = value.parse()?;
                    if let Lit::Str(s) = lit {
                        suffix = Some(s.value());
                    } else {
                        return Err(meta.error("expected string literal"));
                    }
                }
                Ok(())
            });
        }
    }

    Ok(CallbackDataVariantAttrs { suffix: suffix.unwrap_or_default() })
}

pub(crate) fn callback_data_impl(input: DeriveInput) -> Result<TokenStream> {
    let enum_data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return Err(compile_error_at(
                "CallbackData can only be derived for enums",
                input.ident.span(),
            ))
        }
    };

    let enum_attrs = parse_enum_attrs(&input.attrs)?;
    let prefix = &enum_attrs.prefix;
    let separator = enum_attrs.separator.to_string();

    let type_name = &input.ident;
    let lifetime = &input.generics;
    let where_clause = &lifetime.where_clause;

    let mut variants = Vec::new();

    for variant in &enum_data.variants {
        let variant_attrs = parse_variant_attrs(&variant.attrs)?;
        let suffix = if variant_attrs.suffix.is_empty() {
            variant.ident.to_string()
        } else {
            variant_attrs.suffix
        };

        // Check that separator is not in prefix
        if prefix.contains(enum_attrs.separator) {
            return Err(compile_error_at(
                &format!(
                    "Separator '{}' cannot be used inside prefix '{}'",
                    enum_attrs.separator, prefix
                ),
                input.ident.span(),
            ));
        }

        let fields: Vec<(syn::Ident, syn::Type)> = match &variant.fields {
            Fields::Unit => Vec::new(),
            Fields::Named(named) => named
                .named
                .iter()
                .filter_map(|f| f.ident.clone().map(|i| (i, f.ty.clone())))
                .collect(),
            Fields::Unnamed(_) => {
                return Err(compile_error_at(
                    "Tuple variants are not supported in CallbackData",
                    variant.ident.span(),
                ))
            }
        };

        variants.push(CallbackDataVariant { ident: variant.ident.clone(), suffix, fields });
    }

    // Generate the prefix function
    let prefix_fn = quote! {
        fn prefix() -> &'static str {
            #prefix
        }
    };

    // Generate the separator function
    let separator_fn = quote! {
        fn separator() -> char {
            #separator.chars().next().unwrap_or(':')
        }
    };

    // Generate the serialize function
    let serialize_arms: Vec<TokenStream> = variants
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let suffix = &v.suffix;
            let full = format!("{}{}{}", prefix, separator, suffix);

            if v.fields.is_empty() {
                quote! {
                    Self::#ident => Ok(#full.to_string()),
                }
            } else {
                let field_names: Vec<&syn::Ident> = v.fields.iter().map(|(name, _)| name).collect();
                quote! {
                    Self::#ident { #(#field_names),* } => {
                        let mut result = #full.to_string();
                        #(
                            result.push_str(&#separator.to_string());
                            result.push_str(&#field_names.to_string());
                        )*
                        if result.len() > 64 {
                            return Err(teloxide_max::utils::callback_data::CallbackDataError::TooLong {
                                len: result.len(),
                                max: 64,
                            });
                        }
                        Ok(result)
                    }
                }
            }
        })
        .collect();

    let serialize_fn = quote! {
        fn serialize(&self) -> ::std::result::Result<String, teloxide_max::utils::callback_data::CallbackDataError> {
            match self {
                #(#serialize_arms)*
            }
        }
    };

    // Generate the deserialize function
    let deserialize_arms: Vec<TokenStream> = variants
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let suffix = &v.suffix;
            let full = format!("{}{}{}", prefix, separator, suffix);

            if v.fields.is_empty() {
                quote! {
                    #full => Ok(Self::#ident),
                }
            } else {
                let field_parsers: Vec<TokenStream> = v
                    .fields
                    .iter()
                    .map(|(name, ty)| {
                        let name_str = name.to_string();
                        quote! {
                            let #name: #ty = parts.next()
                                .ok_or_else(|| teloxide_max::utils::callback_data::CallbackDataError::DeserializationError(
                                    format!("missing field '{}'", #name_str)
                                ))?
                                .parse()
                                .map_err(|e: ::std::num::ParseIntError| teloxide_max::utils::callback_data::CallbackDataError::DeserializationError(
                                    format!("failed to parse field '{}': {}", #name_str, e)
                                ))?;
                        }
                    })
                    .collect();

                let field_names: Vec<&syn::Ident> = v.fields.iter().map(|(name, _)| name).collect();

                quote! {
                    #full => {
                        let rest = &data[#full.len()..];
                        if !rest.is_empty() && !rest.starts_with(#separator) {
                            return Err(teloxide_max::utils::callback_data::CallbackDataError::DeserializationError(
                                format!("expected separator '{}' after '{}'", #separator, #full)
                            ));
                        }
                        let rest = if rest.starts_with(#separator) { &rest[1..] } else { "" };
                        let mut parts = rest.split(#separator);
                        #(#field_parsers)*
                        Ok(Self::#ident { #(#field_names),* })
                    }
                }
            }
        })
        .collect();

    let deserialize_fn = quote! {
        fn deserialize(data: &str) -> ::std::result::Result<Self, teloxide_max::utils::callback_data::CallbackDataError> {
            // Try each variant's full prefix (prefix + separator + suffix)
            match data {
                #(#deserialize_arms)*
                _ => Err(teloxide_max::utils::callback_data::CallbackDataError::DeserializationError(
                    format!("no matching variant for data: '{}'", data)
                )),
            }
        }
    };

    let impl_block = quote! {
        impl #lifetime teloxide_max::utils::callback_data::CallbackData for #type_name #lifetime #where_clause {
            #prefix_fn
            #separator_fn
            #serialize_fn
            #deserialize_fn
        }

        impl #lifetime ::std::fmt::Display for #type_name #lifetime #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let s = self.serialize().map_err(|_| ::std::fmt::Error)?;
                write!(f, "{}", s)
            }
        }

        impl #lifetime ::std::str::FromStr for #type_name #lifetime #where_clause {
            type Err = teloxide_max::utils::callback_data::CallbackDataError;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                Self::deserialize(s)
            }
        }
    };

    Ok(impl_block)
}
