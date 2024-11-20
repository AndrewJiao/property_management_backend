pub mod price_basic;
pub mod room_info;
pub mod owner_info;

pub mod basic {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FindDto {
        pub search_type: Option<String>,
    }

}

pub trait ToUpdatePO {
    type PO<'a>
    where
        Self: 'a;
    fn to_update_po(&self, id: i32) -> Self::PO<'_>;
}

pub trait ToInsertPO {
    type PO<'a>
    where
        Self: 'a;
    fn to_insert_po(&self) -> Self::PO<'_>;
}

