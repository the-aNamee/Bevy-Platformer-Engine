use bevy::prelude::*;
use serde::Deserialize;
use ron;
use zip::result;
use std::{fs, io};

const OBJECT_FILE_EXTENSION: &str = "object";
const OBJECT_FOLDER_PATH: &str = "objects";

mod file;

#[derive(Resource)]
pub struct AllObjectData(Vec<ObjectData>);

#[derive(Debug)]
pub struct ObjectData {
    name: String,
    discription: String,
    sprites: Vec<Handle<Image>>
}

#[derive(Deserialize, Debug, Default)]
pub struct ObjectFileData {
    name: String,
    discription: String
}

pub fn load_all_objects_system(
    mut images: ResMut<Assets<Image>>,
    mut object_data_resource: ResMut<AllObjectData>
) {
    let mut entries = fs::read_dir(OBJECT_FOLDER_PATH).unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();

    for entry in entries {
        let path = entry.to_str().unwrap_or("");
        if path.ends_with(OBJECT_FILE_EXTENSION) {
            println!("{path}");
            let object_data = ObjectFileData::load_object_data_from_file(path);
            if object_data.is_none() {
                println!("Object data is none");
                continue;
            }
            let object_data = object_data.unwrap();

            // This is where we turn object data into the other thingy.
            let mut object_theory = object_data.into_object_data();
            
            // Present me just became future me. Present me is dealing with it.
            let mut loop_num = 0;
            loop {
                let image = file::extract_image_from_zip(path, &format!("{loop_num}.png"));
                if image.is_none() {
                    break;
                }
                let image_handle = images.add(image.unwrap());
                object_theory.sprites.push(image_handle);
                loop_num += 1;
            }

            // println!("{:?}", object_theory);
            object_data_resource.0.push(object_theory);
        }
    }
}

pub struct  ObjectManagerPlugin;

impl Plugin for ObjectManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_all_objects_system)
            .insert_resource(AllObjectData::empty());
    }
}

impl ObjectFileData {
    fn load_object_data_from_file(path: &str) -> Option<ObjectFileData> {
        let string = file::extract_file_from_zip(path, "object.ron");
        let s: Result<ObjectFileData, ron::de::SpannedError> = ron::from_str(&string);
        if s.is_ok() {
            let result = s.unwrap();
            Some(result)
        } else {
            None
        }
    }

    fn into_object_data(&self) -> ObjectData {
        let name = self.name.to_string();
        let discription = self.discription.to_string();
        return ObjectData {
            name,
            discription,
            sprites: Vec::new() // I am not going to deal with juggling the asset server, future me can deal with it.
        };
    }
}

impl AllObjectData {
    fn empty() -> AllObjectData {
        return AllObjectData(Vec::new());
    }
}
