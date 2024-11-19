pub struct OwnerBasicInfoAggRoot {
    id: i32,
    room_number: String,
    owner_name: Option<String>,
    room_square: Option<String>,
    create_by: String,
    update_by: String,
    create_time: Option<NaiveDateTime>,
    update_time: Option<NaiveDateTime>,
    is_delete: bool,
    comment: Option<String>,
    other_basic: Option<AppJson>,
}
