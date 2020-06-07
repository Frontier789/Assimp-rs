#![feature(clamp)]
#![allow(dead_code)]
extern crate assimp;
extern crate downcast_rs;
extern crate gl;
extern crate glui;
extern crate glui_proc;
extern crate rand;

use assimp::{aiImportFile, aiPostProcessSteps, aiReleaseImport};

use glui::graphics::{DrawResources, DrawShaderSelector, RenderCommand, RenderSequence};
use glui::mecs::*;
use glui::tools::*;

fn load_object() -> RenderSequence {
    // let pts = tools::parsurf(
    //     |x, y| Vec3::pol(1.0, x * PI - PI / 2.0, y * 2.0 * PI),
    //     30,
    //     30,
    // );
    // let tpt = tools::parsurf(|x, y| Vec2::new(x, y), 30, 30);
    // let clr = tools::parsurf(|_, _| Vec4::WHITE, 30, 30);

    let mut pts = vec![];
    let mut indices = vec![];
    let mut normals = vec![];
    let ptr = aiImportFile(
        "suzanne.obj",
        aiPostProcessSteps::Triangulate | aiPostProcessSteps::GenSmoothNormals,
    );
    unsafe {
        let mesh_count = (*ptr).mNumMeshes as usize;
        println!("Meshes: {}", mesh_count);

        for j in 0..mesh_count {
            let mesh = &*(*(*ptr).mMeshes.add(j));
            let vertex_count = mesh.mNumVertices as usize;
            let face_count = mesh.mNumFaces as usize;

            println!("Vertices of mesh: {}", vertex_count);

            for i in 0..vertex_count {
                pts.push(*mesh.mVertices.add(i));
                normals.push(*mesh.mNormals.add(i));
            }

            for i in 0..face_count {
                let face = &*mesh.mFaces.add(i);
                for k in 0..3 {
                    indices.push(*face.mIndices.add(k));
                }
            }
        }
    }
    aiReleaseImport(ptr);

    let clr: Vec<Vec4> = normals
        .iter()
        .map(|n| Vec4::grey(n.dot(Vec3::new(1.0, 1.0, 1.0).sgn()).clamp(0.0, 1.0)))
        .collect();

    let pbuf = Buffer::from_vec(pts);
    let cbuf = Buffer::from_vec(clr);
    let mut vao = VertexArray::new();
    vao.attrib_buffer(0, &pbuf);
    vao.attrib_buffer(1, &cbuf);
    vao.set_indices_vec(indices);

    let mut render_seq = RenderSequence::new();

    render_seq.add_buffer(pbuf.into_base_type());
    render_seq.add_buffer(cbuf.into_base_type());

    render_seq.add_command(RenderCommand {
        vao,
        mode: DrawMode::Triangles,
        shader: DrawShaderSelector::DefaultColored,
        uniforms: vec![
            // Uniform::Matrix4(
            //     "uv_matrix".to_owned(),
            //     Mat4::from_arr_arr([
            //         [1.0, 0.0, 0.0, 0.0],
            //         [0.0, -1.0, 0.0, 1.0],
            //         [0.0, 0.0, 1.0, 0.0],
            //         [0.0, 0.0, 0.0, 1.0],
            //     ]),
            // ),
            // Uniform::Texture2D(
            //     "tex".to_owned(),
            //     draw_resources.texture_id("images/stone").unwrap(),
            // ),
        ],
        transparent: true,
        instances: 1,
    });

    render_seq
}

fn grid() -> RenderSequence {
    let mut pts = vec![];
    let mut clr = vec![];

    for x in -10..11 {
        pts.push(Vec3::new(x as f32, -10.0, -0.01));
        pts.push(Vec3::new(x as f32, 10.0, -0.01));
        pts.push(Vec3::new(-10.0, x as f32, -0.01));
        pts.push(Vec3::new(10.0, x as f32, -0.01));
        clr.push(Vec4::new(0.3, 0.3, 0.3, 1.0));
        clr.push(Vec4::new(0.3, 0.3, 0.3, 1.0));
        clr.push(Vec4::new(0.3, 0.3, 0.3, 1.0));
        clr.push(Vec4::new(0.3, 0.3, 0.3, 1.0));
    }

    let pbuf = Buffer::from_vec(pts);
    let cbuf = Buffer::from_vec(clr);
    let mut vao = VertexArray::new();
    vao.attrib_buffer(0, &pbuf);
    vao.attrib_buffer(1, &cbuf);

    let mut render_seq = RenderSequence::new();

    render_seq.add_buffer(pbuf.into_base_type());
    render_seq.add_buffer(cbuf.into_base_type());

    render_seq.add_command(RenderCommand {
        vao,
        mode: DrawMode::Lines,
        shader: DrawShaderSelector::DefaultColored,
        uniforms: vec![],
        transparent: false,
        instances: 1,
    });

    render_seq
}

fn main() {
    let mut w: World = World::new_win(Vec2::new(640.0, 480.0), "Vmodel", Vec3::grey(0.1));

    let draw_res = DrawResources::new(w.window_info().unwrap()).unwrap();

    let object_seq = load_object();
    let e = w.as_static_mut().entity();
    w.as_static_mut().add_component(
        e,
        DrawComponent {
            render_seq: object_seq,
            model_matrix: Mat4::identity(),
        },
    );

    let grid_seq = grid();
    let e = w.as_static_mut().entity();
    w.as_static_mut().add_component(
        e,
        DrawComponent {
            render_seq: grid_seq,
            model_matrix: Mat4::offset(Vec3::new(0.0, 0.0, -1.0)),
        },
    );

    // unsafe {
    //     gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    // }

    let cam = Camera::new(ModelViewController::new(&mut w));

    let ds = DrawSystem {
        camera: cam,
        resources: draw_res,
    };

    let dsid = w.add_system(ds);
    w.make_system_ui_aware(dsid);

    w.run();
}
