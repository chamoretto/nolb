use proc_macro::TokenStream;

/// Generate `extern "C"` wrappers for all of the functions of the `nolb_plugin::Plugin` trait
///
/// # Example
/// ```
/// use nolb_plugin::{wrap_plugin, Provider};
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
/// use nolb_plugin::{wrap_plugin, Provider};
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
