use std::env::Args;
use tom_CM::run;

use tom_CM::gid::{generate_network_tetrahedron_for_gid, get_msh, get_res,get_hexahedra8_of_triangle};

fn main() {
    
    // hello();

    let mesh = get_msh("./data/gid/box/box.msh");
    let res = get_res("./data/gid/box/box.res");
    // let T4 = generate_network_tetrahedron_for_gid(mesh, res);

    let triangles =get_hexahedra8_of_triangle(mesh, res);
    pollster::block_on(run(triangles));
}
