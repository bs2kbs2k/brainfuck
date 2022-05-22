use std::process::id;

use convert_case::{Case, Casing};
use proc_macro2::Literal;
use quote::quote;
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    token::{Brace, Bracket, Paren},
    Error, Ident, LitStr, Token,
};

extern crate proc_macro;

struct EbnfStmt {
    lhs: String,
    rhs: EbnfExpr,
}

enum EbnfExpr {
    Identifier(String),
    Literal(String),
    Optional(Box<EbnfExpr>),
    Repeat(Box<EbnfExpr>),
    Group(Box<EbnfExpr>),
    Alternative(Vec<EbnfExpr>),
    Sequence(Vec<EbnfExpr>),
}

#[derive(PartialEq)]
enum EbnfExprType {
    Alternative,
    Sequence,
}

struct EbnfFile(Vec<EbnfStmt>);

impl Parse for EbnfFile {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut stmts = Vec::new();
        while !input.is_empty() {
            println!("{:?}", input);
            let stmt = input.parse::<EbnfStmt>()?;
            stmts.push(stmt);
        }
        Ok(EbnfFile(stmts))
    }
}

impl Parse for EbnfStmt {
    fn parse(input: ParseStream) -> Result<Self> {
        println!("parse EbnfStmt");
        let lhs = {
            let mut ident_tokens = vec![];
            while !input.peek(Token![=]) {
                println!("{:?}", input);
                ident_tokens.push(format!("{}", input.parse::<Ident>()?));
            }
            ident_tokens.join("_")
        };
        input.parse::<Token![=]>()?;
        let rhs = input.parse::<EbnfExpr>()?;
        Ok(EbnfStmt { lhs, rhs })
    }
}

impl Parse for EbnfExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        println!("parse EbnfExpr");
        let mut exprs = vec![];
        let mut expr_type = None;
        while !input.peek(Token![;]) {
            println!("{:?}", input);
            if input.peek(Ident) {
                let mut ident_tokens = vec![];
                while input.peek(Ident) {
                    ident_tokens.push(format!("{}", input.parse::<Ident>()?));
                }
                exprs.push(EbnfExpr::Identifier(ident_tokens.join("_")));
            } else if input.peek(LitStr) {
                exprs.push(EbnfExpr::Literal(input.parse::<LitStr>()?.value()));
            } else if input.peek(Paren) {
                let expr;
                parenthesized!(expr in input);
                exprs.push(EbnfExpr::Group(Box::new(expr.parse::<EbnfExpr>()?)));
            } else if input.peek(Brace) {
                let expr;
                braced!(expr in input);
                exprs.push(EbnfExpr::Repeat(Box::new(expr.parse::<EbnfExpr>()?)));
            } else if input.peek(Bracket) {
                let expr;
                bracketed!(expr in input);
                exprs.push(EbnfExpr::Optional(Box::new(expr.parse::<EbnfExpr>()?)));
            } else if input.peek(Token![|]) {
                input.parse::<Token![|]>()?;
                if expr_type.is_some() && expr_type != Some(EbnfExprType::Alternative) {
                    return Err(Error::new(input.span(), "ur ebnf bad"));
                } else {
                    expr_type = Some(EbnfExprType::Alternative);
                }
            } else if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                if expr_type.is_some() && expr_type != Some(EbnfExprType::Sequence) {
                    return Err(Error::new(input.span(), "ur ebnf bad"));
                } else {
                    expr_type = Some(EbnfExprType::Sequence);
                }
            } else {
                break;
                return Err(Error::new(input.span(), "ur ebnf bad"));
            }
        }
        input.parse::<Token![;]>();
        match expr_type {
            Some(EbnfExprType::Alternative) => Ok(EbnfExpr::Alternative(exprs)),
            Some(EbnfExprType::Sequence) => Ok(EbnfExpr::Sequence(exprs)),
            None => {
                if exprs.len() == 1 {
                    Ok(exprs.pop().unwrap())
                } else {
                    Err(Error::new(input.span(), "ur ebnf bad"))
                }
            }
        }
    }
}

