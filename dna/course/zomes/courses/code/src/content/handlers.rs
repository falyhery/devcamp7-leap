use hdk::holochain_persistence_api::cas::content::Address; 
use hdk::prelude::LinkMatch; 
use holochain_entry_utils::HolochainEntry; 
use crate::anchor_trait::AnchorTrait; 
use crate::helper; 

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    // create new Content entry 
    let new_content: Content = Content::new( 
        title, 
        timestamp,
    );
    // commit this entry to DHT and save its address 
    let new_content_address: HashString = hdk::commit_entry(&new_content.entry())?;

    // link SectionAnchor to Content Entry for this content to be findable 
    hdk::link_entries(
        base: &section_anchor_address, 
        target: &content_address, 
        link_type: SectionAnchor::link_type(),
        tag: "".to_owned(), 
    )?; 

    Ok(content_address)
}

pub fn get_latest_content() -> ZomeApiResult<Option<Content, Address>> {
    helper::get_latest_data_entry::<Content>(section_anchor_address, &SectionAnchor::link_type())
}

fn commit_update(
    content: Content, 
    previous_content_address: &Address, 
) -> ZomeApiResult<Address> {
    // commit updated course to DHT and get its new address 
    let new_content_address: HashString = hdk::update_entry(content.entry(), previous_content_address)?;

    // remove link to previous version of content 
    hdk::remove_link(
        base: section_anchor_address, 
        target: &previous_content_address, 
        link_type: SectionAnchor::link_type(), 
        tag: "".to_owned(),
    )?;

    // create link to new version of content 
    hdk::link_entries(
        base: section_anchor_address, 
        target: &new_content_address, 
        link_type: SectionAnchor::link_type(), 
        tag: "".to_owned(),
    )?;

    Ok(content_address.to_owned())
}

pub fn update(
    title: String,    
) -> ZomeApiResult<Address> {
    let latest_content_result: Option<(Content, HashString)> = get_latest_content()?;
    match latest_content_result {
        Some((mut previous_content: Content, previous_content_address: HashString)) => {
            // update this content 
            previous_content.title = title; 

            commit_update(
                previous_content, 
                &previous_content_address,
            )?; 
        }
        None => {
            return Err(ZomeApiError::from(
                "Can't update a deleted content".to_owned(), 
            )); 
        }
    }
}

pub fn delete() -> ZomeApiResult<Address> {
    // remove link from SectionAnchor to Content Entry 
    hdk::remove_link(
        &SectionAnchor::new().address()?, 
        &content_address, 
        SectionAnchor::link_type(),
        "".to_owned(),
    )?;

    hdk::remove_entry(&content_address)
}