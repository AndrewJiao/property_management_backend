
pub mod price_basic;
pub mod owner_info;

pub trait ToUpdatePO {
    type PO<'a> where Self: 'a;
    fn to_update_po(&self, id: i32) -> Self::PO<'_>;
}

