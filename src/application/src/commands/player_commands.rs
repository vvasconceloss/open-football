use uuid::Uuid;

pub struct CreatePlayerCommand {
    pub last_name: String,
    pub first_name: String,
    pub nation: Uuid,
    pub position: String,
    pub growth_potential: f32,
    pub birth_date: String,
    pub attributes: String,
}
