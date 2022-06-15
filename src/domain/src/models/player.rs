use crate::game::maths::Vector3;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub address: usize,
    pub name: String,
    pub id: String,
    pub location: Vector3,
    pub direction: f32,
    pub is_local: bool,
    pub last_aggressor: Option<String>,
    pub is_dead: bool,
}
