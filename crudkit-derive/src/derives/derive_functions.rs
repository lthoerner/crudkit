use deluxe::ExtractAttributes;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident, Result as SynResult};

use crate::synerror;

#[derive(ExtractAttributes)]
#[deluxe(attributes(relation))]
struct RelationAttributes {
    schema_name: Option<String>,
    relation_name: String,
    primary_key: String,
}

#[derive(ExtractAttributes)]
#[deluxe(attributes(defaultable))]
struct DefaultableRecordAttribute;

#[derive(ExtractAttributes)]
#[deluxe(attributes(auto_primary_key))]
struct AutoPrimaryKeyAttribute;

#[derive(ExtractAttributes)]
#[deluxe(attributes(manual_primary_key))]
struct ManualPrimaryKeyAttribute;

#[derive(Clone)]
enum PrimaryKeyAttribute {
    Auto,
    Manual,
    None,
}

pub fn derive_id_parameter(input: TokenStream) -> SynResult<TokenStream> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let (_, type_fields) = get_struct_data_and_named_fields(&type_name, &type_data, "IdParameter")?;

    let first_field = type_fields.named.into_iter().next().unwrap();
    let first_field_name = first_field.ident.unwrap();

    Ok(quote! {
        impl crate::api::IdParameter for #type_name {
            fn new(#first_field_name: usize) -> Self {
                Self { #first_field_name }
            }

            fn id(&self) -> usize {
                self.#first_field_name
            }
        }
    }
    .into())
}

pub fn derive_relation(input: TokenStream) -> SynResult<TokenStream> {
    let mut input: DeriveInput = syn::parse(input)?;
    let type_name = input.ident.clone();
    let type_data = input.data.clone();
    let record_type_name = suffix_ident(&type_name, "Record");

    get_struct_data_and_named_fields(&type_name, &type_data, "Relation")?;

    let Ok(RelationAttributes {
        schema_name,
        relation_name,
        primary_key,
    }) = deluxe::extract_attributes(&mut input)
    else {
        return synerror!(
            type_name,
            "cannot derive `Relation` without `#[relation(...)]` attribute"
        );
    };

    let optional_schema_definition = schema_name.map(|schema_name| {
        quote! {
            const SCHEMA_NAME: &str = #schema_name;
        }
    });

    Ok(quote! {
        impl crudkit::traits::shared::Relation for #type_name {
            type Record = #record_type_name;
            #optional_schema_definition
            const RELATION_NAME: &str = #relation_name;
            const PRIMARY_KEY: &str = #primary_key;

            fn with_records(records: Vec<Self::Record>) -> Self {
                Self { records }
            }

            fn take_records(self) -> Vec<Self::Record> {
                self.records
            }

            fn records(&self) -> &[Self::Record] {
                &self.records
            }
        }
    }
    .into())
}

pub fn derive_read_relation(input: TokenStream) -> SynResult<TokenStream> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let record_type_name = suffix_ident(&type_name, "Record");

    get_struct_data_and_named_fields(&type_name, &type_data, "ReadRelation")?;

    Ok(quote! {
        impl crudkit::traits::read::ReadRelation for #type_name {
            type ReadRecord = #record_type_name;
        }
    }
    .into())
}

pub fn derive_write_relation(input: TokenStream) -> SynResult<TokenStream> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let record_type_name = suffix_ident(&type_name, "Record");

    get_struct_data_and_named_fields(&type_name, &type_data, "WriteRelation")?;

    Ok(quote! {
        impl crudkit::traits::write::WriteRelation for #type_name {
            type WriteRecord = #record_type_name;
        }
    }
    .into())
}

pub fn derive_record(input: TokenStream) -> SynResult<TokenStream> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let relation_type_name = trim_ident_suffix(&type_name, "Record");

    let (_, type_fields) = get_struct_data_and_named_fields(&type_name, &type_data, "Record")?;

    let column_names = get_field_names(&type_fields);

    Ok(quote! {
        impl crudkit::traits::shared::Record for #type_name {
            const COLUMN_NAMES: &[&str] = &[#(#column_names),*];

            type Relation = #relation_type_name;
        }
    }
    .into())
}

pub fn derive_read_record(input: TokenStream) -> SynResult<TokenStream> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let relation_type_name = trim_ident_suffix(&type_name, "ReadRecord");

    get_struct_data_and_named_fields(&type_name, &type_data, "ReadRecord")?;

    Ok(quote! {
        impl crudkit::traits::read::ReadRecord for #type_name {
            type ReadRelation = #relation_type_name;
        }
    }
    .into())
}

