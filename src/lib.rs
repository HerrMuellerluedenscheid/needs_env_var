use proc_macro::TokenStream;
use std::env::var;


/// Skip compilation if the environment variable `env_var` is undefined or if its content does not
/// match the provided value.
#[proc_macro_attribute]
pub fn needs_env_var(env_var: TokenStream, input: TokenStream) -> TokenStream {
    let macro_string = env_var.to_string().replace(" = ", "=");
    let mut parts = macro_string.split('=');

    let var_str = parts.next().expect("macro needs an environment variable name");
    let matches  = parts.next();

    let var_content = var(var_str);
    let exists = var_content.is_ok();

    if !exists || matches.is_some() && matches != var_content.ok().as_deref() {
        println!("\x1b[93mskipped. environment variable: \"{}\" did not match or was not set.\x1b[0m", var_str);
        return TokenStream::new();
    }
    input
}
