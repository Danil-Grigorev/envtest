use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, Signature};

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct EnvtestArgs {
    #[darling(default)]
    environment: Option<syn::Expr>,
}

#[proc_macro_attribute]
pub fn envtest(attr: TokenStream, item: TokenStream) -> TokenStream {
    match expand_envtest(attr, item) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_envtest(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let args = syn::parse::<EnvtestArgs>(attr)?;
    let input = syn::parse::<ItemFn>(item)?;

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;
    let name = sig.ident.clone();
    let inner_name = format_ident!("__envtest_inner_{}", name);
    let outer_sig = outer_signature(sig.clone());
    let inner_sig = inner_signature(sig.clone(), inner_name.clone());
    let environment = args
        .environment
        .unwrap_or_else(|| syn::parse_quote!(::envtest::Environment::default()));

    let result = quote! {
        #(#attrs)*
        #vis #outer_sig {
            #vis #inner_sig #block

            let server = (#environment)
                .create()
                .await
                .expect("failed to create envtest environment");
            let client = server
                .client()
                .expect("failed to create kube::Client from envtest environment");

            #inner_name(client).await
        }
    };

    Ok(result.into())
}

fn outer_signature(mut sig: Signature) -> Signature {
    sig.inputs.clear();
    sig
}

fn inner_signature(mut sig: Signature, ident: syn::Ident) -> Signature {
    sig.ident = ident;
    sig
}
