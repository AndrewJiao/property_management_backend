pub mod page;


pub mod filter {
    pub trait Filterable: Sized {
        fn if_filter<P>(self, param: &Option<P>, real_fun: fn(&P, Self) -> Self) -> Self;
    }

    impl<T> Filterable for T
    {
        fn if_filter<P>(self, param: &Option<P>, real_fun: fn(&P, Self) -> Self) -> Self {
            match param {
                None => { self }
                Some(page) => { real_fun(page, self) }
            }
        }
    }

    // fn if_filter<Predicate>(self, predicate: Predicate) -> Filter<Self, Predicate>
    // where
    //     Self: methods::FilterDsl<Predicate>,
    // {
    //     methods::FilterDsl::filter(self, predicate)
    // }
}
