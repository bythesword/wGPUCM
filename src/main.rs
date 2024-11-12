// extern crate nalgebra_glm as glm;
// use nalgebra::{dvector, Matrix3, Vector3};
use std::env::Args;
use tom_CM::run;
// extern crate nalgebra_glm as glm;
use cgmath::{InnerSpace, Matrix3, Matrix4, Vector3};
use tom_CM::gid::{
    generate_contour_of_trigangles, generate_network_tetrahedron_for_gid,
    get_hexahedra8_of_triangle, get_msh, get_res, get_res_by_site_frame_id,
    get_tetrahedra4_of_triangle, get_value_point_of_AB,
};

fn main() {
    let mut v = Vector3::new(1.0, 2.0, 3.0);
    v = v * 2.0;
    let m: Matrix3<f32> = Matrix3::new(11.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
    let mut m1: Matrix4<f32> = Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let result = m * v;
    v.normalize();
    m1.x[0]=2.0;
    println!("{:?}\n", result);
    println!("{:?}", m1);
    return;

    // let mesh = get_msh("./data/gid/box/box.msh");
    let mesh = get_msh("./data/gid/box/t4.msh");
    let res = get_res("./data/gid/box/t4.res");
    let T4 = generate_network_tetrahedron_for_gid(&mesh, &res);

    let trangles_of_contour = generate_contour_of_trigangles([6.1].to_vec(), &T4, &mesh, &res);
    // let abc = get_res_by_site_frame_id(&res, "a.m", "1", 4);
    // print!("id(4)={}",abc);

    // let triangles =get_hexahedra8_of_triangle(mesh, res);
    // let triangles = get_tetrahedra4_of_triangle(&mesh, &res);
    let triangles = trangles_of_contour
        .get("a.m")
        .unwrap()
        .get("1")
        .unwrap()
        .get("6.1")
        .unwrap();
    print!("{:#?}", triangles);
    pollster::block_on(run(triangles.to_vec()));
}
