use crate::game::maths::Vector3;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub id: String,
    pub location: Vector3,
}