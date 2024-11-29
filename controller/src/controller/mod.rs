pub mod hello;
pub mod price_basic;
pub mod owner_info;
pub mod room_info;
pub mod property_fee;
pub mod owner_fee;


trait IfFilter {
    fn if_filter<'a, P>(self, param: &'a Option<P>, fun: fn(Self, &'a P) -> Self) -> Self;
    fn if_filter_tow_param<'a, 'b, P1, P2>(self, param1: &'a Option<P1>, param2: &'b Option<P2>, fun: fn(Self, (&'a P1, &'b P2)) -> Self) -> Self;
}
impl<T> IfFilter for T {
    fn if_filter<'a, P>(self, param: &'a Option<P>, fun: fn(Self, &'a P) -> Self) -> Self {
        if let Some(param) = param {
            fun(self, param)
        } else {
            self
        }
    }

    fn if_filter_tow_param<'a, 'b, P1, P2>(self, param1: &'a Option<P1>, param2: &'b Option<P2>, fun: fn(Self, (&'a P1, &'b P2)) -> Self) -> Self {
        if let Some(p1) = param1 {
            if let Some(p2) = param2 {
                return fun(self, (p1, p2));
            }
        }
        self
    }
}
