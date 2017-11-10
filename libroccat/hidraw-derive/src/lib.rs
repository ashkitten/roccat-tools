#![recursion_limit = "128"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{Body, DeriveInput, Expr, Field, Ident, Lit, MetaItem, Ty, VariantData};

#[proc_macro_derive(HidrawRead, attributes(hidraw_constant, hidraw_bytesum))]
pub fn derive_hid_read(input: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&input.to_string()).unwrap();
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

            ioctl!(readwrite __hidraw_read with b'H', 0x07; Self);

            pub unsafe fn read(interface: &::std::fs::File) -> Result<Self> {
                use std::os::unix::io::AsRawFd;

                let mut data = Self {
                    #(#const_field_names: #const_field_vals,)*
                    .. ::std::mem::uninitialized()
                };

                let mut errors = 0;
                loop {
                    match Self::__hidraw_read(interface.as_raw_fd(), &mut data) {
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

    output.parse().unwrap()
}

#[proc_macro_derive(HidrawWrite, attributes(hidraw_constant, hidraw_bytesum))]
pub fn derive_hid_write(input: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&input.to_string()).unwrap();
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
            ioctl!(readwrite __hidraw_write with b'H', 0x06; Self);

            pub unsafe fn write(self, interface: &::std::fs::File) -> Result<()> {
                use std::os::unix::io::AsRawFd;

                let mut data = Self {
                    #(#const_field_names: #const_field_vals,)*
                    .. self
                };

                #assign_bytesum

                let mut errors = 0;
                loop {
                    match Self::__hidraw_write(interface.as_raw_fd(), &mut data) {
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

fn get_bytesum_field(input: &DeriveInput) -> Option<(Ident, Ty)> {
    match input.body {
        Body::Struct(VariantData::Struct(ref fields)) => {
            let last = fields.last().unwrap();
            for attr in &last.attrs {
                if attr.name() == "hidraw_bytesum" {
                    return Some((
                        last.ident
                            .as_ref()
                            .expect("Only named fields are supported")
                            .clone(),
                        last.ty.clone(),
                    ));
                }
            }
        }
        _ => panic!("Hidraw derive only supports structs"),
    }
    return None;
}
