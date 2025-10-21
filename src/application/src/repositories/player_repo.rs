use crate::errors::ApplicationError;
use domain::entities::{
    nation::NationId,
    player::{Player, PlayerId},
};

pub trait PlayerRepository {
    fn save(&self, player: &Player) -> Result<(), ApplicationError>;
    fn find_by_id(&self, id: &PlayerId) -> Result<Option<Player>, ApplicationError>;
    fn list_by_nation(&self, id: &NationId) -> Result<Vec<Player>, ApplicationError>;
}
