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
-   [x] **Chapter 5**: Surface Normals and Multiple Objects
-   [x] **Chapter 6**: Antialiasing (Random Sampling)
-   [x] **Chapter 7**: Diffuse Materials
-   [x] **Chapter 8**: Metal
-   [x] **Chapter 9**: Dielectrics
-   [x] **Chapter 10**: Positionable Camera
-   [x] **Chapter 11**: Defocus Blur
-   [x] **Chapter 12**: Where Next?

## Usage

To run the project and generate a rendered image:

```bash
cargo run --release
```

## My Final Render (Example)

![Final Render](./final_render.png)
