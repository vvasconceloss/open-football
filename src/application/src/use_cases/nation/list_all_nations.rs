use crate::{errors::ApplicationError, repositories::nation_repo::NationRepository};
use domain::entities::nation::Nation;

pub struct ListAllNations<'a, Repository: NationRepository> {
    repo: &'a Repository,
}

impl<'a, Repository: NationRepository> ListAllNations<'a, Repository> {
    pub fn execute(&self) -> Result<Vec<Nation>, ApplicationError> {
        let nations = self.repo.list_all()?;
        Ok(nations)
    }
}
