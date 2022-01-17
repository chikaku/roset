use crate::get_wrapped_unnamed;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;
use syn::Ident;

pub(crate) struct EnumFromWrapped {
    enum_name: Ident,
    enum_data: DataEnum,
}

impl EnumFromWrapped {
    pub fn new(enum_name: Ident, enum_data: DataEnum) -> Self {
        EnumFromWrapped {
            enum_name,
            enum_data,
        }
    }

    pub fn write_output(&self) -> TokenStream {
        self.enum_data
            .variants
            .iter()
            .map(|var| {
                let var_name = &var.ident;
                let enum_name = &self.enum_name;
                let wrapped = get_wrapped_unnamed("EnumFromWrapped", enum_name, var.fields.clone());

                quote! {
                    impl From<#wrapped> for #enum_name {
                        fn from(inner: #wrapped) -> Self {
                            Self::#var_name(inner)
                        }
                    }
                }
            })
            .collect()
    }
}
