//! Conditional string literal generation

/// Define a macro that produces a string literal whose contents is revealed
/// and masked based on the current build configuration (`cfg!`).
macro_rules! pp_text_macro {
    (
        $vis:vis macro $name:ident {
            $($text:tt)*
        }
    ) => {
        pp_text_macro! {
            @internal,
            text: [{ $($text)* }],
            free_idents: [{
                __pp_01 __pp_02 __pp_03 __pp_04
                __pp_05 __pp_06 __pp_07 __pp_08
                __pp_09 __pp_10 __pp_11 __pp_12
                __pp_13 __pp_14 __pp_15 __pp_16
            }],
            private_mod: [{}],
            macro_inner: [{}],
            define_macro: [{ $vis $name }],
        }
    };
    // -----------------------------------------------------------------
    // Each step processes the first element in the given `text`. This continues
    // until none are left.
    (
        @internal,
        // The text being munched
        text: [{
            $text:literal
            $($rest_text:tt)*
        }],
        // A pool of unique identifiers.
        free_idents: [{ $($free_idents:tt)* }],
        // The contents of the generated private module.
        private_mod: [{ $($private_mod:tt)* }],
        // The contents of the generated macro..
        macro_inner: [{ $($macro_inner:tt)* }],
        define_macro: [$define_macro:tt],
    ) => {
        pp_text_macro! {
            @internal,
            text: [{ $($rest_text)* }],
            free_idents: [{ $($free_idents)* }],
            private_mod: [{ $($private_mod)* }],
            macro_inner: [{
                $($macro_inner)*

                $text,
            }],
            define_macro: [$define_macro],
        }
    };

    (
        @internal,
        text: [{
            if cfg!( $($cfg:tt)* ) {
                $($true_text:tt)*
            } else {
                $($false_text:tt)*
            }
            $($rest_text:tt)*
        }],
        free_idents: [{ $free_ident:ident $($rest_free_idents:tt)* }],
        private_mod: [{ $($private_mod:tt)* }],
        macro_inner: [{ $($macro_inner:tt)* }],
        define_macro: [{ $vis:vis $name:ident }],
    ) => {
        pp_text_macro! {
            @internal,
            text: [{ $($rest_text)* }],
            free_idents: [{ $($rest_free_idents)* }],
            private_mod: [{
                $($private_mod)*

                #[cfg($($cfg)*)]
                pp_text_macro! {
                    pub(super) macro $free_ident { $($true_text)* }
                }

                #[cfg(not($($cfg)*))]
                pp_text_macro! {
                    pub(super) macro $free_ident { $($false_text)* }
                }
            }],
            macro_inner: [{
                $($macro_inner)*

                // Refers to the macro in the generated private module
                $name::$free_ident!(),
            }],
            define_macro: [{ $vis $name }],
        }
    };

    (
        @internal,
        text: [{
            if cfg!( $($cfg:tt)* ) {
                $($true_text:tt)*
            }
            $($rest_text:tt)*
        }],
        free_idents: [{ $free_ident:ident $($rest_free_idents:tt)* }],
        private_mod: [{ $($private_mod:tt)* }],
        macro_inner: [{ $($macro_inner:tt)* }],
        define_macro: [{ $vis:vis $name:ident }],
    ) => {
        pp_text_macro! {
            @internal,
            text: [{ $($rest_text)* }],
            free_idents: [{ $($rest_free_idents)* }],
            private_mod: [{
                $($private_mod)*

                #[cfg($($cfg)*)]
                pp_text_macro! {
                    pub(super) macro $free_ident { $($true_text)* }
                }

                #[cfg(not($($cfg)*))]
                pub(super) macro $free_ident () { "" }
            }],
            macro_inner: [{
                $($macro_inner)*

                // Refers to the macro in the generated private module
                $name::$free_ident!(),
            }],
            define_macro: [{ $vis $name }],
        }
    };

    (
        @internal,
        text: [{}],
        free_idents: [{ $free_ident:ident $($rest_free_idents:tt)* }],
        private_mod: [{ $($private_mod:tt)* }],
        macro_inner: [{ $($macro_inner:tt)* }],
        define_macro: [{ $vis:vis $name:ident }],
    ) => {
        // No more text to munch, define the macro
        mod $name { $($private_mod)* }
        $vis macro $name () { concat!( $($macro_inner)* ) }
    };
}

