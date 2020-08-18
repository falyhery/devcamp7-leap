use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{dna::entry_types::Sharing, validation::EntryValidationData},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};
use holochain_entry_utils::HolochainEntry;
use super::validation; 

pub const MAX_TITLE_LEN: usize = 50; 

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section {
    pub title: String,
    pub course_anchor_address: Address,
    pub timestamp: u64,
    pub anchor_address: Address,
}

impl Section {
    pub fn new(
        title: String,
        course_anchor_address: Address,
        timestamp: u64,
        anchor_address: Address,
    ) -> Self {
        Section {
            title: title,
            course_anchor_address: course_anchor_address,
            timestamp: timestamp,
            anchor_address: anchor_address,
        }
    }
}

impl HolochainEntry for Section {
    fn entry_type() -> String {
        String::from("section")
    }
}

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: Section::entry_type(),
        description: "this is the definition of section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Section>| {
            match  validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    validation::create(entry, validation_data)
                },
                EntryValidationData::Modify { new_entry, old_entry, old_entry_header, validation_data } => {
                    validation::modify(new_entry, old_entry, old_entry_header, validation_data)
                },
                EntryValidationData::Delete { old_entry, old_entry_header, validation_data } => {
                    validation::delete(old_entry, old_entry_header, validation_data)
                }
            }
        },
        // Since now Section entry is a data entry that is hidden behind the SectionAnchor,
        // there won't be any links that it has.
        links:[]
    )
}
