# Ray Tracing in One Weekend

This project is a simple implementation of the concepts from Peter Shirley's _Ray Tracing in One Weekend_ book series. The series is an excellent introduction to ray tracing and consists of three books:

1. [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
2. [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
3. [Ray Tracing: The Rest of Your Life](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)

This implementation focuses on the first book, and the code is written in **Rust**.

## Resources

-   [PPM Viewer](https://www.cs.rhodes.edu/welshc/COMP141_F16/ppmReader.html): A simple tool to view the rendered `.ppm` images.

## Progress

-   [x] **Chapter 1**: Output an Image
-   [x] **Chapter 2**: Vec3
-   [x] **Chapter 3**: Rays, a Simple Camera, and Background
-   [x] **Chapter 4**: Adding a Sphere
-   [ ] **Chapter 5**: Surface Normals and Multiple Objects _(Work in Progress)_

## Usage

To run the project and generate a rendered image:

```bash
cargo run > image.ppm
```

This will produce an image file named image.ppm. Use the PPM Viewer to view the image.

> Note: In the future, I may update the project to directly export the rendered image to a file instead of relying on stdout redirection.
