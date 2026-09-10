use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, ItemStruct, Token, Type, braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Const,
};

use crate::SegmendKind::{Capture, Constant};
use proc_macro2::{Span, TokenStream as TokenStream2};

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

struct CaptureInput {
    name: Ident,
    typ: Type,
}
impl Parse for CaptureInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let typ: Type = input.parse()?;

        Ok(Self { name, typ })
    }
}

#[proc_macro]
pub fn route_capture(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CaptureInput);
    let name = input.name;
    let typ = input.typ;

    let snake_name = to_snake_case(&name.to_string());

    let expanded = quote! {
        pub struct #name<N> {
            _marker: ::std::marker::PhantomData<N>,
        }

        impl<N> Route for #name<N> {}

        impl<N> RouteSegment<N> for #name<N>
        where
            N: Route,
        {}

        impl<N> RouteCapture<N> for #name<N>
        where
            N: Route,
        {
            const NAME: &'static str = #snake_name;
            type Ty = #typ;
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn route_constant(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);

    let lower_name = name.to_string().to_lowercase();

    let expanded = quote! {
        pub struct #name<N> {
            _marker: ::std::marker::PhantomData<N>,
        }

        impl<N> Route for #name<N> {}

        impl<N> RouteSegment<N> for #name<N>
        where
            N: Route,
        {}

        impl<N> RouteConstant<N> for #name<N>
        where
            N: Route,
        {
            const VALUE: &'static str = #lower_name;
        }
    };

    TokenStream::from(expanded)
}

fn build_path(segments: Vec<Segment>) -> TokenStream2 {
    let mut format_parts = Vec::new();
    let mut arguments = Vec::new();

    let idents: Vec<Ident> = segments.iter().map(|seg| seg.ident.clone()).collect();

    for (i, seg) in segments.iter().enumerate() {
        format_parts.push("{}");

        let ident = build_generic2(idents[i..].to_vec());
        println!("{}", ident.to_string());
        match seg.kind {
            Constant => {
                arguments.push(quote! {
                    #ident::VALUE
                });
            }

            Capture => {
                arguments.push(quote! {
                    #ident::NAME
                });
            }
        }
    }

    let format_string = format!("/{}", format_parts.join("/"));

    quote! {
        format!(#format_string, #(#arguments),*)
    }
}

fn build_generic2(idents: Vec<Ident>) -> TokenStream2 {
    let mut ret = quote! { EndRoute };
    for ident in idents.iter().rev() {
        ret = quote! {
            #ident::<#ret>
        };
    }
    ret
}

fn build_generic(idents: Vec<Ident>) -> TokenStream2 {
    let mut ret = quote! { EndRoute };
    for ident in idents.iter().rev() {
        ret = quote! {
            #ident<#ret>
        };
    }
    ret
}

fn check_struct(segments: Vec<Segment>, item: ItemStruct) -> TokenStream2 {
    let fields = item
        .fields
        .into_iter()
        .filter(|x| x.ident.is_some())
        .map(|x| (x.ident.unwrap(), x.ty))
        .collect::<Vec<(Ident, Type)>>();
    let captures = segments
        .into_iter()
        .filter(|x| match x.kind {
            Capture => true,
            Constant => false,
        })
        .map(|x| x.ident.to_string());

    quote! {
        assert!(true, "test");
    }
}

#[derive(Clone)]
enum SegmendKind {
    Constant,
    Capture,
}

#[derive(Clone)]
struct Segment {
    ident: Ident,
    kind: SegmendKind,
}
impl Parse for Segment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);

            let ident: Ident = content.parse()?;

            Ok(Self {
                ident,
                kind: SegmendKind::Capture,
            })
        } else {
            let ident: Ident = input.parse()?;

            Ok(Self {
                ident,
                kind: SegmendKind::Constant,
            })
        }
    }
}

struct ParamsInput {
    identifiers: Punctuated<Segment, Token![,]>,
}

impl Parse for ParamsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            identifiers: Punctuated::<Segment, Token![,]>::parse_terminated(input)?,
        })
    }
}

#[proc_macro_attribute]
pub fn params(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    let input = parse_macro_input!(attr as ParamsInput);
    let route = build_path(input.identifiers.iter().cloned().collect());
    let generic = build_generic(
        input
            .identifiers
            .iter()
            .map(|seg| seg.ident.clone())
            .collect::<Vec<Ident>>(),
    );

    let ident = &item.ident;
    let check_fn_name = to_snake_case(&format!("__check{}", ident.to_string()));
    let check_fn_ident = Ident::new(&check_fn_name, Span::call_site());

    let check_struct = check_struct(input.identifiers.iter().cloned().collect(), item.clone());
    quote! {
        #item

        impl Parameters<#generic> for #ident {
            fn get_route_string() -> String {
                #route
            }
        }
    }
    .into()
}
