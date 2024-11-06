
// extern crate nalgebra_glm as glm;
use std::env::Args;
use nalgebra::Vector3;
use tom_CM::run;

use tom_CM::gid::{
    generate_network_tetrahedron_for_gid, get_hexahedra8_of_triangle, get_msh, get_res,
};

fn main() {
    // hello();

    // let mesh = get_msh("./data/gid/box/box.msh");
    // let res = get_res("./data/gid/box/box.res");
    // // let T4 = generate_network_tetrahedron_for_gid(mesh, res);

    // let triangles =get_hexahedra8_of_triangle(mesh, res);
    // pollster::block_on(run(triangles));

    let mut vec = Vector3::new(1.0, 2.0, 3.0);
    vec.x = 10.0;
    vec.y += 30.0;
    assert_eq!(vec.x, 10.0);
    assert_eq!(vec.y + 100.0, 132.0);
}
