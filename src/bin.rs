mod cimport;
mod data;
mod postprocess;

pub use cimport::*;
pub use data::*;
pub use postprocess::*;

fn main() {
    let ptr = aiImportFile("cube.obj", aiPostProcessSteps::None);
    unsafe {
        println!("Meshes: {}", (*ptr).mNumMeshes);
        let n = (*(*(*ptr).mMeshes)).mNumVertices as usize;
        println!("Vertices of mesh 0: {}", n);
        for i in 0..n {
            println!("v_{} = {:?}", i, *(*(*(*ptr).mMeshes)).mVertices.add(i));
        }
    }
    aiReleaseImport(ptr);
}
