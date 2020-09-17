# Assimp-rs
Rust bindings for the famous Open Asset Import library

Only very basic functionality is provided, positions, UV-coordinates and normals are loaded but materials, bones etc. not.
The API is 1-1 transfer of the Assimp C API (documentation included)

Two convenience functions are provided `aiImportFileToMesh` and `aiImportFileToMeshes` which use OOP Rust.
