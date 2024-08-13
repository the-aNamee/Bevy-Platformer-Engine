use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;


use bevy::render::render_resource::Extent3d;
use image::GenericImageView;



pub fn extract_file_from_zip(zip_path: &str, file_name: &str) -> String {
    // Open the ZIP file, handle errors by returning an empty string
    let zip_file = match File::open(zip_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open ZIP file.");
            return String::new(); // Correctly returning String in case of error
        }
    };
    
    // Load the ZIP data into a ZipArchive, handle errors by returning an empty string
    let mut zip_archive = match ZipArchive::new(zip_file) {
        Ok(archive) => archive,
        Err(_) => {
            println!("Failed to read ZIP archive.");
            return String::new(); // Correctly returning String in case of error
        }
    };
    
    // Extract the specified file from the ZIP archive, handle errors by returning an empty string
    let mut extracted_file = match zip_archive.by_name(file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to find file in ZIP archive.");
            return String::new(); // Correctly returning String in case of error
        }
    };
    
    // Read the file content into a string, handle errors by returning an empty string
    let mut content = String::new();
    if extracted_file.read_to_string(&mut content).is_err() {
        println!("Failed to read file content.");
        return String::new(); // Correctly returning String in case of error
    }
    
    // Return the content of the extracted file
    content
}



pub fn extract_image_from_zip(zip_path: &str, file_name: &str) -> Option<Image> {
    // Open the ZIP file, handle errors by returning an empty Image
    let zip_file = match File::open(zip_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open ZIP file.");
            return None; // Return default image in case of error
        }
    };

    // Load the ZIP data into a ZipArchive, handle errors by returning an empty Image
    let mut zip_archive = match ZipArchive::new(zip_file) {
        Ok(archive) => archive,
        Err(_) => {
            println!("Failed to read ZIP archive.");
            return None; // Return default image in case of error
        }
    };

    // Extract the specified file from the ZIP archive, handle errors by returning an empty Image
    let mut extracted_file = match zip_archive.by_name(file_name) {
        Ok(file) => file,
        Err(_) => {
            // println!("Failed to find file in ZIP archive.");
            return None; // Return default image in case of error
        }
    };

    // Read the file content into a Vec<u8>, handle errors by returning an empty Image
    let mut content: Vec<u8> = Vec::new();
    if extracted_file.read_to_end(&mut content).is_err() {
        println!("Failed to read file content.");
        return None; // Return default image in case of error
    }

    Some(png_data_to_bevy_image(content))

    // // Decode the image data
    // let dynamic_image = match image::load_from_memory(&content) {
    //     Ok(img) => img,
    //     Err(_) => {
    //         println!("Failed to decode image.");
    //         return Image::default(); // Return default image in case of error
    //     }
    // };

    // // Convert DynamicImage to Bevy's Image with the correct usage flags
    // let bevy_image = Image::from_dynamic(
    //     dynamic_image,
    //     false, // is_srgb
    //     RenderAssetUsages::TEXTURE_BINDING, // Default usage for most textures
    // );

    
}

fn png_data_to_bevy_image(png_data: Vec<u8>) -> Image {
    // Step 1: Decode the PNG data using the `image` crate
    let img = image::load_from_memory(&png_data).expect("Failed to load image from memory");

    // Step 2: Get the image dimensions
    let (width, height) = img.dimensions();

    // Step 3: Convert the image to RGBA8 format
    let rgba = img.to_rgba8();

    // Step 4: Create a Bevy Image object
    let texture = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        rgba.into_raw(),
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all()
    );

    texture
}