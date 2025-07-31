use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input};

#[proc_macro_derive(ScriptArgs)]
pub fn script_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let mut fields = Vec::new();
    if let Data::Struct(ref data) = input.data {
        if let Fields::Named(ref field_names) = data.fields {
            for field in field_names.named.iter() {
                let field_ident = field.ident.as_ref().unwrap();
                let field_name = field_ident.to_string();
                if let Some(field_token) = get_field(&field_name, &field.ty) {
                    fields.push(quote! {
                        #field_ident: #field_token
                    });
                }
            }
        }
    }

    let expanded = quote! {
        impl #struct_name {
            pub fn from_args(
                args: std::collections::HashMap<String, bevy_mod_scripting::core::bindings::ScriptValue>,
                world: bevy_mod_scripting::core::bindings::WorldGuard,
            ) -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    };
    TokenStream::from(expanded)
}

fn get_field(field_name: &str, ty: &Type) -> Option<proc_macro2::TokenStream> {
    let type_ident = get_type_ident(ty)?;
    if type_ident == "Option" {
        let inner_type = get_option_inner_type(ty)?;
        let inner_ident = get_type_ident(inner_type)?;
        Some(expand_method(&inner_ident, field_name))
    } else {
        let f = expand_method(&type_ident, field_name);
        Some(quote! {
            #f.expect(&format!("Missing args: {}", #field_name))
        })
    }
}

fn get_type_ident(ty: &Type) -> Option<proc_macro2::Ident> {
    if let Type::Path(ty_path) = ty {
        if let Some(segment) = ty_path.path.segments.last() {
            return Some(segment.ident.clone());
        }
    }
    None
}

fn get_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(ty_path) = ty {
        if let Some(segment) = ty_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(gen_args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = gen_args.args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}

fn expand_method(type_ident: &proc_macro2::Ident, field_name: &str) -> proc_macro2::TokenStream {
    if type_ident == "String" {
        quote! { args.get_string(#field_name) }
    } else if type_ident == "usize" {
        quote! { args.get_usize(#field_name) }
    } else if type_ident == "bool" {
        quote! { args.get_bool(#field_name) }
    } else if type_ident == "f32" {
        quote! { args.get_f32(#field_name) }
    } else if type_ident == "f64" {
        quote! { args.get_f64(#field_name) }
    } else if type_ident == "i64" {
        quote! { args.get_i64(#field_name) }
    } else {
        quote! { args.get_reflect(#field_name, world.clone()) }
    }
}
