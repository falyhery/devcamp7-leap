use hdk::holochain_persistence_api::cas::content::Address; 
use hdk::prelude::LinkMatch; 
use holochain_entry_utils::HolochainEntry; 

use crate::anchor_trait::AnchorTrait;
use crate::helper; 

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    // initialize SectionAnchor instance 
    let section_anchor: SectionAnchor = SectionAnchor::new(
        title: title.clone(), 
        timestamp);
    // commit SectionAnchor to DHT 
    let section_anchor_address: HashString = hdk::commit_entry(&section_anchor.entry())?;

    // create new Section entry
    let new_section: Section = Section::new(
        title, 
        timestamp,
        anchor_address: section_anchor_address.clone(),
    );
    // commit this entry to the DHT and save its address
    let new_section_address: HashString = hdk::commit_entry(&new_section.entry())?;

    // link SectionAnchor to Section entry
    hdk::link_entries(
        base: &section_anchor_address, 
        target: &new_section_address, 
        link_type: SectionAnchor::link_type(),
        tag: "".to_owned(),
    )?;

    // link SectionAnchor to Content Entry for this content to be findable 
    hdk::link_entries(
        base: &section_anchor_address,
        target: &new_content_address, 
        link_type: "section_content",
        tag: "".to_owned(),
    )?;

    Ok(section_anchor_address)
}

pub fn get_latest_section(
    section_anchor_address: &Address, 
) -> ZomeApiResult<Option<Section, Address>> {
    helper::get_latest_data_entry::<Section>(entry_anchor_address: section_anchor_address, link_type: &SectionAnchor::link_type())
}

fn commit_update(
    section: Section, 
    previous_section_address: &Address, 
    section_anchor_address: &Address, 
) -> ZomeApiResult<Address> {
    // commit updated section to DHT and get its new address
    let new_section_address: HashString = hdk::update_entry(new_entry: section.entry(), previous_section_address)?; 

    // remove link to previous version of section 
    hdk::remove_link(
        base: section_anchor_address, 
        target: &previous_section_address, 
        link_type: SectionAnchor::link_type(), 
        tag: "".to_owned(),
    )?; 

    Ok(section_anchor_address.to_owned())
}

pub fn update(
    title: String, 
    section_anchor_address: &Address, 
) -> ZomeApiResult<Address> {
    let latest_section_result: Option<(Section, HashString)> = get_latest_section(section_anchor_address)?; 
    match latest_section_result {
        Some((mut previous_section: Section, previous_section_address: HashString)) => {
            //update this section 
            previous_section.title = title; 

            commit_update(
                previous_section,
                &previous_section_address, 
                section_anchor_address, 
            )?; 

            // returning address of the section anchor. 
            return Ok(section_anchor_address.clone());
        }
        None => {
            return Err(ZomeApiError::from(
                "Can't update a deleted section".to_owned(),
            ));
        }
    }
}

pub fn delete(section_anchor_address: Address) -> ZomeApiResult<Address> {
    // retrieve section_anchor entry. 
    let section_anchor: SectionAnchor = hdk::utils::get_as_type(address: section_anchor_address.clone())?;

    // remove link from CourseAnchor to SectionAnchor 
    hdk::remove_link(
        base: &CourseAnchor::new().address()?, 
        target: &section_anchor_address, 
        link_type: CourseAnchor::link_type(), 
        tag: "".to_owned(),
    )?; 

    hdk::remove_entry(&section_anchor_address)
}