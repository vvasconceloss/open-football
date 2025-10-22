use uuid::Uuid;

pub struct CreateNationCommand {
    pub name: String,
    pub reputation: u8,
}

pub struct FindNationByIdCommand {
    pub id: Uuid,
}
