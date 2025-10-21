use crate::errors::ApplicationError;
use domain::entities::nation::{Nation, NationId};

pub trait NationRepository {
    fn save(&self, nation: &Nation) -> Result<(), ApplicationError>;
    fn find_by_id(&self, id: &NationId) -> Result<Option<Nation>, ApplicationError>;
    fn list_all(&self) -> Result<Vec<Nation>, ApplicationError>;
}
