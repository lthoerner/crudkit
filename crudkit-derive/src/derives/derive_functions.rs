use deluxe::ExtractAttributes;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Result as SynResult, Type,
};

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

#[derive(Clone, PartialEq)]
enum PrimaryKeyAttribute {
    Auto,
    Manual,
    None,
}

pub fn derive_id_parameter(input: TokenStream2) -> SynResult<TokenStream2> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let (_, unparsed_type_fields) =
        get_struct_data_and_unparsed_fields(&type_name, &type_data, "IdParameter")?;

    let first_field = unparsed_type_fields.named.into_iter().next().unwrap();
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

pub fn derive_relation(input: TokenStream2) -> SynResult<TokenStream2> {
    let mut input: DeriveInput = syn::parse2(input)?;
    let type_name = input.ident.clone();
    let type_data = input.data.clone();
    let record_type_name = suffix_ident(&type_name, "Record");

    get_struct_data_and_unparsed_fields(&type_name, &type_data, "Relation")?;

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

pub fn derive_read_relation(input: TokenStream2) -> SynResult<TokenStream2> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let record_type_name = suffix_ident(&type_name, "Record");

    get_struct_data_and_unparsed_fields(&type_name, &type_data, "ReadRelation")?;

    Ok(quote! {
        impl crudkit::traits::read::ReadRelation for #type_name {
            type ReadRecord = #record_type_name;
        }
    }
    .into())
}

pub fn derive_write_relation(input: TokenStream2) -> SynResult<TokenStream2> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let record_type_name = suffix_ident(&type_name, "Record");

    get_struct_data_and_unparsed_fields(&type_name, &type_data, "WriteRelation")?;

    Ok(quote! {
        impl crudkit::traits::write::WriteRelation for #type_name {
            type WriteRecord = #record_type_name;
        }
    }
    .into())
}

pub fn derive_record(input: TokenStream2) -> SynResult<TokenStream2> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let relation_type_name = trim_ident_suffix(&type_name, "Record");

    let (_, unparsed_type_fields) =
        get_struct_data_and_unparsed_fields(&type_name, &type_data, "Record")?;

    let column_names: Vec<String> = parse_field_data(&unparsed_type_fields)?
        .into_iter()
        .map(|f| f.name)
        .collect();

    Ok(quote! {
        impl crudkit::traits::shared::Record for #type_name {
            const COLUMN_NAMES: &[&str] = &[#(#column_names),*];

            type Relation = #relation_type_name;
        }
    }
    .into())
}

pub fn derive_read_record(input: TokenStream2) -> SynResult<TokenStream2> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let relation_type_name = trim_ident_suffix(&type_name, "Record");

    get_struct_data_and_unparsed_fields(&type_name, &type_data, "ReadRecord")?;

    Ok(quote! {
        impl crudkit::traits::read::ReadRecord for #type_name {
            type ReadRelation = #relation_type_name;
        }
    }
    .into())
}

