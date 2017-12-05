// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

use {quote, syn};

struct ParseQuotes {
    single: quote::Tokens,
    list: quote::Tokens,
    takes_index: bool,
}

fn decodable_parse_quotes() -> ParseQuotes {
    ParseQuotes {
        single: quote! { rlp.val_at },
        list: quote! { rlp.list_at },
        takes_index: true,
    }
}

fn decodable_wrapper_parse_quotes() -> ParseQuotes {
    ParseQuotes {
        single: quote! { rlp.as_val },
        list: quote! { rlp.as_list },
        takes_index: false,
    }
}

pub fn impl_decodable(ast: &syn::DeriveInput) -> quote::Tokens {
    let body = match ast.body {
        syn::Body::Struct(ref s) => s,
        _ => panic!("#[derive(RlpDecodable)] is only defined for structs."),
    };

    let stmts: Vec<_> = match *body {
        syn::VariantData::Struct(ref fields) | syn::VariantData::Tuple(ref fields) => {
            fields.iter().enumerate().map(decodable_field_map).collect()
        }
        syn::VariantData::Unit => panic!("#[derive(RlpDecodable)] is not defined for Unit structs."),
    };

    let name = &ast.ident;

    let dummy_const = syn::Ident::new(format!("_IMPL_RLP_DECODABLE_FOR_{}", name));
    let impl_block = quote! {
        impl rlp::Decodable for #name {
            fn decode(rlp: &rlp::UntrustedRlp) -> Result<Self, rlp::DecoderError> {
                let result = #name {
                    #(#stmts)*
                };

                Ok(result)
            }
        }
    };

    quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            extern crate rlp;
            #impl_block
        };
    }
}

pub fn impl_decodable_wrapper(ast: &syn::DeriveInput) -> quote::Tokens {
    let body = match ast.body {
        syn::Body::Struct(ref s) => s,
        _ => panic!("#[derive(RlpDecodableWrapper)] is only defined for structs."),
    };

    let stmt = match *body {
        syn::VariantData::Struct(ref fields) | syn::VariantData::Tuple(ref fields) => {
            if fields.len() == 1 {
                let field = fields.first().expect("fields.len() == 1; qed");
                decodable_field(0, field, decodable_wrapper_parse_quotes())
            } else {
                panic!("#[derive(RlpDecodableWrapper)] is only defined for structs with one field.")
            }
        }
        syn::VariantData::Unit => panic!("#[derive(RlpDecodableWrapper)] is not defined for Unit structs."),
    };

    let name = &ast.ident;

    let dummy_const = syn::Ident::new(format!("_IMPL_RLP_DECODABLE_FOR_{}", name));
    let impl_block = quote! {
        impl rlp::Decodable for #name {
            fn decode(rlp: &rlp::UntrustedRlp) -> Result<Self, rlp::DecoderError> {
                let result = #name {
                    #stmt
                };

                Ok(result)
            }
        }
    };

    quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            extern crate rlp;
            #impl_block
        };
    }
}

fn decodable_field_map(tuple: (usize, &syn::Field)) -> quote::Tokens {
    decodable_field(tuple.0, tuple.1, decodable_parse_quotes())
}

fn decodable_field(index: usize, field: &syn::Field, quotes: ParseQuotes) -> quote::Tokens {
    let ident = match field.ident {
        Some(ref ident) => ident.to_string(),
        None => index.to_string(),
    };

    let id = syn::Ident::new(ident);
    let index = syn::Ident::new(index.to_string());

    let single = quotes.single;
    let list = quotes.list;

    match field.ty {
        syn::Ty::Path(_, ref path) => {
            let ident = &path.segments
                .first()
                .expect("there must be at least 1 segment")
                .ident;
            if &ident.to_string() == "Vec" {
                if quotes.takes_index {
                    quote! { #id: #list(#index)?, }
                } else {
                    quote! { #id: #list()?, }
                }
            } else {
                if quotes.takes_index {
                    quote! { #id: #single(#index)?, }
                } else {
                    quote! { #id: #single()?, }
                }
            }
        }
        _ => panic!("rlp_derive not supported"),
    }
}
