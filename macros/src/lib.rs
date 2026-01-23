// Copyright © 2025 Stephan Kunz
//! Macros for port and port collection creation.

#![allow(unused, dead_code)]

//use darling::FromMeta; // needed for Debug!
use proc_macro::TokenStream;
use quote::quote;
use syn::{
	Expr, Ident, LitStr, Result, Token, Type, TypeGroup, TypeParam,
	parse::{Parse, ParseStream},
	parse_macro_input,
};

#[doc(hidden)]
extern crate proc_macro;

/// Creates an array of ports.
/// # Usage:
/// port_array!(<comma_separated_list_of_name/port_pairs>)
///
/// Best to use with port generation macros:
/// port_array!(
///     inbound!(<name>, <type>, <value>),
///     outbound!(<name>, <type>),
///     inoutbound!(<name>. <value>)
/// )
#[proc_macro]
pub fn port_array(input: TokenStream) -> TokenStream {
	let output: proc_macro2::TokenStream = input.into();
	quote! {
		dataport::PortArray::from([#output])
	}
	.into()
}

/// Creates a list of ports.
/// # Usage:
/// port_list!(<comma_separated_list_of_name/port_pairs>)
///
/// Best to use with port generation macros:
/// port_list!(
///     inoutbound!(<name>, <type>, <value>),
///     inbound!(<name>, <type>),
///     outbound!(<name>. <value>)
/// )
#[proc_macro]
pub fn port_list(input: TokenStream) -> TokenStream {
	let output: proc_macro2::TokenStream = input.into();
	quote! {
		dataport::PortList::from([#output])
	}
	.into()
}

/// Creates a map of ports.
/// # Usage:
/// port_map!(<comma_separated_list_of_name/port_pairs>)
///
/// Best to use with port generation macros:
/// port_map!(
///     outbound!(<name>, <type>, <value>),
///     inoutbound!(<name>, <type>),
///     inbound!(<name>. <value>)
/// )
#[proc_macro]
pub fn port_map(input: TokenStream) -> TokenStream {
	let output: proc_macro2::TokenStream = input.into();
	quote! {
		dataport::PortMap::from([#output])
	}
	.into()
}

struct Params {
	port_name: proc_macro2::TokenStream,
	port_type: Option<Type>,
	port_value: Option<Expr>,
}

impl Parse for Params {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.is_empty() {
			panic!("macro needs at least two comma separated parameters");
		}

		let old = input;
		let port_name = if let Ok(name) = input.parse::<LitStr>() {
			quote! {#name}
		} else {
			let value = old.parse::<Ident>()?;
			quote! {#value}
		};
		input.parse::<Token![,]>()?; // consume separator

		let mut port_type = None;
		let mut port_value = None;

		let old = input;
		if let Ok(ty) = input.parse::<Type>() {
			port_type = Some(ty);
			if !input.is_empty() {
				input.parse::<Token![,]>()?; // consume separator
				port_value = Some(old.parse::<Expr>()?);
			}
		} else {
			port_value = Some(old.parse::<Expr>()?)
		}

		Ok(Params {
			port_name,
			port_type,
			port_value,
		})
	}
}

/// Creates a name/inbound_port pair.
/// # Usages:
/// inbound!(name, type)
/// inbound!(name, value)
/// inbound!(name, type, value)
#[proc_macro]
pub fn inbound(input: TokenStream) -> TokenStream {
	let params = parse_macro_input!(input as Params);

	let name = params.port_name;
	if let Some(value) = params.port_value {
		if let Some(tp) = params.port_type {
			quote! {
				(#name.into(), dataport::PortVariant::InBound(dataport::BoundInPort::with_value::<#tp>(#value)))
			}
			.into()
		} else {
			quote! {
				(#name.into(), dataport::PortVariant::InBound(dataport::BoundInPort::with_value(#value)))
			}
			.into()
		}
	} else if let Some(tp) = params.port_type {
		quote! {
			(#name.into(), dataport::PortVariant::InBound(dataport::BoundInPort::new::<#tp>()))
		}
		.into()
	} else {
		panic!("missing type or value as parameter")
	}
}

/// Creates a name/inoutbound_port pair.
/// # Usages:
/// inoutbound!(name, type)
/// inoutbound!(name, value)
/// inoutbound!(name, type, value)
#[proc_macro]
pub fn inoutbound(input: TokenStream) -> TokenStream {
	let params = parse_macro_input!(input as Params);

	let name = params.port_name;
	if let Some(value) = params.port_value {
		if let Some(tp) = params.port_type {
			quote! {
				(#name.into(), dataport::PortVariant::InOutBound(dataport::BoundInOutPort::with_value::<#tp>(#value)))
			}
			.into()
		} else {
			quote! {
				(#name.into(), dataport::PortVariant::InOutBound(dataport::BoundInOutPort::with_value(#value)))
			}
			.into()
		}
	} else if let Some(tp) = params.port_type {
		quote! {
			(#name.into(), dataport::PortVariant::InOutBound(dataport::BoundInOutPort::new::<#tp>()))
		}
		.into()
	} else {
		panic!("missing type or value as parameter")
	}
}

/// Creates a name/outbound_port pair.
/// # Usages:
/// outbound!(name, type)
/// outbound!(name, value)
/// outbound!(name, type, value)
#[proc_macro]
pub fn outbound(input: TokenStream) -> TokenStream {
	let params = parse_macro_input!(input as Params);

	let name = params.port_name;
	if let Some(value) = params.port_value {
		if let Some(tp) = params.port_type {
			quote! {
				(#name.into(), dataport::PortVariant::OutBound(dataport::BoundOutPort::with_value::<#tp>(#value)))
			}
			.into()
		} else {
			quote! {
				(#name.into(), dataport::PortVariant::OutBound(dataport::BoundOutPort::with_value(#value)))
			}
			.into()
		}
	} else if let Some(tp) = params.port_type {
		quote! {
			(#name.into(), dataport::PortVariant::OutBound(dataport::BoundOutPort::new::<#tp>()))
		}
		.into()
	} else {
		panic!("missing type or value as parameter")
	}
}