/// Preprocessed `llvm_asm!`.
macro_rules! pp_asm {
    // -------------------------------------------------------------------
    // Munch the input until `:` or EOF is found
    (
        @internal,
        unprocessed: [{ : $($unprocessed:tt)* }],
        code: [{ $($code:tt)* }],
    ) => {{
        pp_asm!(
            @done,
            unprocessed: [{ : $($unprocessed)* }],
            code: [{ $($code)* }],
        );
    }};
    (
        @internal,
        unprocessed: [{}],
        code: [{ $($code:tt)* }],
    ) => {{
        pp_asm!(
            @done,
            unprocessed: [{}],
            code: [{ $($code)* }],
        );
    }};

    (
        @internal,
        unprocessed: [{ $fragment:tt $($unprocessed:tt)* }],
        code: [{ $($code:tt)* }],
    ) => {{
        pp_asm!(
            @internal,
            unprocessed: [{ $($unprocessed)* }],
            code: [{ $($code)* $fragment }],
        );
    }};
    // -------------------------------------------------------------------
    (
        @done,
        unprocessed: [{ $($unprocessed:tt)* }],
        code: [{ $($code:tt)* }],
    ) => {{
        pp_text_macro! {
            macro pp_asm_code { $($code)* }
        }
        llvm_asm!(pp_asm_code!() $($unprocessed)*);
    }};
    // -------------------------------------------------------------------
    // The entry point
    (
        // TODO: remove `$l:lit`
        $l:literal $($rest:tt)*
    ) => {
        pp_asm!(
            @internal,
            unprocessed: [{ $l $($rest)* }],
            code: [{}],
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pp_text() {
        pp_text_macro!(
            macro got {
                "hello"
                "foo1"
                if cfg!(any()) { "foo2" } else { "bar2" }
                if cfg!(any()) { "foo3" }
                if cfg!(all()) { "foo4" } else { "bar4" }
                if cfg!(all()) { "foo5" }

                if cfg!(any()) {
                    "foo6"
                    if cfg!(any()) { "hoge1" } else { "piyo1" }
                    if cfg!(all()) { "hoge2" } else { "piyo2" }
                    "foo7"
                } else {
                    "foo8"
                    if cfg!(any()) { "hoge3" } else { "piyo3" }
                    if cfg!(all()) { "hoge4" } else { "piyo4" }
                    "foo9"
                }
                if cfg!(any()) {
                    "foo10"
                    if cfg!(any()) { "hoge5" } else { "piyo5" }
                    if cfg!(all()) { "hoge6" } else { "piyo6" }
                    "foo11"
                }
                if cfg!(all()) {
                    "foo12"
                    if cfg!(any()) { "hoge7" } else { "piyo7" }
                    if cfg!(all()) { "hoge8" } else { "piyo8" }
                    "foo13"
                }
            }
        );

        let got: &str = got!();
        let expected = {
            extern crate std;
            use std::borrow::ToOwned;
            let mut x = "hellofoo1".to_owned();
            x += if cfg!(any()) { "foo2" } else { "bar2" };
            x += if cfg!(any()) { "foo3" } else { "" };
            x += if cfg!(all()) { "foo4" } else { "bar4" };
            x += if cfg!(all()) { "foo5" } else { "" };

            if cfg!(any()) {
                x += "foo6";
                x += if cfg!(any()) { "hoge1" } else { "piyo1" };
                x += if cfg!(all()) { "hoge2" } else { "piyo2" };
                x += "foo7";
            } else {
                x += "foo8";
                x += if cfg!(any()) { "hoge3" } else { "piyo3" };
                x += if cfg!(all()) { "hoge4" } else { "piyo4" };
                x += "foo9";
            }
            if cfg!(any()) {
                x += "foo10";
                x += if cfg!(any()) { "hoge5" } else { "piyo5" };
                x += if cfg!(all()) { "hoge6" } else { "piyo6" };
                x += "foo11";
            }
            if cfg!(all()) {
                x += "foo12";
                x += if cfg!(any()) { "hoge7" } else { "piyo7" };
                x += if cfg!(all()) { "hoge8" } else { "piyo8" };
                x += "foo13";
            }

            x
        };

        assert_eq!(got, expected);
    }
}
