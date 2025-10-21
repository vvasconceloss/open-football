use crate::{
    commands::nation_commands::CreateNationCommand, errors::ApplicationError,
    repositories::nation_repo::NationRepository,
};
use domain::{
    entities::nation::{Nation, NationId},
    value_objects::{nation::name::NationName, reputation::Reputation},
};

pub struct CreateNation<'a, Repository: NationRepository> {
    repo: &'a Repository,
}

impl<'a, Repository: NationRepository> CreateNation<'a, Repository> {
    pub fn execute(&self, command: CreateNationCommand) -> Result<NationId, ApplicationError> {
        let name = NationName::new(command.name)?;
        let reputation = Reputation::new(command.reputation)?;

        let nation = Nation::new(name, reputation)?;

        self.repo.save(&nation)?;
        Ok(nation.id().clone())
    }
}
