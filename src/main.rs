use cgmath::Vector2;
// extern crate nalgebra_glm as glm;
use nalgebra::Vector3;
use std::env::Args;
use tom_CM::run;

use tom_CM::gid::{
    generate_network_tetrahedron_for_gid, get_hexahedra8_of_triangle, get_msh, get_res,
    get_res_by_site_frame_id, get_tetrahedra4_of_triangle, get_value_point_of_AB,
};

fn main() {
    // hello();

    // let mesh = get_msh("./data/gid/box/box.msh");
    let mesh = get_msh("./data/gid/box/t4.msh");
    let res = get_res("./data/gid/box/t4.res");
    let T4 = generate_network_tetrahedron_for_gid(&mesh, &res);

    // let abc = get_res_by_site_frame_id(&res, "a.m", "1", 4);
    // print!("id(4)={}",abc);

    // let triangles =get_hexahedra8_of_triangle(mesh, res);
    
    // let triangles = get_tetrahedra4_of_triangle(&mesh, &res);
    // pollster::block_on(run(triangles));

    // let vec1 = Vector3::new(1.0, 0.0, 0.0);
    // let vec2 = Vector3::new(11.0, 0.0, 0.0);
    // let u1 = 2.0;
    // let u2 = 12.0;
    // let u3 = 4.0;
    // let vector1 = vec2 - vec1;

    // let v = (u3 - u1) / (u2 - u1);
    // let mut vector2 = vector1 * v;
    // print!("{:#?}\n", vector2);
    // vector2 = vec1 + vector2;
    // print!("{:#?}\n", vector2);

    // let abc = get_value_point_of_AB(vec2, vec1, 0., 10., 2.);
    // print!("{:#?}", abc);
}
