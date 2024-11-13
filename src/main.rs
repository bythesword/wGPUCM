// extern crate nalgebra_glm as glm;
// use nalgebra::{dvector, Matrix3, Vector3};
use simple_delaunay_lib::delaunay_2d::{
    delaunay_struct_2d::DelaunayStructure2D, simplicial_struct_2d::Node,
};
use std::env::Args;
use tom_CM::run;
// extern crate nalgebra_glm as glm;
use cgmath::{dot, InnerSpace, Matrix3, Matrix4, SquareMatrix, Vector3, Vector4};
use tom_CM::gid::{
    generate_contour_of_trigangles, generate_network_tetrahedron_for_gid,
    get_hexahedra8_of_triangle, get_msh, get_res, get_res_by_site_frame_id,
    get_tetrahedra4_of_triangle, get_value_point_of_AB,
};

fn main() {
    let mesh = get_msh("./data/gid/box/box.msh");
    let res = get_res("./data/gid/box/box.res");
    // let mesh = get_msh("./data/gid/box/t4.msh");
    // let res = get_res("./data/gid/box/t4.res");

    let T4 = generate_network_tetrahedron_for_gid(&mesh, &res);

    let vvv: f32 = 11.1;
    let trangles_of_contour = generate_contour_of_trigangles([vvv].to_vec(), &T4, &mesh, &res);
    // let abc = get_res_by_site_frame_id(&res, "a.m", "1", 4);
    // print!("id(4)={}",abc);

    // let triangles =get_hexahedra8_of_triangle(mesh, res);
    // let triangles = get_tetrahedra4_of_triangle(&mesh, &res);
    let triangles = trangles_of_contour
        .get("a.m")
        .unwrap()
        .get("1")
        .unwrap()
        .get(&vvv.to_string())
        .unwrap();
    // print!("{:#?}", triangles);
    pollster::block_on(run(triangles.to_vec()));
}
