//! Custom derive support for `zeroize`

#![crate_type = "proc-macro"]
#![deny(warnings, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use synstructure::{decl_derive, BindStyle};

/// Name of zeroize-related attributes
const ZEROIZE_ATTR: &str = "zeroize";

/// Custom derive for `Zeroize`
fn derive_zeroize(s: synstructure::Structure) -> TokenStream {
    let attributes = ZeroizeDeriveAttrs::parse(&s);

    match attributes.drop {
        Some(true) => derive_zeroize_with_drop(s),
        Some(false) => derive_zeroize_without_drop(s),
        None => panic!("must specify either zeroize(drop) or zeroize(no_drop) attribute"),
    }
}
decl_derive!([Zeroize, attributes(zeroize)] => derive_zeroize);

/// Custom derive attributes for `Zeroize`
struct ZeroizeDeriveAttrs {
    /// Derive a `Drop` impl which calls zeroize on this type
    drop: Option<bool>,
}

impl Default for ZeroizeDeriveAttrs {
    fn default() -> Self {
        Self { drop: None }
    }
}

impl ZeroizeDeriveAttrs {
    /// Parse attributes from the incoming AST
    fn parse(s: &synstructure::Structure) -> Self {
        let mut result = Self::default();

        for v in s.variants().iter() {
            for attr in v.ast().attrs.iter() {
                if attr.path.is_ident(ZEROIZE_ATTR) {
                    // TODO(tarcieri): hax, but probably good enough for now
                    match attr.tts.to_string().as_ref() {
                        "( drop )" => result.drop = Some(true),
                        "( no_drop )" => result.drop = Some(false),
                        other => panic!("unknown zeroize attribute: {}", other),
                    }
                }
            }
        }

        result
    }
}

/// Custom derive for `Zeroize` (without `Drop`)
fn derive_zeroize_without_drop(mut s: synstructure::Structure) -> TokenStream {
    s.bind_with(|_| BindStyle::RefMut);

    let zeroizers = s.each(|bi| quote! { #bi.zeroize(); });

    s.bound_impl(
        quote!(zeroize::Zeroize),
        quote! {
            fn zeroize(&mut self) {
                match self {
                    #zeroizers
                }
            }
        },
    )
}

/// Custom derive for `Zeroize` and `Drop`
fn derive_zeroize_with_drop(s: synstructure::Structure) -> TokenStream {
    let drop_impl = s.gen_impl(quote! {
        gen impl Drop for @Self {
            fn drop(&mut self) {
                self.zeroize();
            }
        }
    });

    let zeroize_impl = derive_zeroize_without_drop(s);

    quote! {
        #zeroize_impl

        #[doc(hidden)]
        #drop_impl
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use synstructure::test_derive;

    #[test]
    fn zeroize_without_drop() {
        test_derive! {
            derive_zeroize_without_drop {
                struct Z {
                    a: String,
                    b: Vec<u8>,
                    c: [u8; 3],
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                const _DERIVE_zeroize_Zeroize_FOR_Z: () = {
                    extern crate zeroize;
                    impl zeroize::Zeroize for Z {
                        fn zeroize(&mut self) {
                            match self {
                                Z {
                                    a: ref mut __binding_0,
                                    b: ref mut __binding_1,
                                    c: ref mut __binding_2,
                                } => {
                                    { __binding_0.zeroize(); }
                                    { __binding_1.zeroize(); }
                                    { __binding_2.zeroize(); }
                                }
                            }
                        }
                    }
                };
            }
            no_build // tests the code compiles are in the `zeroize` crate
        }
    }

    #[test]
    fn zeroize_with_drop() {
        test_derive! {
            derive_zeroize_with_drop {
                struct Z {
                    a: String,
                    b: Vec<u8>,
                    c: [u8; 3],
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                const _DERIVE_zeroize_Zeroize_FOR_Z: () = {
                    extern crate zeroize;
                    impl zeroize::Zeroize for Z {
                        fn zeroize(&mut self) {
                            match self {
                                Z {
                                    a: ref mut __binding_0,
                                    b: ref mut __binding_1,
                                    c: ref mut __binding_2,
                                } => {
                                    { __binding_0.zeroize(); }
                                    { __binding_1.zeroize(); }
                                    { __binding_2.zeroize(); }
                                }
                            }
                        }
                    }
                };
                #[doc(hidden)]
                #[allow(non_upper_case_globals)]
                const _DERIVE_Drop_FOR_Z: () = {
                    impl Drop for Z {
                        fn drop(&mut self) {
                            self.zeroize();
                        }
                    }
                };
            }
            no_build // tests the code compiles are in the `zeroize` crate
        }
    }
}