pub fn derive_write_record(input: TokenStream2) -> SynResult<TokenStream2> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;

    let relation_type_name = trim_ident_suffix(&type_name, "Record");
    let create_params_type_name = suffix_ident(&type_name, "CreateQueryParameters");
    let update_params_type_name = suffix_ident(&type_name, "UpdateQueryParameters");

    let (_, unparsed_type_fields) =
        get_struct_data_and_unparsed_fields(&type_name, &type_data, "WriteRecord")?;

    let type_fields = parse_field_data_with_attributes(&type_name, &unparsed_type_fields)?;

    let key_placeholders: Vec<String> = type_fields
        .iter()
        .map(|f| format!("{} = {{}}", f.data.name))
        .collect();
    let where_clause_with_key_placeholders = format!("WHERE {}", key_placeholders.join(", "));

    let create_query_parameter_fields: Vec<TokenStream2> = type_fields
        .iter()
        .filter_map(|f| match f.primary_key {
            PrimaryKeyAttribute::Auto => None,
            _ => {
                // * This needs to be done instead of just using `quote!(#f)` because otherwise, any
                // * additional attributes on the field would be included in the output
                let field_name = f.data.ident.clone();
                let field_type = f.data.r#type.clone();
                Some(quote!(#field_name: #field_type))
            }
        })
        .collect();

    let create_query_mapped_fields: Vec<TokenStream2> = type_fields
        .iter()
        .map(|f| {
            let field_name = f.data.ident.clone();
            match f.primary_key {
                PrimaryKeyAttribute::Auto => quote!(#field_name: None),
                _ => quote!(#field_name: params.#field_name),
            }
        })
        .collect();

    let update_query_parameter_fields: Vec<TokenStream2> = type_fields
        .iter()
        .map(|f| {
            let field_type = f.data.r#type.clone();
            let (new_field_name, new_field_type) = match f.primary_key {
                PrimaryKeyAttribute::None => (
                    Ident::new(&format!("new_{}", f.data.name), type_name.span()),
                    quote!(Option<#field_type>),
                ),
                _ => (f.data.ident.clone(), quote!(#field_type)),
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

            fn update_one(
                database: &crudkit::database::PgDatabase,
                update_params: Self::UpdateQueryParameters,
            ) -> impl std::future::Future<Output = ()> {
                async move {
                    todo!()
                }
            }
        }
    }
    .into())
}

pub fn derive_generate_table(input: TokenStream2) -> SynResult<TokenStream2> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;

    get_struct_data_and_unparsed_fields(&type_name, &type_data, "GenerateTable")?;

    Ok(quote! {
        impl crate::database::traits::generate::GenerateTable for #type_name {}
    }
    .into())
}

pub fn derive_single_insert(input: TokenStream2) -> SynResult<TokenStream2> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;

    let (_, unparsed_type_fields) =
        get_struct_data_and_unparsed_fields(&type_name, &type_data, "SingleInsert")?;

    let type_fields = parse_field_data_with_attributes(&type_name, &unparsed_type_fields)?;

    let binding_statements: Vec<TokenStream2> = type_fields
        .into_iter()
        .map(|f| {
            let FieldDataWithAttributeFlags {
                data: FieldData {
                    ident: field_ident, ..
                },
                defaultable,
                ..
            } = f;

            if defaultable {
                quote! {
                    match record.#field_ident {
                        Some(column_value) => { builder.push_bind(column_value); },
                        None => { builder.push("DEFAULT"); },
                    }
                }
            } else {
                quote!(builder.push_bind(record.#field_ident);)
            }
        })
        .collect();

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

pub fn derive_bulk_insert(input: TokenStream2) -> SynResult<TokenStream2> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    get_struct_data_and_unparsed_fields(&type_name, &type_data, "BulkInsert")?;

    Ok(quote! {
        impl crudkit::traits::write::BulkInsert for #type_name {}
    }
    .into())
}

pub fn derive_identifiable_record(input: TokenStream2) -> SynResult<TokenStream2> {
    let (type_name, type_data) = parse_type_ident_and_data(input)?;
    let (_, unparsed_type_fields) =
        get_struct_data_and_unparsed_fields(&type_name, &type_data, "IdentifiableRecord")?;

    let first_field = unparsed_type_fields.named.into_iter().next().unwrap();
    let first_field_name = first_field.ident.unwrap();

    Ok(quote! {
        impl crudkit::traits::shared::IdentifiableRecord for #type_name {
            fn id(&self) -> Option<i32> {
                self.#first_field_name
            }
        }
    }
    .into())
}

fn parse_type_ident_and_data(input: TokenStream2) -> SynResult<(Ident, Data)> {
    let DeriveInput {
        ident: struct_ident,
        data: struct_data,
        ..
    } = syn::parse2(input)?;

    Ok((struct_ident, struct_data))
}

fn get_struct_data_and_unparsed_fields(
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

fn parse_field_data(unparsed_fields: &FieldsNamed) -> SynResult<Vec<FieldData>> {
    Ok(unparsed_fields.named.iter().map(FieldData::from).collect())
}

fn parse_field_data_with_attributes(
    struct_ident: &Ident,
    unparsed_fields: &FieldsNamed,
) -> SynResult<Vec<FieldDataWithAttributeFlags>> {
    unparsed_fields
        .named
        .clone()
        .into_iter()
        .map(|mut f| {
            let auto_primary_key =
                deluxe::extract_attributes::<_, AutoPrimaryKeyAttribute>(&mut f).is_ok();
            let manual_primary_key =
                deluxe::extract_attributes::<_, ManualPrimaryKeyAttribute>(&mut f).is_ok();
            let defaultable = deluxe::extract_attributes::<_, DefaultableRecordAttribute>(&mut f).is_ok();

            let primary_key = match (auto_primary_key, manual_primary_key) {
                (true, true) => return synerror!(struct_ident, "cannot use both `#[auto_primary_key]` and `#[manual_primary_key]` on a single column"),
                (true, false) => PrimaryKeyAttribute::Auto,
                (false, true) => PrimaryKeyAttribute::Manual,
                (false, false) => PrimaryKeyAttribute::None,
            };

            let data = FieldData::from(&f);

            Ok(FieldDataWithAttributeFlags{ data, primary_key, defaultable })
        })
        .collect()
}

fn field_name_string(field: &Field) -> String {
    field
        .ident
        .clone()
        .unwrap()
        .to_string()
        .trim_start_matches("r#")
        .to_owned()
}

fn prefix_ident(ident: &Ident, prefix: &str) -> Ident {
    Ident::new(&format!("{}{}", prefix, ident), ident.span())
}

fn suffix_ident(ident: &Ident, suffix: &str) -> Ident {
    Ident::new(&format!("{}{}", ident, suffix), ident.span())
}

fn trim_ident_prefix(ident: &Ident, prefix: &str) -> Ident {
    Ident::new(
        ident.clone().to_string().trim_start_matches(prefix),
        ident.span(),
    )
}

fn trim_ident_suffix(ident: &Ident, suffix: &str) -> Ident {
    Ident::new(
        ident.clone().to_string().trim_end_matches(suffix),
        ident.span(),
    )
}

struct FieldData {
    ident: Ident,
    r#type: Type,
    name: String,
}

struct FieldDataWithAttributeFlags {
    data: FieldData,
    primary_key: PrimaryKeyAttribute,
    defaultable: bool,
}

impl From<&Field> for FieldData {
    fn from(value: &Field) -> Self {
        let ident = value.ident.clone().unwrap();
        let r#type = value.ty.clone();
        let name = field_name_string(&value);

        Self {
            ident,
            r#type,
            name,
        }
    }
}

impl From<FieldData> for Ident {
    fn from(value: FieldData) -> Self {
        value.ident
    }
}

impl From<FieldData> for String {
    fn from(value: FieldData) -> Self {
        value.name
    }
}
