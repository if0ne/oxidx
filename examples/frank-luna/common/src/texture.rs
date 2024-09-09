use std::path::PathBuf;

use oxidx::dx::Resource;

#[derive(Debug)]
pub struct Texture {
    pub name: String,
    pub filename: PathBuf,
    pub image: Resource,
    pub upload_buffer: Resource,
}
