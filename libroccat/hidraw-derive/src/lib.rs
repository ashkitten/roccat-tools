#![recursion_limit = "128"]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Expr, Field, Ident, Lit, Meta, MetaNameValue, Type};

#[proc_macro_derive(HidrawRead, attributes(hidraw_constant, hidraw_bytesum))]
pub fn derive_hid_read(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;
    let (const_field_names, const_field_vals) = get_const_fields(&input);
    let check_bytesum =
        if let Some((bytesum_field_name, bytesum_field_type)) = get_bytesum_field(&input) {
            quote! {
                {
                    const BYTES_SIZE: usize = ::std::mem::size_of::<#name>() -
                        ::std::mem::size_of::<#bytesum_field_type>();
                    let bytes: [u8; BYTES_SIZE] = ::std::mem::transmute_copy(&data);
                    if data.#bytesum_field_name ==
                        bytes.iter().map(|b| *b as #bytesum_field_type).sum()
                    {
                        return Ok(data)
                    }
                }
            }
        } else {
            quote!(return Ok(data))
        };

    let output = quote! {
        impl #name {
            pub unsafe fn read(interface: &::std::fs::File) -> Result<#name, ::failure::Error> {
                nix::ioctl_readwrite!(__hidraw_read, b'H', 0x07, #name);

                use std::os::unix::io::AsRawFd;

                let mut data = #name {
                    #(#const_field_names: #const_field_vals,)*
                    .. ::std::mem::uninitialized()
                };

                let mut errors = 0;
                loop {
                    match __hidraw_read(interface.as_raw_fd(), &mut data) {
                        Ok(_) => #check_bytesum,
                        Err(error) => {
                            if errors < 10 {
                                errors += 1;
                            } else {
                                return Err(error.into());
                            }
                        }
                    }
                }
            }
        }
    };

    output.into()
}

#[proc_macro_derive(HidrawWrite, attributes(hidraw_constant, hidraw_bytesum))]
pub fn derive_hid_write(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;
    let (const_field_names, const_field_vals) = get_const_fields(&input);
    let assign_bytesum =
        if let Some((bytesum_field_name, bytesum_field_type)) = get_bytesum_field(&input) {
            quote! {
                const BYTES_SIZE: usize = ::std::mem::size_of::<#name>() -
                    ::std::mem::size_of::<#bytesum_field_type>();
                let bytes: [u8; BYTES_SIZE] = ::std::mem::transmute_copy(&data);
                data.#bytesum_field_name = bytes.iter().map(|b| *b as #bytesum_field_type).sum();
            }
        } else {
            quote!()
        };

    let output = quote! {
        impl #name {
            pub unsafe fn write(self, interface: &::std::fs::File) -> Result<(), ::failure::Error> {
                nix::ioctl_readwrite!(__hidraw_write, b'H', 0x06, #name);

                use std::os::unix::io::AsRawFd;

                let mut data = #name {
                    #(#const_field_names: #const_field_vals,)*
                    .. self
                };

                #assign_bytesum

                let mut errors = 0;
                loop {
                    match __hidraw_write(interface.as_raw_fd(), &mut data) {
                        Ok(_) => return Ok(()),
                        Err(error) => {
                            if errors < 10 {
                                errors += 1;
                            } else {
                                return Err(error.into());
                            }
                        }
                    }
                }
            }
        }
    };

    output.into()
}

fn get_const_fields(input: &DeriveInput) -> (Vec<Ident>, Vec<Expr>) {
    match input.data {
        Data::Struct(DataStruct { ref fields, .. }) => {
            fields.iter().flat_map(get_const_field).unzip()
        }
        _ => panic!("Hidraw derive only supports structs"),
    }
}

fn get_const_field(field: &Field) -> Option<(Ident, Expr)> {
    let name = field
        .ident
        .as_ref()
        .expect("hidraw: Only named fields are supported");

    field
        .attrs
        .iter()
        .flat_map(|attr| match attr.interpret_meta() {
            Some(Meta::NameValue(MetaNameValue {
                ref ident, ref lit, ..
            })) if ident == "hidraw_constant" => match *lit {
                Lit::Str(ref lit_str) => {
                    let expr = syn::parse_str(&lit_str.value()).unwrap();
                    Some((name.clone(), expr))
                }
                _ => panic!("hidraw: Unsupported constant literal"),
            },
            _ => None,
        })
        .next()
}

fn get_bytesum_field(input: &DeriveInput) -> Option<(Ident, Type)> {
    match input.data {
        Data::Struct(DataStruct { ref fields, .. }) => {
            let last = fields.iter().last().unwrap();
            for attr in &last.attrs {
                match attr.interpret_meta() {
                    Some(Meta::Word(ident)) => {
                        if ident == "hidraw_bytesum" {
                            return Some((
                                last.ident
                                    .as_ref()
                                    .expect("Only named fields are supported")
                                    .clone(),
                                last.ty.clone(),
                            ));
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => panic!("Hidraw derive only supports structs"),
    }
    return None;
}
