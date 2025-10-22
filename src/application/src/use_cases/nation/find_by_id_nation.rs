use crate::{
    commands::nation_commands::FindNationByIdCommand, errors::ApplicationError,
    repositories::nation_repo::NationRepository,
};
use domain::entities::nation::{Nation, NationId};

pub struct FindNationById<'a, Repository: NationRepository> {
    repo: &'a Repository,
}

impl<'a, Repository: NationRepository> FindNationById<'a, Repository> {
    pub fn execute(&self, command: FindNationByIdCommand) -> Result<Nation, ApplicationError> {
        let nation_id = NationId::from(command.id);

        match self.repo.find_by_id(&nation_id)? {
            Some(nation) => Ok(nation),
            None => Err(ApplicationError::EntityNotFound(format!(
                "A nation with ID {} was not found.",
                nation_id
            ))),
        }
    }
}
