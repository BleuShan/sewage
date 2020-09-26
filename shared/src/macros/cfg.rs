//! Cfg attribute macros.

/// Defines a block of code for a native target
#[macro_export]
macro_rules! cfg_native {
       ($($item:item)*) => {
        $(
            #[cfg(not(target_arch = "wasm32"))]
            #[cfg_attr(docsrs, doc(cfg(not(target_arch = "wasm32"))))]
            $item
        )*
    }
}

/// Defines a block of code for a wasm target
#[macro_export]
macro_rules! cfg_wasm {
       ($($item:item)*) => {
        $(
            #[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
            #[cfg_attr(docsrs, doc(cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))))]
            $item
        )*
    }
}

/// Defines a block of code for a test target
#[macro_export]
macro_rules! cfg_test {
       ($($item:item)*) => {
        $(
            #[cfg(test)]
            #[cfg_attr(docsrs, doc(cfg(test)))]
            $item
        )*
    }
}
