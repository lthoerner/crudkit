mod derives;

use proc_macro::{self, TokenStream};

macro_rules! synerror {
    ( $span_ident:ident, $message:expr ) => {
        syn::Result::Err(syn::Error::new($span_ident.span(), $message))
    };
}

macro_rules! propagate_synerror {
    ( $derive_fn:expr ) => {
        match $derive_fn {
            Ok(output) => output,
            Err(error) => error.into_compile_error().into(),
        }
    };
}

pub(crate) use synerror;

#[proc_macro_derive(IdParameter)]
pub fn derive_id_parameter(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_id_parameter(input))
}

#[proc_macro_derive(Relation, attributes(relation))]
pub fn derive_relation(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_relation(input))
}

#[proc_macro_derive(ReadRelation)]
pub fn derive_read_relation(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_read_relation(input))
}

#[proc_macro_derive(WriteRelation)]
pub fn derive_write_relation(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_write_relation(input))
}

#[proc_macro_derive(Record)]
pub fn derive_record(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_record(input))
}

#[proc_macro_derive(ReadRecord)]
pub fn derive_read_record(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_read_record(input))
}

#[proc_macro_derive(WriteRecord, attributes(auto_primary_key, manual_primary_key))]
pub fn derive_write_record(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_write_record(input))
}

#[proc_macro_derive(GenerateTable)]
pub fn derive_generate_table(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_generate_table(input))
}

#[proc_macro_derive(SingleInsert, attributes(defaultable))]
pub fn derive_single_insert(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_single_insert(input))
}

#[proc_macro_derive(BulkInsert)]
pub fn derive_bulk_insert(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_bulk_insert(input))
}

#[proc_macro_derive(IdentifiableRecord)]
pub fn derive_identifiable_record(input: TokenStream) -> TokenStream {
    propagate_synerror!(derives::derive_functions::derive_identifiable_record(input))
}
