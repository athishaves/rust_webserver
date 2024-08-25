use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemImpl, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as syn::AttributeArgs);

    let mut base_path = None;
    let mut middlewares = Vec::new();

    for arg in attr_args.iter() {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("path") {
                if let syn::Lit::Str(lit_str) = &nv.lit {
                    base_path = Some(lit_str.value());
                }
            } else if nv.path.is_ident("middleware") {
                if let syn::Lit::Str(lit_str) = &nv.lit {
                    middlewares.push(lit_str.value());
                }
            }
        }
    }

    let ast = parse_macro_input!(item as ItemImpl);
    let struct_name = &ast.self_ty;

    let mut routes: Vec<(String, String, String)> = Vec::new();

    for item in ast.items.iter() {
        if let syn::ImplItem::Method(method) = item {
            for attr in &method.attrs {
                if attr.path.is_ident("route") {
                    if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                        let mut method_str = None;
                        let mut path_str = None;

                        for nested in meta_list.nested.iter() {
                            if let NestedMeta::Lit(syn::Lit::Str(lit_str)) = nested {
                                if method_str.is_none() {
                                    method_str = Some(lit_str.value());
                                } else if path_str.is_none() {
                                    path_str = Some(lit_str.value());
                                }
                            }
                        }

                        let method_str =
                            method_str.expect("Expected a string literal for HTTP method");
                        let path_str = path_str.expect("Expected a string literal for path");

                        routes.push((method_str, path_str, method.sig.ident.to_string()));
                    }
                }
            }
        }
    }

    let route_definitions = routes.iter().map(|(method, path, handler_name)| {
        let full_path = format!("{}{}", base_path.get_or_insert("".to_string()), path);
        let handler_ident = format_ident!("{}", handler_name);

        match method.as_str() {
            "DELETE" => quote! { .route(#full_path, axum::routing::delete(Self::#handler_ident)) },
            "GET" => quote! { .route(#full_path, axum::routing::get(Self::#handler_ident)) },
            "HEAD" => quote! { .route(#full_path, axum::routing::head(Self::#handler_ident)) },
            "OPTIONS" => {
                quote! { .route(#full_path, axum::routing::options(Self::#handler_ident)) }
            }
            "PATCH" => quote! { .route(#full_path, axum::routing::patch(Self::#handler_ident)) },
            "POST" => quote! { .route(#full_path, axum::routing::post(Self::#handler_ident)) },
            "PUT" => quote! { .route(#full_path, axum::routing::put(Self::#handler_ident)) },
            "TRACE" => quote! { .route(#full_path, axum::routing::trace(Self::#handler_ident)) },
            _ => quote! {},
        }
    });

    let middleware_stack = middlewares.iter().map(|middleware| {
        let middleware_ident = format_ident!("{}", middleware);
        quote! { .layer(axum::middleware::from_fn(#middleware_ident)) }
    });

    let expanded = quote! {
        #ast

        impl #struct_name {
            pub fn router() -> axum::Router {
                axum::Router::new()
                    #(#route_definitions)*
                    #(#middleware_stack)*
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
