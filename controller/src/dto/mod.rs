pub mod price_basic;
pub mod room_info;
pub mod owner_info;
pub mod property_fee;
pub mod owner_fee;
pub mod user;

pub trait ToUpdatePO {
    type PO<'a>
    where
        Self: 'a;
    fn to_update_po(&self, id: i64) -> Self::PO<'_>;
}

pub trait ToInsertPO {
    type PO<'a>
    where
        Self: 'a;
    fn to_insert_po(&self) -> Self::PO<'_>;
}

pub trait ToDesc {
    fn to_desc(&self) -> String;
}
