#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use std::ffi::CString;

use data::*;
use glui::tools::mesh::{Mesh, MeshFace};
use postprocess::aiPostProcessSteps;

mod raw_assimp {
    use aiScene;
    use std::os::raw::{c_char, c_uint};

    #[link(name = "c:/Projects/rust/assimp/assimp-vc141-mt")]
    extern "C" {
        pub fn aiImportFile(pFile: *const c_char, pFlags: c_uint) -> *const aiScene;
        pub fn aiReleaseImport(pScene: *const aiScene);
    }
}

// --------------------------------------------------------------------------------
/** Reads the given file and returns its content.
 *
 * If the call succeeds, the imported data is returned in an aiScene structure.
 * The data is intended to be read-only, it stays property of the ASSIMP
 * library and will be stable until aiReleaseImport() is called. After you're
 * done with it, call aiReleaseImport() to free the resources associated with
 * this file. If the import fails, NULL is returned instead. Call
 * aiGetErrorString() to retrieve a human-readable error text.
 * @param pFile Path and filename of the file to be imported,
 *   expected to be a null-terminated c-string. NULL is not a valid value.
 * @param pFlags Optional post processing steps to be executed after
 *   a successful import. Provide a bitwise combination of the
 *   #aiPostProcessSteps flags.
 * @return Pointer to the imported data or NULL if the import failed.
 */
pub fn aiImportFile(pFile: &str, pFlags: aiPostProcessSteps) -> *const aiScene {
    let cstr = CString::new(pFile).unwrap();
    unsafe { raw_assimp::aiImportFile(cstr.as_ptr(), pFlags.into()) }
}

// --------------------------------------------------------------------------------
/** Releases all resources associated with the given import process.
 *
 * Call this function after you're done with the imported data.
 * @param pScene The imported data to release. NULL is a valid value.
 */
pub fn aiReleaseImport(pScene: *const aiScene) {
    unsafe {
        raw_assimp::aiReleaseImport(pScene);
    }
}

pub fn aiImportFileToMesh(file: &str) -> Option<Mesh> {
    let mut pts = vec![];
    let mut tpt = vec![];
    let mut faces = vec![];
    let mut normals = vec![];
    let ptr = aiImportFile(
        file,
        aiPostProcessSteps::Triangulate
            | aiPostProcessSteps::GenSmoothNormals
            | aiPostProcessSteps::GenUVCoords
            | aiPostProcessSteps::FlipUVs,
    );
    if ptr.is_null() {
        return None;
    }
    unsafe {
        let mesh_count = (*ptr).mNumMeshes as usize;
        // println!("Meshes: {}", mesh_count);

        let mut ind_base = 0;

        for j in 0..mesh_count {
            let mesh = &*(*(*ptr).mMeshes.add(j));
            let vertex_count = mesh.mNumVertices as usize;
            let face_count = mesh.mNumFaces as usize;

            // println!("Vertices of mesh: {}", vertex_count);

            for i in 0..vertex_count {
                pts.push(*mesh.mVertices.add(i));
                normals.push(*mesh.mNormals.add(i));
                tpt.push((*mesh.mTextureCoords[0].add(i)).xy())
            }

            for i in 0..face_count {
                let face = &*mesh.mFaces.add(i);
                let a = *face.mIndices.add(0) + ind_base;
                let b = *face.mIndices.add(1) + ind_base;
                let c = *face.mIndices.add(2) + ind_base;
                faces.push(MeshFace::new(a, b, c));
            }

            ind_base += vertex_count as u32;
        }
    }
    aiReleaseImport(ptr);

    Some(Mesh {
        points: pts,
        normals: Some(normals),
        faces,
        uvcoords: Some(tpt),
    })
}

pub fn aiImportFileToMeshes(file: &str) -> Option<Vec<Mesh>> {
    let ptr = aiImportFile(
        file,
        aiPostProcessSteps::Triangulate
            | aiPostProcessSteps::GenSmoothNormals
            | aiPostProcessSteps::GenUVCoords,
    );
    if ptr.is_null() {
        return None;
    }
    let mut meshes;

    unsafe {
        let mesh_count = (*ptr).mNumMeshes as usize;
        // println!("Meshes: {}", mesh_count);
        meshes = Vec::with_capacity(mesh_count);

        for j in 0..mesh_count {
            let mesh = &*(*(*ptr).mMeshes.add(j));
            let vertex_count = mesh.mNumVertices as usize;
            let face_count = mesh.mNumFaces as usize;

            // println!("vc: {}, fc: {}", vertex_count, face_count);

            let mut uvs = vec![];
            let mut pts = vec![];
            let mut faces = vec![];
            let mut normals = vec![];

            // println!("Vertices of mesh: {}", vertex_count);

            for i in 0..vertex_count {
                pts.push(*mesh.mVertices.add(i));
                normals.push(*mesh.mNormals.add(i));
                uvs.push((*mesh.mTextureCoords[0].add(i)).xy());
            }

            let mut mn = 10000000;
            let mut mx = 0;

            for i in 0..face_count {
                let face = &*mesh.mFaces.add(i);
                let a = *face.mIndices.add(0);
                let b = *face.mIndices.add(1);
                let c = *face.mIndices.add(2);
                faces.push(MeshFace::new(a, b, c));
                mn = a.min(b.min(c.min(mn)));
                mx = a.max(b.max(c.max(mx)));
            }
            // println!("face range: {}..{}", mn, mx);

            meshes.push(Mesh {
                points: pts,
                normals: Some(normals),
                faces,
                uvcoords: Some(uvs),
            });
        }
    }
    aiReleaseImport(ptr);

    Some(meshes)
}
