# Assimp-rs
Rust bindings for the famous [Open Asset Import library](https://www.assimp.org/)

Only very basic functionality is provided, positions, UV-coordinates and normals are loaded but materials, bones etc. not.
The API is 1-1 transfer of the Assimp C API (documentation included)

Two convenience functions are provided `aiImportFileToMesh` and `aiImportFileToMeshes` which use OOP Rust.

## Dependency
The code can be compiled via `cargo build`, however the resulting binaries will need the Assimp library in a shared object form (.so on linux, .dll on Windows)
A shared library can be compiled from the Assimp source, see [Assimp](https://www.assimp.org/)
