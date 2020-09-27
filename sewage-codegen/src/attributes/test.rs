use crate::prelude::*;
use std::collections::HashSet;
use syn::{
    group,
    token::Paren,
    Ident,
    ItemFn,
    Path,
    Token,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum TestAttributeArg {
    Runtime(Path),
}

impl Parse for TestAttributeArg {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let name = ident.to_string().to_ascii_lowercase();
        match name.as_str() {
            "runtime" => {
                if input.peek(Paren) {
                    group::parse_parens(input)
                        .map(|parens| parens.content.parse().map(Self::Runtime))
                        .flatten()
                } else if input.peek(Token![=]) {
                    input.parse::<Token![=]>()?;
                    input.parse().map(Self::Runtime)
                } else {
                    Err(Error::new(input.span(), "Missing arguments"))
                }
            }
            _ => Err(Error::new(
                input.span(),
                format!(r#"Invalid identifier: "{name}""#),
            )),
        }
    }
}

#[derive(Debug, Deref)]
struct TestAttributeArgs(HashSet<TestAttributeArg>);

impl Parse for TestAttributeArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.is_empty() {
            Err(Error::new(input.span(), "Missing arguments"))
        } else {
            let args = input
                .parse_terminated::<_, Token![,]>(TestAttributeArg::parse)?
                .into_pairs()
                .map(|item| item.value().clone())
                .collect();
            Ok(Self(args))
        }
    }
}

#[derive(Debug, Deref)]
struct TestAttributeItem(ItemFn);

impl Parse for TestAttributeItem {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let item: ItemFn = input.parse()?;
        if item.sig.asyncness.is_none() {
            return Err(Error::new_spanned(item.sig, "Must be async"));
        }
        Ok(Self(item))
    }
}

#[derive(Debug)]
pub struct TestAttribute {
    args: TestAttributeArgs,
    item: TestAttributeItem,
}

impl TestAttribute {
    pub fn parse(args: TokenStream, item: TokenStream) -> Self {
        Self {
            args: parse(args).unwrap_or_abort(),
            item: parse(item).unwrap_or_abort(),
        }
    }
}

impl ToTokens for TestAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(quote! {})
    }
}

impl From<TestAttribute> for TokenStream {
    fn from(attribute: TestAttribute) -> Self {
        TokenStream::from(attribute.into_token_stream())
    }
}
