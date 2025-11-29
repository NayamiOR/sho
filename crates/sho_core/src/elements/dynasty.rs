use crate::id::Id;

#[derive(Clone)]
pub struct Dynasty {
    name: String,
    start_bc_year: Option<u32>,
    end_bc_year: Option<u32>,
}

pub struct ReignMotto {
    name: String,
    emperor: Id,
    dynasty: Id,
    start_bc_year: Option<u32>,
    end_bc_year: Option<u32>,
}
