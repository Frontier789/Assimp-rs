mod cimport;
mod data;
mod postprocess;

pub use cimport::*;
pub use data::*;
pub use postprocess::*;

fn main() {
    let ptr = aiImportFile("cube.obj", aiPostProcessSteps::Triangulate);
    unsafe {
        println!("Meshes: {}", (*ptr).mNumMeshes);
        let n = (*(*(*ptr).mMeshes)).mNumVertices as usize;
        println!("Vertices of mesh 0: {}", n);
        for i in 0..n {
            println!("v_{} = {:?}", i, *(*(*(*ptr).mMeshes)).mVertices.add(i));
        }
        let n = (*(*(*ptr).mMeshes)).mNumFaces as usize;
        println!("Faces of mesh 0: {}", n);
        for i in 0..n {
            let face = &*((*(*(*ptr).mMeshes)).mFaces.add(i));
            for k in 0..face.mNumIndices as usize {
                if k > 0 {
                    print!("/");
                }
                print!("{}", *face.mIndices.add(k));
            }
            println!();
        }
    }
    aiReleaseImport(ptr);
}
