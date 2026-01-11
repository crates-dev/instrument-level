use crate::*;

/// Helper function to parse additional instrument parameters from attributes.
///
/// # Arguments
///
/// - `attr` - The attribute TokenStream containing additional parameters
///
/// # Returns
///
/// - `TokenStream2` - The parsed additional parameters as a TokenStream
fn parse_instrument_params(attr: TokenStream) -> TokenStream2 {
    if attr.is_empty() {
        return quote! {};
    }
    let tokens: TokenStream2 = attr.into();
    quote! { #tokens }
}

/// Macro for adding trace-level instrumentation to functions.
///
/// # Arguments
///
/// - `_attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The function to instrument
///
/// # Returns
///
/// - `TokenStream` - The instrumented function with trace-level logging
pub fn instrument_trace_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: TokenStream2 = parse_instrument_params(_attr);
    let expanded: TokenStream2 = quote! {
        #[::tracing::instrument(level = "trace", #params)]
        #input_fn
    };
    TokenStream::from(expanded)
}

/// Macro for adding debug-level instrumentation to functions.
///
/// # Arguments
///
/// - `_attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The function to instrument
///
/// # Returns
///
/// - `TokenStream` - The instrumented function with debug-level logging
pub fn instrument_debug_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: TokenStream2 = parse_instrument_params(_attr);
    let expanded: TokenStream2 = quote! {
        #[::tracing::instrument(level = "debug", #params)]
        #input_fn
    };
    TokenStream::from(expanded)
}

/// Macro for adding info-level instrumentation to functions.
///
/// # Arguments
///
/// - `_attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The function to instrument
///
/// # Returns
///
/// - `TokenStream` - The instrumented function with info-level logging
pub fn instrument_info_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: TokenStream2 = parse_instrument_params(_attr);
    let expanded: TokenStream2 = quote! {
        #[::tracing::instrument(level = "info", #params)]
        #input_fn
    };
    TokenStream::from(expanded)
}

/// Macro for adding warn-level instrumentation to functions.
///
/// # Arguments
///
/// - `_attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The function to instrument
///
/// # Returns
///
/// - `TokenStream` - The instrumented function with warn-level logging
pub fn instrument_warn_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: TokenStream2 = parse_instrument_params(_attr);
    let expanded: TokenStream2 = quote! {
        #[::tracing::instrument(level = "warn", #params)]
        #input_fn
    };
    TokenStream::from(expanded)
}

/// Macro for adding error-level instrumentation to functions.
///
/// # Arguments
///
/// - `_attr` - Additional tracing instrument parameters (optional): target, name, skip, fields, etc.
/// - `item` - The function to instrument
///
/// # Returns
///
/// - `TokenStream` - The instrumented function with error-level logging
pub fn instrument_error_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let params: TokenStream2 = parse_instrument_params(_attr);
    let expanded: TokenStream2 = quote! {
        #[::tracing::instrument(level = "error", #params)]
        #input_fn
    };
    TokenStream::from(expanded)
}
