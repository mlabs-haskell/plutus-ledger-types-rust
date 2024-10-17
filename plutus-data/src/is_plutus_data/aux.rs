use crate::{PlutusData, PlutusDataError, PlutusType};

/// Given a vector of PlutusData, parse it as an array whose length is known at
/// compile time.
///
/// This function is used by the derive macro.
pub fn parse_fixed_len_constr_fields<const LEN: usize>(
    v: &[PlutusData],
) -> Result<&[PlutusData; LEN], PlutusDataError> {
    v.try_into()
        .map_err(|_| PlutusDataError::UnexpectedListLength {
            got: v.len(),
            wanted: LEN,
        })
}

/// Given a PlutusData, parse it as PlutusData::Constr and its tag as u32. Return
/// the u32 tag and fields.
///
/// This function is used by the derive macro.
pub fn parse_constr(data: &PlutusData) -> Result<(u32, &Vec<PlutusData>), PlutusDataError> {
    match data {
        PlutusData::Constr(tag, fields) => u32::try_from(tag)
            .map_err(|err| PlutusDataError::UnexpectedPlutusInvariant {
                got: err.to_string(),
                wanted: "Constr bigint tag within u32 range".into(),
            })
            .map(|tag| (tag, fields)),
        _ => Err(PlutusDataError::UnexpectedPlutusType {
            wanted: PlutusType::Constr,
            got: PlutusType::from(data),
        }),
    }
}

/// Given a PlutusData, parse it as PlutusData::Constr and verify its tag.
///
/// This function is used by the derive macro.
pub fn parse_constr_with_tag(
    data: &PlutusData,
    expected_tag: u32,
) -> Result<&Vec<PlutusData>, PlutusDataError> {
    let (tag, fields) = parse_constr(data)?;

    if tag != expected_tag {
        Err(PlutusDataError::UnexpectedPlutusInvariant {
            got: tag.to_string(),
            wanted: format!("Constr with tag {}", expected_tag),
        })
    } else {
        Ok(fields)
    }
}

/// Given a PlutusData, parse it as PlutusData::List. Return the plutus data list.
///
/// This function is used by the derive macro.
pub fn parse_list(data: &PlutusData) -> Result<&Vec<PlutusData>, PlutusDataError> {
    match data {
        PlutusData::List(list_of_plutus_data) => Ok(list_of_plutus_data),
        _ => Err(PlutusDataError::UnexpectedPlutusType {
            got: PlutusType::from(data),
            wanted: PlutusType::List,
        }),
    }
}
