use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Data;
use syn::DataEnum;
use syn::DeriveInput;
use syn::Field;
use syn::Fields;
use syn::Ident;

mod enum_from;
mod enum_from_wrapped;
mod enum_into_wrapped;

use enum_from::EnumFrom;
use enum_from_wrapped::EnumFromWrapped;
use enum_into_wrapped::EnumIntoWrapped;

/// Implement trait `FromStr` `From<T>` for **specific** variant in `enum` type
///
/// - `enum_from(str = "what")` attributes could be used to implement `FromStr` trait and `to_str` method
/// - `enum_from(inner)` attributes could be used to implement `From<T>` for specific variant inner type
///
/// Note: `enum_from(str)` **must** be used for all variant if you use it in one variant
/// ```
/// use roset::EnumFrom;
/// use std::str::FromStr;
///
/// #[derive(PartialEq, Debug, EnumFrom)]
/// enum Animal {
///     #[enum_from(str = "🐱")]
///     Cat,
///     #[enum_from(str = "🐶")]
///     Dog,
/// }
///
/// assert_eq!(Animal::from_str("🐱"), Ok(Animal::Cat));
/// assert_eq!(Animal::from_str("🐶"), Ok(Animal::Dog));
/// assert_eq!((Animal::Cat).to_str(), "🐱");
/// assert_eq!((Animal::Dog).to_str(), "🐶");
/// ```
///
/// ```
/// use roset::EnumFrom;
///
/// #[derive(PartialEq, Debug)]
/// struct Complex {
///     real: i64,
///     imag: i64,
/// }
///
/// #[derive(PartialEq, Debug, EnumFrom)]
/// enum Number {
///     #[enum_from(inner)]
///     Integer(i32),
///     #[enum_from(inner)]
///     Complex(Complex),
///     // #[enum_from(inner)]
///     Float(f64),
/// }
/// assert_eq!(Number::from(1), Number::Integer(1));
/// assert_eq!(Number::from(Complex{real: 1, imag: -1}), Number::Complex(Complex{real: 1, imag: -1}));
///
/// // error: the trait `From<{float}>` is not implemented for `Number`
/// // Number::from(1.0);
/// ```
#[proc_macro_derive(EnumFrom, attributes(enum_from))]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (id, data) = assert_enum("EnumFrom", input);
    let mut handler = EnumFrom::new(id, data);
    handler.parse_attributes();
    handler.write_output().into()
}

/// Implement `From<T>` for every variant inner type in `enum`
///
/// ```
/// use roset::EnumFromWrapped;
///
/// #[derive(PartialEq, Debug, EnumFromWrapped)]
/// enum Number {
///     Integer(i32),
///     Float(f64),
/// }
///
/// assert_eq!(Number::from(1), Number::Integer(1));
/// assert_eq!(Number::from(1.0), Number::Float(1.0));
/// ```
#[proc_macro_derive(EnumFromWrapped)]
pub fn enum_from_wrapped(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (id, data) = assert_enum("EnumFromWrapped", input);
    EnumFromWrapped::new(id, data).write_output().into()
}

/// Implement `TryFrom<T>` for every variant inner type in `enum`
///
/// ```
/// use roset::EnumIntoWrapped;
/// use std::convert::TryInto;
///
/// #[derive(PartialEq, Debug, EnumIntoWrapped)]
/// enum Number {
///     Integer(i32),
///     Float(f64),
/// }
///
/// let a = Number::Integer(1);
/// assert_eq!(a.try_into(), Ok(1));
///
/// let b: Result<f64, ()> = Number::Float(1.0).try_into();
/// assert!(b.is_ok());
///
/// let c: Result<i32, ()> = Number::Float(1.0).try_into();
/// assert!(c.is_err());
/// ```
#[proc_macro_derive(EnumIntoWrapped)]
pub fn enum_into_wrapped(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (id, data) = assert_enum("EnumIntoWrapped", input);
    EnumIntoWrapped::new(id, data).write_output().into()
}

fn assert_enum(name: &str, input: DeriveInput) -> (Ident, DataEnum) {
    let ident = input.ident.clone();
    match input.data {
        Data::Enum(data) => (ident, data),
        _ => panic!("{} must be an enum to use {}", &ident, name),
    }
}

fn get_wrapped_unnamed(
    macro_name: &str,
    enum_name: &Ident,
    fields: Fields,
) -> Punctuated<Field, Comma> {
    let err = format!("{}: can not use {}", enum_name, macro_name);
    match fields {
        Fields::Unnamed(field) => field.unnamed,
        Fields::Unit => panic!("{} with unit variant", err),
        Fields::Named(_) => panic!("{} with named variant", err),
    }
}