pub fn derive_write_record(input: TokenStream) -> SynResult<TokenStream> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;

    let relation_type_name = trim_ident_suffix(&type_name, "Record");
    let create_params_type_name = suffix_ident(&type_name, "CreateQueryParameters");
    let update_params_type_name = suffix_ident(&type_name, "UpdateQueryParameters");

    let (_, type_fields) = get_struct_data_and_named_fields(&type_name, &type_data, "WriteRecord")?;

    let mut fields_with_primary_key_flags = Vec::new();
    for mut field in type_fields.named {
        let auto_primary_key =
            deluxe::extract_attributes::<_, AutoPrimaryKeyAttribute>(&mut field).is_ok();
        let manual_primary_key =
            deluxe::extract_attributes::<_, ManualPrimaryKeyAttribute>(&mut field).is_ok();

        let primary_key_flag = match (auto_primary_key, manual_primary_key) {
            (true, true) => return synerror!(type_name, "cannot use both `#[auto_primary_key]` and `#[manual_primary_key]` on a single column"),
            (true, false) => PrimaryKeyAttribute::Auto,
            (false, true) => PrimaryKeyAttribute::Manual,
            (false, false) => PrimaryKeyAttribute::None,
        };

        fields_with_primary_key_flags.push((field, primary_key_flag))
    }

    let (primary_key_fields, primary_key_field_accessors): (Vec<Ident>, Vec<TokenStream2>) =
        fields_with_primary_key_flags
            .iter()
            .filter_map(|(f, pk)| {
                let field_name = f.ident.clone().unwrap();
                match pk {
                    PrimaryKeyAttribute::Auto => {
                        Some((field_name.clone(), quote!(#field_name.unwrap())))
                    }
                    PrimaryKeyAttribute::Manual => Some((field_name.clone(), quote!(#field_name))),
                    _ => None,
                }
            })
            .unzip();

    let key_placeholders: Vec<String> = primary_key_fields
        .iter()
        .map(|f| format!("{} = {{}}", f.to_string()))
        .collect();
    let where_clause_with_key_placeholders = format!("WHERE {}", key_placeholders.join(", "));

    let create_query_parameter_fields: Vec<TokenStream2> = fields_with_primary_key_flags
        .clone()
        .into_iter()
        .filter_map(|(f, pk)| match pk {
            PrimaryKeyAttribute::Auto => None,
            _ => {
                // * This needs to be done instead of just using `quote!(#f)` because otherwise, any
                // * additional attributes on the field would be included in the output
                let field_name = f.ident.unwrap();
                let field_type = f.ty;
                Some(quote!(#field_name: #field_type))
            }
        })
        .collect();

    let create_query_mapped_fields: Vec<TokenStream2> = fields_with_primary_key_flags
        .clone()
        .into_iter()
        .map(|(f, pk)| {
            let field_name = f.ident.unwrap();
            match pk {
                PrimaryKeyAttribute::Auto => quote!(#field_name: None),
                _ => quote!(#field_name: params.#field_name),
            }
        })
        .collect();

    let update_query_parameter_fields: Vec<TokenStream2> = fields_with_primary_key_flags
        .into_iter()
        .map(|(f, pk)| {
            let field_type = f.ty.clone();
            let (new_field_name, new_field_type) = match pk {
                PrimaryKeyAttribute::None => {
                    let field_name_string = f
                        .ident
                        .clone()
                        .unwrap()
                        .to_string()
                        .trim_start_matches("r#")
                        .to_owned();

                    (
                        Ident::new(&format!("new_{}", field_name_string), type_name.span()),
                        quote!(Option<#field_type>),
                    )
                }
                _ => (f.ident.unwrap(), quote!(#field_type)),
            };

            quote!(#new_field_name: #new_field_type)
        })
        .collect();

    Ok(quote! {
        #[derive(Clone, serde::Deserialize)]
        pub struct #create_params_type_name {
            #(
                #create_query_parameter_fields
            ),*
        }

        #[derive(Clone, serde::Deserialize)]
        pub struct #update_params_type_name {
            #(
                #update_query_parameter_fields
            ),*
        }

        impl From<#create_params_type_name> for #type_name {
            fn from(params: #create_params_type_name) -> Self {
                Self {
                    #(
                        #create_query_mapped_fields
                    ),*
                }
            }
        }

        impl crudkit::traits::write::WriteRecord for #type_name {
            type WriteRelation = #relation_type_name;
            type CreateQueryParameters = #create_params_type_name;
            type UpdateQueryParameters = #update_params_type_name;
        }
    }
    .into())
}

pub fn derive_generate_table(input: TokenStream) -> SynResult<TokenStream> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;

    get_struct_data_and_named_fields(&type_name, &type_data, "GenerateTable")?;

    Ok(quote! {
        impl crate::database::traits::generate::GenerateTable for #type_name {}
    }
    .into())
}

pub fn derive_single_insert(input: TokenStream) -> SynResult<TokenStream> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;

    let (_, type_fields) = get_struct_data_and_named_fields(&type_name, &type_data, "SingleInsert")?;

    let fields: Vec<(Ident, bool)> = type_fields.named.into_iter().map(|mut f| {
        let field_ident = f.ident.clone().unwrap();
        let defaultable_attribute: Option<DefaultableRecordAttribute> =
        deluxe::extract_attributes(&mut f).ok();

        (field_ident, defaultable_attribute.is_some())
    }).collect();

    let binding_statements: Vec<TokenStream2> = fields.into_iter().map(|(field_ident, defaultable)| {
        match defaultable {
            true => {
                quote! {
                    match record.#field_ident {
                        Some(column_value) => { builder.push_bind(column_value); },
                        None => { builder.push("DEFAULT"); },
                    }
                }
            }
            false => quote!(builder.push_bind(record.#field_ident);),
        }
    }).collect();

    Ok(quote! {
        impl crudkit::traits::write::SingleInsert for #type_name {
            fn push_column_bindings(
                mut builder: sqlx::query_builder::Separated<sqlx::Postgres, &str>,
                record: Self,
            ) {
                #(
                    #binding_statements
                )*
            }
        }
    }
    .into())
}

pub fn derive_bulk_insert(input: TokenStream) -> SynResult<TokenStream> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    get_struct_data_and_named_fields(&type_name, &type_data, "BulkInsert")?;

    Ok(quote! {
        impl crudkit::traits::write::BulkInsert for #type_name {}
    }
    .into())
}

pub fn derive_identifiable_record(input: TokenStream) -> SynResult<TokenStream> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let (_, type_fields) =
        get_struct_data_and_named_fields(&type_name, &type_data, "IdentifiableRecord")?;

    let first_field = type_fields.named.into_iter().next().unwrap();
    let first_field_name = first_field.ident.unwrap();

    Ok(quote! {
        impl crate::database::tables::IdentifiableRecord for #type_name {
            fn id(&self) -> Option<i32> {
                self.#first_field_name
            }
        }
    }
    .into())
}

