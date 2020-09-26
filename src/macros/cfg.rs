//! Cfg attribute macros.

#![allow(unused_macros)]

macro_rules! cfg_native {
       ($($item:item)*) => {
        $(
            #[cfg(not(target_arch = "wasm32"))]
            #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
            $item
        )*
    }
}

macro_rules! cfg_wasm {
       ($($item:item)*) => {
        $(
            #[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
            #[cfg_attr(docsrs, doc(cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))))]
            $item
        )*
    }
}
