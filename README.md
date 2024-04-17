# Image Processing with Rust

This Rust code provides a simple example of parallel image processing using the Rayon library. It loads an image, splits it into chunks, processes each chunk concurrently, and then reconstructs the final image.

## Requirements
- Rust programming language and Cargo package manager
- The following dependencies are required:
  - `image` crate for image manipulation
  - `rayon` crate for parallel processing

## Usage
1. Clone the repository or download the code.
2. Make sure you have Rust and Cargo installed.
3. Update the `img_path` variable in `main()` function to point to your image file.
4. Set the `NUM_THREADS` constant to the desired number of threads for parallel processing.
5. Run the code using `cargo run`.

## Code Overview
- **`main()` Function**:
  - Opens an image file specified by `img_path`.
  - Divides the image into chunks based on the number of threads.
  - Processes each chunk concurrently using Rayon's parallel iterators.
  - Combines the processed chunks to reconstruct the final image.
  - Saves the final image in JPEG format as `output.jpg`.

- **`process_chunk()` Function**:
  - Takes a portion of the image (a chunk) and processes it.
  - Applies a simple manipulation to each pixel (inversion in this example).

- **`manipulate_color()` Function**:
  - Inverts the color of a pixel.

## Parallel Processing
The code utilizes Rayon's parallel iterators to distribute the image processing workload across multiple threads, improving performance.
