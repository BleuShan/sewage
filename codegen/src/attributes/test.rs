use crate::prelude::*;
use std::collections::HashSet;
use syn::{
    buffer::Cursor,
    parse::StepCursor,
    Ident,
    ItemFn,
    Path,
    Token,
};

fn to_argument_value_tokens<'c, 'a>(
    cursor: StepCursor<'c, 'a>,
) -> Result<(TokenStream2, Cursor<'c>)> {
    let mut current = *cursor;
    while let Some((tt, next)) = current.token_tree() {
        match tt {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                return Ok((group.stream(), next))
            }
            TokenTree::Punct(punct) if punct.as_char() == '=' => match next.token_tree() {
                Some((tokens, tail)) => {
                    let span = tokens.span();
                    let stream = quote_spanned! {span => #tokens};
                    return Ok((stream, tail));
                }
                _ => current = next,
            },
            _ => current = next,
        }
    }
    Err(Error::new_spanned(
        cursor.token_stream(),
        "Missing arguments",
    ))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum TestAttributeArg {
    Runtime(Path),
}

impl Parse for TestAttributeArg {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let name = ident.to_string().to_lowercase();
        if name == "runtime" {
            let args = input.step(to_argument_value_tokens)?;
            let pat = parse2(args)?;
            return Ok(Self::Runtime(pat));
        }

        Err(Error::new_spanned(ident, "Invalid"))
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
    pub fn parse(args: TokenStream, item: TokenStream) -> Result<Self> {
        Ok(Self {
            args: parse(args)?,
            item: parse(item)?,
        })
    }
    cfg_test! {
         pub fn parse2(args: TokenStream2, item: TokenStream2) -> Result<Self> {
            Ok(Self {
                args: parse2(args)?,
                item: parse2(item)?,
            })
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

cfg_test! {
    mod tests {
        use super::*;

        #[test]
        fn test_attribute_should_fail_with_no_args() {
            let args = TokenStream2::new();
            let item = quote! {
                fn test() {}
            };
            let result = TestAttribute::parse2(args, item);
            assert!(result.is_err())
        }

        #[test]
        fn test_attribute_should_fail_with_non_async() {
            let args = quote!{
                runtime(tokio)
            };
            let item = quote! {
                fn test() {}
            };
            let result = TestAttribute::parse2(args, item);
            assert!(result.is_err())
        }
    }
}
