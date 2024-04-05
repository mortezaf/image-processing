use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba, RgbaImage, Rgb};
use rayon::prelude::*;

const NUM_THREADS: usize = 4;

fn main() {
    // Read the image data from the file
    let img_path = "/home/morteza/Projects/rust/ip/image-processing/src/image.jpg";
    let img = image::open(img_path).expect("Failed to open image.");

    let width = img.width();
    let height = img.height();

    // Define the number of chunks (sections) for processing
    let num_chunks = NUM_THREADS;

    // Calculate the height of each chunk
    let chunk_height = height / num_chunks as u32;

    // Process each chunk concurrently using Rayon's parallel iterators
    let processed_chunks: Vec<_> = (0..num_chunks)
        .into_par_iter()
        .map(|chunk_index| {
            let start_y = chunk_index * chunk_height as usize;
            let end_y = if chunk_index < num_chunks - 1 {
                start_y + chunk_height as usize
            } else {
                height as usize
            };
            process_chunk(&img, 0, start_y as u32, width, end_y as u32)
        })
        .collect();

    // Combine the processed chunks to reconstruct the final image in RGB format
    let mut final_image = image::DynamicImage::new_rgb8(width, height).to_rgb8();
    for (chunk_index, chunk) in processed_chunks.into_iter().enumerate() {
        let start_y = chunk_index * chunk_height as usize;
        let _end_y = if chunk_index < num_chunks - 1 {
            start_y + chunk_height as usize
        } else {
            height as usize
        };
        for (x, y, pixel) in chunk.enumerate_pixels() {
            final_image.put_pixel(x, y + start_y as u32, Rgb([pixel[0], pixel[1], pixel[2]]));
        }
    }

    // Save the final image in JPEG format
    final_image
        .save_with_format("output.jpg", image::ImageFormat::Jpeg)
        .expect("Failed to save image.");
}

// Function to process a chunk of the image
fn process_chunk(img: &DynamicImage, start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> RgbaImage {
    let mut processed_chunk = ImageBuffer::new(end_x - start_x, end_y - start_y);
    for x in start_x..end_x {
        for y in start_y..end_y {
            let pixel = img.get_pixel(x, y);
            let processed_pixel = manipulate_color(pixel);
            processed_chunk.put_pixel(x - start_x, y - start_y, processed_pixel);
        }
    }
    processed_chunk
}

// Function to manipulate the color of a pixel (simple inversion in this example)
fn manipulate_color(pixel: Rgba<u8>) -> Rgba<u8> {
    Rgba([255 - pixel[0], 255 - pixel[1], 255 - pixel[2], pixel[3]])
}