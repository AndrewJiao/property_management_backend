// @generated automatically by Diesel CLI.

pub mod basic {
    pub mod sql_types {
        #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
        #[diesel(postgres_type(name = "calculate_operation"))]
        pub struct CalculateOperation;
    }

    diesel::table! {
        basic.posts (id) {
            id -> Int4,
            title -> Varchar,
            body -> Text,
            published -> Bool,
        }
    }

    diesel::table! {
        basic.t_owner_basic_info (id) {
            id -> Int4,
            room_number -> Varchar,
            owner_name -> Nullable<Varchar>,
            room_square -> Nullable<Numeric>,
            create_by -> Nullable<Varchar>,
            update_by -> Nullable<Varchar>,
            create_time -> Timestamp,
            update_time -> Timestamp,
            is_delete -> Bool,
            comment -> Nullable<Text>,
            other_basic -> Nullable<Json>,
            delete_at -> Nullable<Timestamp>,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;
        use super::sql_types::CalculateOperation;

        basic.t_price_basic (id) {
            id -> Int8,
            name -> Nullable<Varchar>,
            basic_number -> Nullable<Numeric>,
            create_by -> Nullable<Varchar>,
            update_by -> Nullable<Varchar>,
            create_time -> Timestamp,
            update_time -> Timestamp,
            is_delete -> Nullable<Bool>,
            operation_type -> Nullable<CalculateOperation>,
            comment -> Nullable<Text>,
            basic_code -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        basic.t_property_fee_detail (id) {
            id -> Int4,
            room_number -> Nullable<Varchar>,
            room_owner_name -> Nullable<Varchar>,
            management_fee -> Nullable<Numeric>,
            part_fee -> Nullable<Numeric>,
            machine_room_renovation_fee -> Nullable<Numeric>,
            electric_fee -> Nullable<Numeric>,
            electric_share_fee -> Nullable<Numeric>,
            water_fee -> Nullable<Numeric>,
            water_share_fee -> Nullable<Numeric>,
            liquidate_fee -> Nullable<Numeric>,
            pre_store_fee -> Nullable<Numeric>,
            record_version -> Nullable<Varchar>,
            create_by -> Nullable<Varchar>,
            update_by -> Nullable<Varchar>,
            create_time -> Timestamp,
            update_time -> Timestamp,
            is_delete -> Nullable<Bool>,
            delete_at -> Nullable<Timestamp>,
            comment -> Nullable<Text>,
            total_fee -> Nullable<Numeric>,
        }
    }

    diesel::table! {
        basic.t_room_info_detail (id) {
            id -> Int8,
            room_number -> Nullable<Varchar>,
            water_meter_num_before -> Nullable<Int8>,
            water_meter_num -> Nullable<Int8>,
            water_meter_sub -> Nullable<Int8>,
            electricity_meter_num_before -> Nullable<Int8>,
            electricity_meter_num -> Nullable<Int8>,
            electricity_meter_sub -> Nullable<Int8>,
            month_version -> Nullable<Varchar>,
            comment -> Nullable<Varchar>,
            create_by -> Nullable<Varchar>,
            update_by -> Nullable<Varchar>,
            create_time -> Timestamp,
            update_time -> Timestamp,
            is_delete -> Bool,
            room_owner_name -> Nullable<Varchar>,
            delete_at -> Nullable<Timestamp>,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(
        posts,
        t_owner_basic_info,
        t_price_basic,
        t_property_fee_detail,
        t_room_info_detail,
    );
}
