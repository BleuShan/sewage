pub(crate) use proc_macro::{
    Diagnostic,
    Level,
    TokenStream,
};
pub(crate) use proc_macro2::{
    Delimiter,
    Span,
    TokenStream as TokenStream2,
    TokenTree,
};
pub(crate) use quote::{
    quote,
    quote_spanned,
    ToTokens,
    TokenStreamExt as TokenStream2Ext,
};
pub(crate) use shared::{
    cfg_test,
    std_traits::*,
};
use std::result::Result as StdResult;
pub(crate) use syn::{
    parse,
    parse::{
        Parse,
        ParseStream,
    },
    parse2,
    Error,
};

pub(crate) type Result<OK, Err = Error> = StdResult<OK, Err>;
