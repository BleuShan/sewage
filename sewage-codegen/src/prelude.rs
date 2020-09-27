pub(crate) use proc_macro::TokenStream;
pub(crate) use proc_macro2::TokenStream as TokenStream2;
pub(crate) use proc_macro_error::*;
pub(crate) use quote::{
    quote,
    ToTokens,
    TokenStreamExt as TokenStream2Ext,
};
pub(crate) use shared::{
    result::*,
    std_traits::*,
};

pub(crate) use syn::{
    parse,
    parse::{
        Parse,
        ParseStream,
    },
    Error,
};

pub(crate) type Result<OK, Err = Error> = StdResult<OK, Err>;
