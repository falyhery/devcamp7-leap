use crate::helper::validate_entity_title;

pub fn create(entry: Content, validation_data: ValidationData) -> Result<(), String> {
    validate_entity_title(entry.name, entity_name: "Content", allowed_length: MAX_NAME_LEN)
}

pub fn modify(
    new_entry: Content, 
    old_entry: Content, 
    _old_entry_header: ChainHeader, 
    validation_data: ValidationData, 
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &old_entry.teacher_address, 
        validation_data.sources(), 
        "modify the content",
    )?;
    helper::validate_entity_title(&new_entry.name, &Content::entry_type(), MAX_NAME_LEN)?;
    validate_no_teacher_change(old_entry, new_entry)
}

fn validate_no_teacher_change(old_entry: Content, new_entry: Content) -> Result<(), String> {
    if new_entry.teacher_address != old_entry.teacher_address {
        return Err(String::from("Cannot change the teacher of the content"));
    }
    Ok(())
}

pub fn delete(
    entry: Content, 
    _entry_header: ChainHeader, 
    validation_data: ValidationDaa, 
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address, 
        validation_data.sources(), 
        "delete the content",
    )
}

pub fn section_anchor_to_content_link(validation_data: LinkValidationData) -> Result<(), String> {
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
            let target: Content = hdk::utils::get_as_type(link.link.target().clone())?;
            if base.teacher_address != target.teacher_address {
                return Err(String::from(
                    "Can't link SectionAnchor to Content because their teacher addresses are different",
                ));
            } else if author != base.teacher_address {
                return Err(String::from(
                    "Can't link SectionAnchor to Content because your address isn't specified as teacher address for this content",
                ));
            }
            Ok(())
        }
        hdk::LinkValidationData::LinkRemove {
            link,
            validation_data,
        } => {
            // get author of this entry 
            let author: HashString = validation_data.package.chain_header.provenances()[0].source();
            // get link base: entry from which the link goes 
            let base: SectionAnchor = hdk::utils:.get_as_type(address: link.link.base().clone())?;
            if author != base.teacher_address {
                return Err(String::from(
                    "Can't remove link from SectionAnchor to Content because your address isn't specified as teacher_address for this course",
                ));
            }
            Ok(())
        }
    }
}
