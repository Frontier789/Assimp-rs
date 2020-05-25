#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;
use std::os::raw::c_uint;

use super::mesh::aiMesh;

#[repr(C)]
pub struct aiNode {
    dummy: i32,
}
#[repr(C)]
pub struct aiMaterial {
    dummy: i32,
}
#[repr(C)]
pub struct aiAnimation {
    dummy: i32,
}
#[repr(C)]
pub struct aiTexture {
    dummy: i32,
}
#[repr(C)]
pub struct aiCamera {
    dummy: i32,
}
#[repr(C)]
pub struct aiLight {
    dummy: i32,
}
#[repr(C)]
pub struct aiMetadata {
    dummy: i32,
}

// -------------------------------------------------------------------------------
/** The root structure of the imported data.
 *
 *  Everything that was imported from the given file can be accessed from here.
 *  Objects of this class are generally maintained and owned by Assimp, not
 *  by the caller. You shouldn't want to instance it, nor should you ever try to
 *  delete a given scene on your own.
 */
// -------------------------------------------------------------------------------
#[repr(C)]
pub struct aiScene {
    /** Any combination of the AI_SCENE_FLAGS_XXX flags. By default
     * this value is 0, no flags are set. Most applications will
     * want to reject all scenes with the AI_SCENE_FLAGS_INCOMPLETE
     * bit set.
     */
    pub mFlags: c_uint,

    /** The root node of the hierarchy.
     *
     * There will always be at least the root node if the import
     * was successful (and no special flags have been set).
     * Presence of further nodes depends on the format and content
     * of the imported file.
     */
    pub mRootNode: *const aiNode,

    /** The number of meshes in the scene. */
    pub mNumMeshes: c_uint,

    /** The array of meshes.
     *
     * Use the indices given in the aiNode structure to access
     * this array. The array is mNumMeshes in size. If the
     * AI_SCENE_FLAGS_INCOMPLETE flag is not set there will always
     * be at least ONE material.
     */
    pub mMeshes: *const *const aiMesh,

    /** The number of materials in the scene. */
    pub mNumMaterials: c_uint,

    /** The array of materials.
     *
     * Use the index given in each aiMesh structure to access this
     * array. The array is mNumMaterials in size. If the
     * AI_SCENE_FLAGS_INCOMPLETE flag is not set there will always
     * be at least ONE material.
     */
    pub mMaterials: *const *const aiMaterial,

    /** The number of animations in the scene. */
    pub mNumAnimations: c_uint,

    /** The array of animations.
     *
     * All animations imported from the given file are listed here.
     * The array is mNumAnimations in size.
     */
    pub mAnimations: *const *const aiAnimation,

    /** The number of textures embedded into the file */
    pub mNumTextures: c_uint,

    /** The array of embedded textures.
     *
     * Not many file formats embed their textures into the file.
     * An example is Quake's MDL format (which is also used by
     * some GameStudio versions)
     */
    pub mTextures: *const *const aiTexture,

    /** The number of light sources in the scene. Light sources
     * are fully optional, in most cases this attribute will be 0
     */
    pub mNumLights: c_uint,

    /** The array of light sources.
     *
     * All light sources imported from the given file are
     * listed here. The array is mNumLights in size.
     */
    pub mLights: *const *const aiLight,

    /** The number of cameras in the scene. Cameras
     * are fully optional, in most cases this attribute will be 0
     */
    pub mNumCameras: c_uint,

    /** The array of cameras.
     *
     * All cameras imported from the given file are listed here.
     * The array is mNumCameras in size. The first camera in the
     * array (if existing) is the default camera view into
     * the scene.
     */
    pub mCameras: *const *const aiCamera,

    /**
     *  @brief  The global metadata assigned to the scene itself.
     *
     *  This data contains global metadata which belongs to the scene like
     *  unit-conversions, versions, vendors or other model-specific data. This
     *  can be used to store format-specific metadata as well.
     */
    pub mMetaData: *const aiMetadata,

    mPrivate: *const c_void,
}
