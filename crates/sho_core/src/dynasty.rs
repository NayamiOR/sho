use crate::id::Id;

pub struct Dynasty {
    name: String,
    start_bc_year: Option<u32>,
    end_bc_year: Option<u32>,
}

impl Dynasty {
    pub fn new(name: String, start_bc_year: Option<u32>, end_bc_year: Option<u32>) -> Self {
        Dynasty {
            name,
            start_bc_year,
            end_bc_year,
        }
    }
}

pub struct ReignMotto {
    name: String,
    emperor: Id,
    dynasty: Id,
    start_bc_year: Option<u32>,
    end_bc_year: Option<u32>,
}

impl ReignMotto {
    pub fn new(
        name: String,
        emperor: Id,
        dynasty: Id,
        start_bc_year: Option<u32>,
        end_bc_year: Option<u32>,
    ) -> Self {
        ReignMotto {
            name,
            emperor,
            dynasty,
            start_bc_year,
            end_bc_year,
        }
    }
}
