use legion::*;
use log::{error};
use zodiac_entities::*;

#[system(for_each)]
pub fn report_build_error(error_occurrence: &BuildErrorOccurrence) {
    error!("{}", error_occurrence.error);
}