use crate::errors::ApplicationError;
use domain::entities::{
    club::{Club, ClubId},
    nation::NationId,
};

pub trait ClubRepository {
    fn save(&self, club: &Club) -> Result<(), ApplicationError>;
    fn find_by_id(&self, id: &ClubId) -> Result<Option<Club>, ApplicationError>;
    fn list_by_nation(&self, id: &NationId) -> Result<Vec<Club>, ApplicationError>;
}
