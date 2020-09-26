use crate::Result;
use once_cell::sync::OnceCell;
use proc_macro::{
    quote,
    TokenStream,
};
use syn::{
    parse,
    parse::{
        Parse,
        ParseStream,
    },
    Error,
    ItemFn,
};

#[derive(Debug)]
pub struct TestAttributeArgs {}

#[derive(Debug)]
pub struct TestAttribute {
    item: ItemFn,
    args: OnceCell<TestAttributeArgs>,
}

impl TestAttribute {
    fn new(item: ItemFn) -> Self {
        Self {
            item,
            args: OnceCell::new(),
        }
    }

    pub fn with_args(self, args: TestAttributeArgs) -> Self {
        self.args.set(args).ok();
        self
    }
}

impl Parse for TestAttributeArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Err(input.error("Failed"))
    }
}

impl Parse for TestAttribute {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let item: ItemFn = ItemFn::parse(input)?;
        Ok(Self::new(item))
    }
}

impl From<TestAttribute> for TokenStream {
    fn from(attribute: TestAttribute) -> Self {
        let TestAttribute { item, args } = attribute;
        match args.get() {
            Some(args) => {
                let attrs = item.attrs;

                quote! {}
            }
            None => quote! {},
        }
    }
}
