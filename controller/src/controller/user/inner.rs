use crate::dto::user::UserResultDto;
use repository::owner_info::OwnerBasicInfoPo;
use repository::user::UserPo;

pub trait ComputeUserResult{
    fn compute_user_result(self) -> UserResultDto;
}
impl ComputeUserResult for (UserPo, Option<Vec<OwnerBasicInfoPo>>){
    fn compute_user_result(self) -> UserResultDto {
        let user_po = self.0;
        let room_vec = self.1;
        let mut t_user:UserResultDto = user_po.into();
        if let Some(room_vec) = room_vec {
            t_user.binding_room_number = Some(room_vec.into_iter().map(|room| room.room_number).collect::<Vec<String>>());
        } else {
            t_user.binding_room_number = None
        }
        t_user
    }
}

impl ComputeUserResult for (UserPo, Option<Vec<String>>){
    fn compute_user_result(self) -> UserResultDto {
        let user_po = self.0;
        let room_vec = self.1;
        let mut t_user:UserResultDto = user_po.into();
        t_user.binding_room_number = room_vec;
        t_user
    }
}