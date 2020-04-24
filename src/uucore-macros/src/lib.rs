extern crate proc_macro;

// spell-checker:ignore () sigpipe uucore uumain

// ref: <https://dev.to/naufraghi/procedural-macro-in-rust-101-k3f> @@ <http://archive.is/Vbr5e>
// ToDO: add path construction from LitStr as <https://oschwald.github.io/maxminddb-rust/syn/struct.LitStr.html> @@ <http://archive.is/8YDua>

struct Tokens {
    expr: syn::Expr,
}

impl syn::parse::Parse for Tokens {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Tokens { expr: input.parse()? })
    }
}

#[proc_macro]
pub fn make_main(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Tokens { expr } = syn::parse_macro_input!(stream as Tokens);
    println!("expr={:?}", expr);
    let expr = match expr {
        syn::Expr::Lit(expr) => {
            println!("found Expr::Lit => {:?}", expr);
            match expr.lit {
                syn::Lit::Str(ref lit) => {
                    let mut s = lit.value();
                    if !s.ends_with("::uumain") { s += "::uumain"; }
                    syn::LitStr::new(&s, proc_macro2::Span::call_site()).parse().unwrap()
                },
                _ => panic!(),
            }
        },
        syn::Expr::Path(expr) => {
            println!("found Expr::Path => {:?}", expr);
            let i = &expr.path.segments.last().unwrap().ident;
            println!("... i => {:?}", i);
            if i.to_string() != "uumain" {
                println!("not equal");
                syn::parse_quote!( #expr::uumain )
            } else {
                expr
            }
        },
        _ => panic!(),
    };
    let f = quote::quote!{ #expr(uucore::args().collect()) };
    println!("f = {:?}", f);
    let result = quote::quote! {
        fn main() {
            use std::io::Write;
            uucore::panic::install_sigpipe_hook();
            let code = #f;
            std::io::stdout().flush().expect("could not flush stdout");
            std::process::exit(code);
        }
    };
    proc_macro::TokenStream::from(result)
}