impl EbnfExpr {
    fn generate_type(&self, name: String) -> proc_macro2::TokenStream {
        let name = Ident::new(&name, proc_macro2::Span::call_site());
        match self {
            EbnfExpr::Identifier(ident) => {
                let ident =
                    Ident::new(&ident.to_case(Case::Pascal), proc_macro2::Span::call_site());
                quote! {
                    pub struct #name(#ident);
                    impl Parse for #name {
                        fn parse(input: String) -> Result<(Self, String)> {
                            let (expr, rest) = #ident::parse(input)?;
                            Ok((#name(expr), rest))
                        }
                    }
                }
            }
            EbnfExpr::Literal(lit) => {
                let lit = Literal::string(lit);
                quote! {
                    pub struct #name;
                    impl Parse for #name {
                        fn parse(input: String) -> Result<(Self, String)> {
                            match input.strip_prefix(#lit) {
                                Some(_) => Ok((#name, input.strip_prefix(#lit).unwrap().to_string())),
                                None => Err(anyhow::anyhow!("invalid input")),
                            }
                        }
                    }
                }
            }
            EbnfExpr::Optional(expr) => {
                let expr = expr.generate_type(
                    (format!("{}", name).to_case(Case::Pascal) + "Inner").to_case(Case::Pascal),
                );
                let expr_name = Ident::new(
                    &(format!("{}", name) + "Inner").to_case(Case::Pascal),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    #expr
                    pub struct #name(Option<#expr_name>);
                    impl Parse for #name {
                        fn parse(input: String) -> Result<(Self, String)> {
                            let (expr, rest) = #expr_name::parse(input)?;
                            Ok((#name(expr), rest))
                        }
                    }
                }
            }
            EbnfExpr::Repeat(expr) => {
                let expr =
                    expr.generate_type((format!("{}", name) + "Inner").to_case(Case::Pascal));
                let expr_name = Ident::new(
                    &(format!("{}", name) + "Inner").to_case(Case::Pascal),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    #expr
                    pub struct #name(Vec<#expr_name>);
                    impl Parse for #name {
                        fn parse(input: String) -> Result<(Self, String)> {
                            let mut exprs = vec![];
                            let mut rest = input;
                            while let Ok((expr, expr_rest)) = #expr_name::parse(rest.clone()) {
                                exprs.push(expr);
                                rest = expr_rest;
                            }
                            Ok((#name(exprs), rest))
                        }
                    }
                }
            }
            EbnfExpr::Group(expr) => {
                let expr =
                    expr.generate_type((format!("{}", name) + "Inner").to_case(Case::Pascal));
                let expr_name = Ident::new(
                    &(format!("{}", name) + "Inner").to_case(Case::Pascal),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    #expr
                    pub struct #name(#expr_name);
                    impl Parse for #name {
                        fn parse(input: String) -> Result<(Self, String)> {
                            let (expr, rest) = #expr_name::parse(input)?;
                            Ok((#name(expr), rest))
                        }
                    }
                }
            }
            EbnfExpr::Alternative(exprs) => {
                let mut types = vec![];
                for (idx, expr) in exprs.iter().enumerate() {
                    let expr = expr.generate_type(
                        (format!("{}", name) + &format!("Inner{}", idx)).to_case(Case::Pascal),
                    );
                    let expr_name = Ident::new(
                        &(format!("{}", name) + &format!("Inner{}", idx)).to_case(Case::Pascal),
                        proc_macro2::Span::call_site(),
                    );
                    types.push((
                        quote! {
                            #expr
                        },
                        expr_name,
                    ));
                }
                let types_tkn = types.iter().fold(quote! {}, |acc, (type_, name)| {
                    quote! {
                        #acc
                        #type_
                    }
                });
                let type_names = types
                    .iter()
                    .map(|(_, name)| name)
                    .fold(quote! {}, |acc, name| {
                        quote! {
                            #acc
                            #name(#name),
                        }
                    });
                let parse_body = types.iter().fold(quote! {}, |acc, (_, typ_name)| {
                    quote! {
                        #acc
                        if let Ok((expr, rest)) = #typ_name::parse(input.clone()) {
                            return Ok((#name::#typ_name(expr), rest));
                        }
                    }
                });
                quote! {
                    #types_tkn
                    pub enum #name {
                        #type_names
                    }
                    impl Parse for #name {
                        fn parse(input: String) -> Result<(Self, String)> {
                            #parse_body
                            Err(anyhow::anyhow!("invalid input"))
                        }
                    }
                }
            }
            EbnfExpr::Sequence(exprs) => {
                let mut types = vec![];
                for (idx, expr) in exprs.iter().enumerate() {
                    let expr = expr.generate_type(
                        (format!("{}", name) + &format!("Inner{}", idx)).to_case(Case::Pascal),
                    );
                    let expr_name = Ident::new(
                        &(format!("{}", name) + &format!("Inner{}", idx)).to_case(Case::Pascal),
                        proc_macro2::Span::call_site(),
                    );
                    types.push((
                        quote! {
                            #expr
                        },
                        expr_name,
                    ));
                }
                let types_tkn = types.iter().fold(quote! {}, |acc, (type_, name)| {
                    quote! {
                        #acc
                        #type_
                    }
                });
                let type_names = types
                    .iter()
                    .map(|(_, name)| name)
                    .fold(quote! {}, |acc, name| {
                        quote! {
                            #acc, #name
                        }
                    });
                let parse_body = types.iter().fold(quote! {}, |acc, (_, typ_name)| {
                    let var_name = Ident::new(
                        &(format!("Token{}", typ_name).to_case(Case::Snake)),
                        proc_macro2::Span::call_site(),
                    );
                    quote! {
                        #acc
                        let (#var_name, rest) = #typ_name::parse(rest)?;
                    }
                });
                let parse_result = types.iter().fold(quote! {}, |acc, (_, typ_name)| {
                    let typ_name = Ident::new(
                        &(format!("Token{}", typ_name).to_case(Case::Snake)),
                        proc_macro2::Span::call_site(),
                    );
                    quote! {
                        #acc, #typ_name
                    }
                });
                quote! {
                    #types_tkn
                    pub struct #name(()#type_names);
                    impl Parse for #name {
                        fn parse(input: String) -> Result<(Self, String)> {
                            let mut rest = input;
                            #parse_body
                            Ok((#name(()#parse_result), rest))
                        }
                    }
                }
            }
        }
    }
}

#[proc_macro]
pub fn ebnf_generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // nedo said no =(
    // shelve this for now
    // std::thread::sleep(std::time::Duration::from_secs(600));
    let input = parse_macro_input!(input as EbnfFile);

    println!("1");

    let output: proc_macro2::TokenStream = {
        let mut output = quote! {
            use anyhow::{anyhow, Result};
            trait Parse
            where
                Self: Sized,
            {
                fn parse(input: String) -> Result<(Self, String)>;
            }
        };
        for statement in input.0 {
            let lhs = &statement.lhs.to_case(Case::Pascal);
            let rhs = &statement.rhs;
            let expr = rhs.generate_type(lhs.clone());
            output.extend(expr);
        }
        output
    };

    proc_macro::TokenStream::from(output)
}
