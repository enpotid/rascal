use crate::backend::group::Group;

struct User {
    name: String,
    id: usize,
    password: [char; 256],
    groups: Vec<Group>,
}
