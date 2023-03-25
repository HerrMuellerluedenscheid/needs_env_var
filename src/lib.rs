use proc_macro::TokenStream;
use std::env::var;

/// Skip compilation if the environment variable `env_var` is undefined.
#[proc_macro_attribute]
pub fn needs_env_var(env_var: TokenStream, item: TokenStream) -> TokenStream {
    let var_str = env_var.to_string();
    let exists = var(&var_str).is_ok();

    if !exists{
        println!("\x1b[93mskipped. environment variable: \"{}\" undefined.\x1b[0m", var_str);
        return TokenStream::new()
    }
    item
}