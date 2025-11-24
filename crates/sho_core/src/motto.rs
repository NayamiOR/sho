use crate::id::Id;

pub struct ReignMotto {
    id: Id,
    name: String,
    emperor:Id,
    dynasty:Id,
    start_bc_year: Option<u32>,
    end_bc_year: Option<u32>,
}