pub fn generate_uuid_v7() -> String {
    let uuid = uuid_v7::gen_uuid_v7();
    uuid.to_string()
}