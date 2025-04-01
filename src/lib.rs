use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemFn, LitStr,
};

struct InputArgs {
    environment_variable: LitStr,
}

impl Parse for InputArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let environment_variable = input.parse::<LitStr>()?;
        Ok(Self {
            environment_variable,
        })
    }
}

#[proc_macro_attribute]
pub fn needs_env_var(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let input_args = parse_macro_input!(attr as InputArgs);
    let environment_variable = input_args.environment_variable.value().replace(" ", "");

    let (env_var, expected_value): (String, Option<String>) =
        match environment_variable.split_once("=") {
            Some((key, value)) => (key.to_string(), Some(value.to_string())),
            None => (environment_variable.clone(), None),
        };

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let expanded = quote! {
        #(#attrs)* #vis #sig {
            match std::env::var(#env_var) {
                Err(_) => {eprintln!("\x1b[93mSkipping test because environment variable {} is not set.\x1b[0m", #env_var);
                    return;
                }
                Ok(value) => {
                    if value != #expected_value {
                        eprintln!("\x1b[93mSkipping test because environment variable {}={}. Expected: {}.\x1b[0m", #env_var, value, #expected_value);
                        return;
                    }
                }
            };
            #block
        }
    };

    TokenStream::from(expanded)
}
