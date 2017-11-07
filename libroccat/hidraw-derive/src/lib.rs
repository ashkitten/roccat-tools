#![recursion_limit = "128"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Body, DeriveInput, Expr, Field, Ident, Lit, MetaItem, VariantData};

#[proc_macro_derive(HidrawRead, attributes(hidraw_constant))]
pub fn derive_hid_read(input: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&input.to_string()).unwrap();
    let name = &input.ident;
    let (const_field_names, const_field_vals) = get_const_fields(&input);

    let output = quote! {
        impl #name {

            ioctl!(readwrite __hidraw_read with b'H', 0x07; #name);

            pub unsafe fn read(interface: &::std::fs::File) -> Result<Self> {
                use std::os::unix::io::AsRawFd;
                use nix::{Error, Errno};

                let mut data = Self {
                    #(#const_field_names: #const_field_vals,)*
                    .. ::std::mem::uninitialized()
                };

                let mut errors = 0;
                while errors < 10 {
                    match Self::__hidraw_read(interface.as_raw_fd(), &mut data) {
                        Ok(_) => break,
                        Err(Error::Sys(Errno::EINTR)) => errors += 1,
                        Err(Error::Sys(Errno::EAGAIN)) => errors += 1,
                        Err(Error::Sys(Errno::ETIMEDOUT)) => errors += 1,
                        Err(other) => return Err(other.into()),
                    }
                }

                Ok(data)
            }

        }
    };

    output.parse().unwrap()
}

#[proc_macro_derive(HidrawWrite, attributes(hidraw_constant))]
pub fn derive_hid_write(input: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&input.to_string()).unwrap();
    let name = &input.ident;

    let output = quote! {
        impl #name {
            ioctl!(readwrite __hidraw_write with b'H', 0x06; #name);

            pub unsafe fn write(mut self, interface: &::std::fs::File) -> Result<()> {
                use std::os::unix::io::AsRawFd;
                use nix::{Error, Errno};

                let mut errors = 0;
                while errors < 10 {
                    match Self::__hidraw_write(interface.as_raw_fd(), &mut self) {
                        Ok(_) => break,
                        Err(Error::Sys(Errno::EINTR)) => errors += 1,
                        Err(Error::Sys(Errno::EAGAIN)) => errors += 1,
                        Err(Error::Sys(Errno::ETIMEDOUT)) => errors += 1,
                        Err(other) => return Err(other.into()),
                    }
                }

                Ok(())
            }

        }
    };

    output.parse().unwrap()
}

fn get_const_fields(input: &DeriveInput) -> (Vec<Ident>, Vec<Expr>) {
    match input.body {
        Body::Struct(VariantData::Struct(ref fields)) => {
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
        .flat_map(|attr| match attr.value {
            MetaItem::NameValue(ref ident, ref lit) if ident == "hidraw_constant" => match *lit {
                Lit::Str(ref str, _) => {
                    let expr = syn::parse_expr(str).unwrap();
                    Some((name.clone(), expr))
                }
                _ => panic!("hidraw: Unsupported constant literal"),
            },
            _ => None,
        })
        .next()
}
