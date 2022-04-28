mod ternary_if;

use proc_macro::TokenStream;

/// Generate `extern "C"` wrappers for all of the functions of the `nolb_plugin::Plugin` trait
///
/// TODO: Remove should_panic from both code blocks when plugins would be ready
/// # Example
/// ```compile_fail
/// use nolb_codegen::wrap_plugin;
///
/// # pub trait Provider {
/// #     fn function(var: i64) -> u64;
/// # }
///
/// struct ProviderName;
///
/// #[wrap_plugin]
/// impl Provider for ProviderName {
///     fn function(var: i64) -> u64 {
///         var as u64
///     }
/// }
/// ```
///
/// This will expand to something like:
/// ```
/// use nolb_codegen::wrap_plugin;
///
/// # pub trait Provider {
/// #     fn function(var: i64) -> u64;
/// # }
///
/// struct ProviderName;
///
/// impl Provider for ProviderName {
///     fn function(var: i64) -> u64 {
///         var as u64
///     }
/// }
///
/// #[no_mangle]
/// pub extern "C" fn function(var: i64) -> u64 {
///     <ProviderName as Provider>::function(var) // here may be added needed conversions to ffi-safe types
/// }
/// ```
#[proc_macro_attribute]
pub fn wrap_plugin(_args: TokenStream, _input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro]
pub fn ternary(input: TokenStream) -> TokenStream {
    ternary_if::inner(input)
}
