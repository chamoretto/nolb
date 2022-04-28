use proc_macro::TokenStream;

use proc_macro2::TokenTree;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Token};

struct Conditional {
    condition: Expr,
    true_branch: Expr,
    false_branch: Expr,
}

fn get_until_question_mark(input: ParseStream) -> Result<Expr> {
    let mut skipped_tokens = vec![];
    while !input.peek(Token![?]) || (input.peek(Token![?]) && input.peek2(Token![?])) {
        let tree = input.parse::<TokenTree>()?;
        skipped_tokens.push(tree);
    }
    syn::parse2(skipped_tokens.into_iter().collect())
}

fn get_until_colon(input: ParseStream) -> Result<Expr> {
    let mut skipped_tokens = vec![];
    while !input.peek(Token![:]) || input.peek(Token![::]) {
        let tree = input.parse::<TokenTree>()?;
        skipped_tokens.push(tree);
    }
    syn::parse2(skipped_tokens.into_iter().collect())
}

impl Parse for Conditional {
    fn parse(input: ParseStream) -> Result<Self> {
        let condition = get_until_question_mark(input)?;
        input.parse::<Token![?]>()?;
        let true_branch = get_until_colon(input)?;
        input.parse::<Token![:]>()?;
        let false_branch: Expr = input.parse()?;

        Ok(Conditional {
            condition,
            true_branch,
            false_branch,
        })
    }
}

pub fn inner(input: TokenStream) -> TokenStream {
    let Conditional {
        condition,
        true_branch,
        false_branch,
    } = parse_macro_input!(input);

    let expanded = quote! {
        {
            if #condition {
                #true_branch
            } else {
                #false_branch
            }
        }
    };
    TokenStream::from(expanded)
}
