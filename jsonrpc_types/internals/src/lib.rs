// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

struct TypeWithAttrs {
    typ: syn::Type,
    attrs: Vec<syn::Attribute>,
}

impl syn::synom::Synom for TypeWithAttrs {
    named!(parse -> Self, do_parse!(
        attrs: many0!(syn::Attribute::parse_outer) >>
        typ: syn!(syn::Type) >>
        (TypeWithAttrs{ typ, attrs })
    ));
}

struct ParamsType {
    name: syn::Ident,
    types: (
        syn::token::Bracket,
        syn::punctuated::Punctuated<TypeWithAttrs, Token![,]>,
    ),
    resp: syn::Ident,
}

impl syn::synom::Synom for ParamsType {
    named!(parse -> Self, do_parse!(
        name: syn!(syn::Ident) >>
        punct!(:) >>
        types: brackets!(call!(syn::punctuated::Punctuated::parse_separated)) >>
        punct!(,) >>
        resp: syn!(syn::Ident) >>
        (ParamsType { name, types, resp })
    ));
}

// Get JSON-RPC name from params name.
// The params name should be `format!("{}Params", capitalize_first(method_name))`.
fn construct_rpcname_from_params_name(params_name: &str) -> syn::LitStr {
    if params_name.len() <= 6 {
        panic!("The name of params [{}] is too short.", params_name);
    }
    if !params_name.ends_with("Params") {
        panic!("Please named the params as: method_name + \"Params\".");
    }
    let rpcname = params_name[..1].to_ascii_lowercase() + &params_name[1..params_name.len() - 6];
    syn::LitStr::new(&rpcname, proc_macro2::Span::call_site())
}

#[proc_macro]
pub fn construct_rpcname(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = {
        let params_name: syn::Ident = syn::parse2(input).unwrap();
        let rpcname = construct_rpcname_from_params_name(params_name.to_string().as_ref());
        quote!(#rpcname)
    };
    output.into()
}

fn generate_attrs_list(attrs_vec: &Vec<syn::Attribute>) -> proc_macro2::TokenStream {
    let mut attrs = quote!();
    for attr in attrs_vec.iter() {
        attrs = quote!(#attrs #attr);
    }
    quote!(#attrs)
}

#[proc_macro]
pub fn construct_params(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    let output = {
        let ParamsType {
            name,
            types: (_, params_types),
            resp,
        } = syn::parse2(input).unwrap();

        let params_size = params_types.len();
        let rpcname = construct_rpcname_from_params_name(name.to_string().as_ref());

        let mut types = quote!();
        let mut params_with_types = quote!();
        let mut params = quote!();
        let mut params_into_vec = quote!();

        match params_size {
            0 => {}
            1 => {
                let TypeWithAttrs { typ, attrs } = &params_types.iter().next().unwrap();
                let param_attrs = generate_attrs_list(attrs);
                types = quote!(#param_attrs pub #typ, #[serde(skip)] OneItemTupleTrick);
                params_with_types = quote!(param: #typ);
                params = quote!(param, OneItemTupleTrick::default());
                params_into_vec = quote!(serde_json::to_value(self.0).unwrap())
            }
            _ => {
                let mut param_num = 0;
                for TypeWithAttrs { typ, attrs } in params_types.iter() {
                    let param_attrs = generate_attrs_list(attrs);
                    let param_name = format!("p{}", param_num);
                    let param_name = syn::Ident::new(&param_name, proc_macro2::Span::call_site());
                    types = quote!(#types #param_attrs pub #typ,);
                    params_with_types = quote!(#params_with_types #param_name: #typ,);
                    params = quote!(#params #param_name,);
                    params_into_vec = quote!(
                        #params_into_vec
                        serde_json::to_value(self.#param_num).unwrap(), );
                    param_num += 1;
                }
            }
        };

        quote!(
            #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
            pub struct #name (#types);

            impl #name {
                pub fn new(#params_with_types) -> #name {
                    #name(#params)
                }
            }

            impl JsonRpcRequest for #name {

                type Response = #resp;

                fn required_len() -> usize {
                    #params_size
                }

                fn method_name(&self) -> &'static str {
                    #rpcname
                }

                fn value_vec(self) -> Vec<serde_json::Value> {
                    vec![#params_into_vec]
                }
            }
        )
    };

    output.into()
}