fn parse_type_ident_and_data(input: TokenStream) -> SynResult<(Ident, Data)> {
    let DeriveInput {
        ident: struct_ident,
        data: struct_data,
        ..
    } = syn::parse(input)?;

    Ok((struct_ident, struct_data))
}

fn get_struct_data_and_named_fields(
    ident: &Ident,
    data: &Data,
    trait_name: &str,
) -> SynResult<(DataStruct, FieldsNamed)> {
    let Data::Struct(data_struct) = data else {
        return synerror!(
            ident,
            format!("cannot derive `{}` for non-struct types", trait_name)
        );
    };

    let Fields::Named(struct_fields) = &data_struct.fields else {
        return synerror!(
            ident,
            format!("cannot derive `{}` for unit or tuple structs", trait_name)
        );
    };

    Ok((data_struct.clone(), struct_fields.clone()))
}

fn get_field_names(fields: &FieldsNamed) -> Vec<String> {
    fields
        .named
        .iter()
        .map(|f| {
            f.ident
                .clone()
                .unwrap()
                .to_string()
                .trim_start_matches("r#")
                .to_owned()
        })
        .collect::<Vec<String>>()
}

fn suffix_ident(ident: &Ident, suffix: &str) -> Ident {
    Ident::new(&format!("{}{}", ident, suffix), ident.span())
}

fn trim_ident_suffix(ident: &Ident, suffix: &str) -> Ident {
    Ident::new(
        ident.clone().to_string().trim_end_matches(suffix),
        ident.span(),
    )
}
