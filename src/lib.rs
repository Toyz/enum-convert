use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Lit, Variant, Attribute, Type, parse_str, Expr, ExprLit};

// Helper to extract the EnumType attribute and parse its type
fn extract_enum_type(attrs: &[Attribute]) -> Option<Vec<Type>> {
    attrs.iter().find_map(|attr| {
        if attr.path.is_ident("EnumType") {
            attr.tokens.clone().into_iter().find_map(|token| {
                if let proc_macro2::TokenTree::Literal(lit) = token {
                    let type_str = lit.to_string().trim_matches('"').to_string();
                    let type_str = type_str.trim();
                    let mut enum_types = vec![];

                    for type_str in type_str.split(',').map(|s| s.trim()) {
                        if let Ok(t) = parse_str::<Type>(&type_str) {
                            enum_types.push(t);
                        }
                    }
                    Some(enum_types)
                } else {
                    None
                }
            })
        } else {
            None
        }
    })
}

#[proc_macro_derive(EnumConvert, attributes(EnumType))]
pub fn enum_convert(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let enum_type = extract_enum_type(&ast.attrs).expect("EnumType attribute is missing");

    let variants = if let Data::Enum(data_enum) = ast.data {
        data_enum.variants
    } else {
        panic!("EnumConvert only supports enums");
    };

    let mut streams = vec![];
    for enum_type in &enum_type {
        let (from_impl, into_impl) = match &enum_type {
            Type::Path(type_path) if type_path.path.is_ident("String") => {
                generate_string_conversions(name, &variants)
            },
            _ => {
                generate_numeric_conversions(name, &enum_type, &variants)
            },
        };

        let gen = quote! {
        #from_impl
        #into_impl
    };

        streams.push(gen);
    }

    (quote! {
        #(#streams)*
    }).into()
}

fn generate_numeric_conversions(name: &Ident, enum_type: &Type, variants: &syn::punctuated::Punctuated<Variant, syn::token::Comma>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut last_number = 0i64; // Use i64 to keep track of the last number under the hood

    let from_arms = variants.iter().map(|v| {
        let ident = &v.ident;
        let value = if let Some((_, expr)) = &v.discriminant {
            // Parse the provided discriminant to get the numeric value
            if let Ok(lit) = parse_literal_expr_to_i64(expr) {
                last_number = lit + 1; // Set the next number to be the current literal plus one
                lit
            } else {
                return quote! { compile_error!("Invalid literal for enum discriminant."); };
            }
        } else {
            let current_number = last_number;
            last_number += 1; // Increment the last number for the next variant
            current_number
        };

        // Emit the raw number without a type suffix
        let number_tokens = proc_macro2::Literal::i64_unsuffixed(value);
        quote! { #number_tokens => Ok(#name::#ident) }
    }).collect::<Vec<_>>();

    let try_from_impl = quote! {
        impl std::convert::TryFrom<#enum_type> for #name {
            type Error = &'static str;

            fn try_from(value: #enum_type) -> Result<Self, Self::Error> {
                match value {
                    #( #from_arms, )*
                    _ => Err("No matching enum variant"),
                }
            }
        }
    };

    // Reset last_number for the Into implementation
    last_number = 0;

    let into_arms = variants.iter().map(|v| {
        let ident = &v.ident;
        let value = if let Some((_, expr)) = &v.discriminant {
            if let Ok(lit) = parse_literal_expr_to_i64(expr) {
                last_number = lit + 1; // Update the next number
                lit
            } else {
                return quote! { compile_error!("Invalid literal for enum discriminant."); };
            }
        } else {
            let current_number = last_number;
            last_number += 1; // Increment the last number for the next variant
            current_number
        };

        let number_tokens = proc_macro2::Literal::i64_unsuffixed(value);
        quote! { #name::#ident => #number_tokens }
    }).collect::<Vec<_>>();

    let from_impl = quote! {
        impl From<#name> for #enum_type {
            fn from(variant: #name) -> Self {
                match variant {
                    #( #into_arms, )*
                }
            }
        }

        impl From<#enum_type> for #name {
            fn from(value: #enum_type) -> Self {
                match value {
                    #( #from_arms, )*
                    _ => panic!("No matching enum variant"),
                }
            }
        }
    };

    (try_from_impl, from_impl)
}

fn parse_literal_expr_to_i64(expr: &Expr) -> Result<i64, String> {
    match expr {
        Expr::Lit(ExprLit {
                      lit: Lit::Int(lit_int), ..
                  }) => {
            lit_int.base10_parse::<i64>().map_err(|e| e.to_string())
        },
        _ => Err("Expected integer literal".to_string()),
    }
}
fn generate_string_conversions(name: &Ident, variants: &syn::punctuated::Punctuated<Variant, syn::token::Comma>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let from_variants = variants.iter().map(|v| {
        let ident = &v.ident;
        quote! { stringify!(#ident) => Ok(#name::#ident), }
    });

    let into_variants = variants.iter().map(|v| {
        let ident = &v.ident;
        quote! { #name::#ident => stringify!(#ident).to_string(), }
    });

    let from_impl = quote! {
        impl std::str::FromStr for #name {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #( #from_variants )*
                    _ => Err("No matching enum variant"),
                }
            }
        }
    };

    let into_impl = quote! {
        impl std::string::ToString for #name {
            fn to_string(&self) -> String {
                match self {
                    #( #into_variants )*
                }
            }
        }
    };

    (from_impl, into_impl)
}
