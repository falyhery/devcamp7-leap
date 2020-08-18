use crate::helper::validate_entity_title;

pub fn create(entry: Section, validation_data: ValidationData) -> Result<(), String> {
    validate_entity_title(entry.title, entity_name: &Section::entry_type(), allowed_length: MAX_TITLE_LEN); 
}

pub fn modify(
    new_entry: Section, 
    old_entry: Section, 
    _old_entry_header: ChainHeader, 
    validation_data: ValidationData, 
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &old_entry.teacher_address, 
        validation_data.sources(), 
        "modify the section",
    )?; 
    helper::validate_entity_title(&new_entry.title, &Section::entry_type(), MAX_TITLE_LEN)?;
    validate_no_teacher_change(old_entry, new_entry)
}

// this fn is only needed in the current module so it's private 
fn validate_no_teacher_change(old_entry: Section, new_entry: Section) -> Result<(), String> {
    if new_entry.teacher_address != old_entry.teacher_address {
        return Err(String::from("Cannot change the teacher of the section"));
    }
    Ok(())
}

pub fn delete(
    entry: Section, 
    _entry_header: ChainHeader, 
    validation_data: ValidationData, 
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address, 
        validation_data.sources(),
        "delete the section",
    )
}

pub fn anchor_to_section_link(validation_data: LinkValidationData) -> Result<(), String> {
    match validation_data {
        hdk::LinkValidationData::LinkAdd {
            link,
            validation_data,
        } => {
            // get author of this entry 
            let author: HashString = validation_data.package.chain_header.provenances()[0].source();
            // get link base: entry from which the link goes 
            let base: SectionAnchor = hdk::utils::get_as_type(link.link.base().clone())?;
            // get link target: entry to which the link goes
            let target: Section = hdk::utils::get_as_type(link.link.target().clone())?; 
            if base.teacher_address != target.teacher_address {
                // notice that we're using return and ending this statement with ; symbol 
                // You can do both: skip ; symbol in the last fn statement or explicitly add return to it and then leave ; as is
                return Err(String::from(
                    "Can't link SectionAnchor to Section because their teacher addresses are different",
                ));
            } else if author != base.teacher_address {
                return Err(String::from(
                    "Can't link SectionAnchor to Section because your address isn't specified as teacher_address for this course"
                ));
            }
            Ok(())
        }
    }
}
