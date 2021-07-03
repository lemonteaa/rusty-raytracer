# rusty-raytracer
Raytracer for learning Computer Graphics

Run using the command `cargo run`. Current and planned branches:

* `master`: Skeleton/framework for the program.
* `simple-trace`: Implement a basic algorithm that shoots ray from the camera.
* `monte-carlo` (TBD): Implement some [Global Illumination](https://en.wikipedia.org/wiki/Global_illumination) style algorithms?

Todo list:

See the projects tag in github.

# Source code structure

* root `src` folder:
  * `main.rs`: Program entry point, hook into the cores, manage the progress bar, etc.
  * `core.rs`: Core rendering algorithms: `trace()`, top level code for dealing with materials (called "lighting model"), anti-aliasing.
  * `sample.rs`: Codes for constructing/setting up scene for test.
  * `util.rs`: Other stuffs, like color.
* `model` folder:
  * Main (`mod.rs` per Rust lang): Objects for 3D stuff in the scene - Scene Graph Model (is a tree), Material, and Ray.
  * `intersect.rs`: `Intersectable` trait to take care of the details of doing ray-object intersection (mechanism vs policy).
* `scene` folder:
  * Main: Main object for the whole scene, which contains the model/scene graph plus a bunch of "meta" stuffs.
  * `light.rs`: Different kinds of Lighting.
  * `camera.rs`: Camera class, manage the projection and view matrices and help to shoot a ray.
