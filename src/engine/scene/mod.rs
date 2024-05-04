use crate::engine::gameobject::GameObject;

pub struct Scene<'a> {
    objects: Vec<GameObject<'a>>,
}