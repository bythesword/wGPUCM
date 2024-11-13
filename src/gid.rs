//! GID解析
pub use gid::{
    generate_contour_of_trigangles, generate_network_tetrahedron_for_gid,
    get_hexahedra8_of_triangle, get_msh, get_res, get_res_by_site_frame_id,
    get_tetrahedra4_of_triangle, get_value_point_of_AB,
};
pub use gid::{mesh_source, resource_source};
mod gid {
    use cgmath::{InnerSpace, Matrix4, SquareMatrix, Vector3, Vector4};
    use simple_delaunay_lib::delaunay_2d::{
        delaunay_struct_2d::DelaunayStructure2D, simplicial_struct_2d::Node,
    };
    // use nalgebra::Vector3;
    // extern crate nalgebra_glm as glm;
    // use nalgebra-glm::dot;
    use std::{
        collections::HashMap,
        convert::TryInto,
        fs::{self, File, OpenOptions},
        hash::Hash,
        io::{BufRead, BufReader, Read, Write},
        string, u64, vec,
    };

    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use serde_json::{Result, Value};

    pub struct mesh_struct {
        name: String,
        dimension: u8,
        element_type: String,
        elements: HashMap<u64, Vec<u64>>,
    }
    pub struct position {}
    pub struct mesh_source {
        nodes: HashMap<u64, [f32; 3]>,
        meshs: HashMap<String, mesh_struct>,
    }

    pub struct resource_source {
        // result_componet: Vec<String>, //abc.u ,...
        result_componet: Vec<String>, //abc.u ,...
        result: HashMap<String, HashMap<String, HashMap<u64, f32>>>, //<"a.b",<"frame id of string",<node_id,value>>>
        frames: Vec<String>,                                         //帧列表,步长与名称是统一的
    }

    pub fn get_data_from_file(filename: &str) -> std::io::Lines<BufReader<File>> {
        let file = File::open(filename).unwrap();
        let lines = BufReader::new(file).lines(); //遍历所有行
        lines
    }
    pub fn get_msh(filename: &str) -> mesh_source {
        let lines = get_data_from_file(filename);
        let nodes: Vec<f32> = Vec::new();
        let _space_seperator = Regex::new(r"([ ]+)").expect("Invalid regex"); //多个空格的regex
                                                                              // let space_seperator = Regex::new(r"([ ]+)").expect("Invalid regex"); //多个空格的regex

        let mut is_mesh = false;
        let mut is_node = false;
        let mut is_element = false;

        let mut mesh_name: String = String::new();
        // let mut nodes: HashMap<u64, [f32; 3]> = HashMap::new();
        let meshs: HashMap<String, mesh_struct> = HashMap::new();
        let mut source: mesh_source = mesh_source {
            nodes: HashMap::new(),
            meshs,
        };
        // let one_mesh;
        for line in lines {
            if let Ok(data) = line {
                if data.contains("Mesh") {
                    let data_split: Vec<&str> = data.as_str().split("\"").collect();
                    if data_split.len() > 2 {
                        is_mesh = true;
                        mesh_name = String::from(data_split[1]);

                        let dimension_string: Vec<&str> =
                            data.as_str().split("Dimension").collect();
                        let d1: Vec<&str> = _space_seperator
                            .split(dimension_string[1])
                            .into_iter()
                            .collect();
                        let dimession: u8 = d1[1].to_string().parse().unwrap();

                        let elemnet_type_string: Vec<&str> =
                            data.as_str().split("Elemtype").collect();

                        let mut elemnet_type = String::from(elemnet_type_string[1]);
                        let _ = elemnet_type.replace("\r\n", "");
                        elemnet_type.remove(0);

                        // print!("mesh name is :{}", mesh_name);
                        let is_already_have_mesh = source.meshs.get_mut(&mesh_name);
                        if is_already_have_mesh.is_none() {
                            // let mut abc=mesh_struct {
                            //     name: mesh_name.clone(),
                            //     dimension:3,
                            //     element_type: "asd".to_owned(),
                            //     elements: HashMap::new(),
                            // };
                            // let mut elements = HashMap::new();
                            source.meshs.insert(
                                mesh_name.clone(),
                                mesh_struct {
                                    name: mesh_name.clone(),
                                    dimension: dimession,
                                    element_type: elemnet_type.to_owned(),
                                    elements: HashMap::new(),
                                    // elements,
                                },
                            );
                            // print!("  =======");
                        }

                        // print!("mesh name is :{}", mesh_name);
                    }
                    if is_mesh {
                        continue;
                    }
                }
                let lowcase_data = data.clone().to_lowercase();
                if lowcase_data.contains("coordinates") && is_node == false {
                    is_node = true;
                    continue;
                }
                if lowcase_data.contains("end coordinates") {
                    is_node = false;
                    continue;
                }
                if lowcase_data.contains("elements") && is_element == false {
                    is_element = true;
                    continue;
                }
                if lowcase_data.contains("elements") {
                    is_element = false;
                    continue;
                }
                if is_node == true {
                    let line_space: Vec<&str> =
                        _space_seperator.split(data.as_str()).into_iter().collect();
                    if line_space.len() == 5 {
                        let k = line_space[1].to_string().parse::<u64>().unwrap();

                        let v = [
                            line_space[2].to_string().parse::<f32>().unwrap(),
                            line_space[3].to_string().parse::<f32>().unwrap(),
                            line_space[4].to_string().parse::<f32>().unwrap(),
                        ];
                        source.nodes.insert(k, v);
                        // print!("{k},{:?}\n", v);
                    } else if line_space.len() == 5 {
                        let k = line_space[1].to_string().parse::<u64>().unwrap();

                        let v = [
                            line_space[2].to_string().parse::<f32>().unwrap(),
                            line_space[3].to_string().parse::<f32>().unwrap(),
                            0.0,
                        ];
                        source.nodes.insert(k, v);
                    }
                    // print!("{:?}\n", line_space);
                }
                if is_element == true {
                    let line_space: Vec<&str> =
                        _space_seperator.split(data.as_str()).into_iter().collect();
                    if line_space.len() > 3 {
                        let k = line_space[1].to_string().parse::<u64>().unwrap();
                        let mut v: Vec<u64> = vec![];
                        for i in 2..line_space.len() {
                            if line_space[i] != "" {
                                // print!("{:#?}", v);
                                v.push(line_space[i].to_string().parse::<u64>().unwrap());
                            }
                        }
                        let meshs = source.meshs.get_mut(&mesh_name);
                        // source.meshs.get(mesh_name);
                        if meshs.is_some() {
                            let mut a = meshs.unwrap().elements.insert(k.into(), v);

                            let aa = 1;
                        }
                    } else {
                    }
                    // print!("{:?}\n", line_space);
                }
            }
        }
        source
    }
    pub fn get_res(filename: &str) -> resource_source {
        let lines = get_data_from_file(filename);
        // let nodes: Vec<f32> = Vec::new();
        let _space_seperator = Regex::new(r"([ ]+)").expect("Invalid regex"); //多个空格的regex
                                                                              // let space_seperator = Regex::new(r"([ ]+)").expect("Invalid regex"); //多个空格的regex

        let mut is_result = false;
        let mut is_frame = false;
        let mut is_values = false;
        let mut is_componet = false;

        let mut result_name: String = String::new(); //当前的result的名称，每次result清空
        let mut component_name: Vec<String> = Vec::new(); //当前result的component name，每次component清空
        let mut frame_name: String = String::new(); //当前result的frame名称，

        let mut frame_name_list: HashMap<String, String> = HashMap::new();
        let mut result_componet_list = HashMap::new();
        let mut source: resource_source = resource_source {
            result_componet: Vec::new(),
            result: HashMap::new(),
            frames: Vec::new(),
        };
        // let one_mesh;
        for line in lines {
            if let Ok(data) = line {
                //result  name and frame name
                if data.contains("Result ") {
                    if component_name.len() > 0 {
                        // for i in 0..component_name.len() {
                        //     source.result_componet.insert()
                        // }
                        component_name = Vec::new();
                    }
                    let data_split: Vec<&str> = data.as_str().split("\"").collect();
                    if data_split.len() == 5 {
                        is_result = true;
                        result_name = data_split[1].clone().to_owned();
                        let frame_string: Vec<&str> =
                            _space_seperator.split(data_split[4]).into_iter().collect();
                        frame_name = frame_string[1].to_owned();
                        frame_name_list.insert(frame_name.clone(), frame_name.clone());
                    }
                    if is_result {
                        continue;
                    }
                }

                let lowcase_data = data.clone().to_lowercase();
                if lowcase_data.contains("componentnames") {
                    let component_list: Vec<&str> = data.as_str().split("\"").collect();
                    component_name = Vec::new();
                    //component name 1个或多个
                    for i in 1..component_list.len() {
                        let empty_of_space = component_list[i].replace(" ", ""); //将空格 消除
                        if empty_of_space.is_empty() == false {
                            //如果不是空，则是component name
                            //非空格，或多个空格
                            let name = component_list[i].to_owned();
                            //组合形成a.b形式的名称
                            let name_of_ab =
                                result_name.clone() + &"." + &component_list[i].clone(); //通过符号“.”链接的名称：a.b                                                                                               //插入KV

                            result_componet_list.insert(name_of_ab.clone(), name_of_ab.clone());
                            component_name.push(name_of_ab.clone()); //当前的组件名称vec push
                                                                     //hashmap是有此a.b的名称
                            let is_result_component = source.result.get_mut(&name_of_ab);
                            //没有这个kv，则insert
                            if is_result_component.is_none() {
                                //建立a.b下hashmap
                                let mut a_b = source.result.insert(
                                    //insert  k= a.b ,v=resource_component_frame的hashmap
                                    name_of_ab.clone(), //场名称
                                    HashMap::new(),
                                );
                                //建立 frame的Hashmap
                                source
                                    .result
                                    .get_mut(&name_of_ab)
                                    .unwrap()
                                    .insert(frame_name.clone(), HashMap::new());
                                //这个new是KV
                            }
                        }
                    }
                    is_componet = true;
                    continue;
                }
                if lowcase_data.contains("values") && is_values == false {
                    is_values = true;
                    continue;
                }
                if lowcase_data.contains("end values") && is_values == true {
                    is_values = false;
                    component_name = Vec::new();
                    continue;
                }

                if is_values == true {
                    let line_space: Vec<&str> =
                        _space_seperator.split(data.as_str()).into_iter().collect();

                    if line_space.len() < component_name.len() {
                        print!("component 与数据不对应");
                    } else {
                        for i in 0..component_name.len() {
                            let name_of_ab = component_name[i].clone(); //通过符号“.”链接的名称：a.b

                            //frame 对应的数据
                            // let mut is_frame_data = is_result_component
                            //     .component_name
                            //     .get_mut(frame_name)
                            //     .unwrap();
                            let k = line_space[1].to_string().parse::<u64>().unwrap();
                            let v = line_space[i + 2].to_string().parse::<f32>().unwrap();
                            let mut is_SC = source.result.get_mut(&name_of_ab);
                            let mut is_kv = source
                                .result
                                .get_mut(&name_of_ab)
                                .unwrap()
                                .get_mut(&frame_name)
                                .unwrap()
                                .insert(k, v);
                            let t = source
                                .result
                                .get_mut(&name_of_ab)
                                .unwrap()
                                .get_mut(&frame_name)
                                .unwrap();
                            let ttt = source
                                .result
                                .get_mut(&name_of_ab)
                                .unwrap()
                                .get_mut(&frame_name)
                                .unwrap()
                                .get(&k);
                            // print!("k={}\n", ttt.unwrap());
                        }
                    }
                    // print!("{:?}\n", line_space);
                }
            }
        }
        for per_sitename in result_componet_list.values() {
            source.result_componet.push(per_sitename.to_string());
        }
        for (k, v) in frame_name_list {
            source.frames.push(k);
        }
        source
    }

    pub fn hello() {
        print!("hello GID");
    }
    //第1个String是线段的两个node的排序名称你
    //所有lines，插值后的插值，String（a，b）节点的排序；hashmap的f32是插值结果（可能多个）与对应的xyz
    // out_triangles:HashMap<f32,Vec<f32>>,//输出
    pub type OneContoursOfAB = HashMap<String, HashMap<String, [f32; 3]>>; //AB
    pub type AllContoursOfAB =
        HashMap<String, HashMap<String, HashMap<String, HashMap<String, [f32; 3]>>>>; //site-->frame-->AB-->Value-->[]

    //所有的等值体,第一个String=site名称,第二个String=frame名称,第三个=插值数值
    pub type AllContour = HashMap<String, HashMap<String, HashMap<String, Vec<f32>>>>;

    ///四面体重构数据
    ///network：四面体数据，无顺序，嵌套的Vec是节点列表（4个）
    ///
    pub struct OneTetrahedronNetwork {
        network: Vec<Vec<u64>>, //四面体节点数据index（a,b,c,d）
    }
    pub struct TetrahedronNetwork {
        meshs: HashMap<String, OneTetrahedronNetwork>,
    }
    pub fn generate_contour_of_trigangles(
        values_list: Vec<f32>,
        terah_network: &TetrahedronNetwork,
        mesh: &mesh_source,
        res: &resource_source,
    ) -> AllContour {
        let site_names = &res.result_componet;
        let frames = &res.frames;

        //所有线计划,每个nodeAB只计算一次
        let mut AB: AllContoursOfAB = AllContoursOfAB::new();
        //每个mesh的四面体计划
        for per_mesh in terah_network.meshs.values() {
            //每个sitename循环
            for per_site in site_names {
                if AB.get(per_site).is_none() {
                    AB.insert(per_site.to_string(), HashMap::new());
                }
                //每帧循环
                for per_frame in frames {
                    if AB.get(per_site).unwrap().get(per_frame).is_none() {
                        AB.get_mut(per_site)
                            .unwrap()
                            .insert(per_frame.to_string(), OneContoursOfAB::new());
                    }
                    //每个四面体序号
                    for per_one_terahedron in &per_mesh.network {
                        let lines: [[usize; 2]; 6] =
                            [[0, 1], [1, 2], [0, 2], [0, 3], [1, 3], [2, 3]];
                        //每条线循环
                        for per_line in lines {
                            let mut one_AB = vec![
                                per_one_terahedron[per_line[0]],
                                per_one_terahedron[per_line[1]],
                            ];
                            one_AB.sort();
                            let one_AB_string = one_AB
                                .iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<String>>()
                                .join("-");
                            let point_a_position =
                                mesh.nodes.get(&per_one_terahedron[per_line[0]]).unwrap();
                            let point_a_vec = Vector3::new(
                                point_a_position[0],
                                point_a_position[1],
                                point_a_position[2],
                            );
                            let point_b_position =
                                mesh.nodes.get(&per_one_terahedron[per_line[1]]).unwrap();

                            let point_b_vec = Vector3::new(
                                point_b_position[0],
                                point_b_position[1],
                                point_b_position[2],
                            );
                            let point_a_value = get_res_by_site_frame_id(
                                res,
                                &per_site,
                                &per_frame,
                                per_one_terahedron[per_line[0]],
                            );
                            let point_b_value = get_res_by_site_frame_id(
                                res,
                                &per_site,
                                &per_frame,
                                per_one_terahedron[per_line[1]],
                            );
                            //每条线,求每个插值在AB之间是否有值,若有值,求位置
                            for per_value in &values_list {
                                let (vec1, v) = get_value_point_of_AB(
                                    point_a_vec,
                                    point_b_vec,
                                    point_a_value,
                                    point_b_value,
                                    *per_value,
                                );
                                //在0-1之间
                                if v >= 0.0 && v <= 1.0 {
                                    if AB
                                        .get(per_site)
                                        .unwrap()
                                        .get(per_frame)
                                        .unwrap()
                                        .get(&one_AB_string)
                                        .is_none()
                                    {
                                        let mut name_s_f = AB
                                            .get_mut(per_site)
                                            .unwrap()
                                            .get_mut(per_frame)
                                            .unwrap();
                                        name_s_f.insert(one_AB_string.clone(), HashMap::new());
                                        let mut name_s_f_ab =
                                            name_s_f.get_mut(&one_AB_string).unwrap();
                                        let f32_3 = [vec1.x, vec1.y, vec1.z];
                                        name_s_f_ab.insert(per_value.to_string(), f32_3);
                                    } else if AB
                                        .get(per_site)
                                        .unwrap()
                                        .get(per_frame)
                                        .unwrap()
                                        .get(&one_AB_string)
                                        .unwrap()
                                        .get(&per_value.to_string())
                                        .is_none()
                                    {
                                        let mut name_s_f_ab = AB
                                            .get_mut(per_site)
                                            .unwrap()
                                            .get_mut(per_frame)
                                            .unwrap()
                                            .get_mut(&one_AB_string)
                                            .unwrap();
                                        let f32_3 = [vec1.x, vec1.y, vec1.z];
                                        name_s_f_ab.insert(per_value.to_string(), f32_3);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut list: AllContour = AllContour::new();
        //site-->1
        //        frames-->2
        //               插值数组-->3
        //                      t4-->4
        //                         per t4-->5
        //                                  perline-->6
        //                                    check-->等值面（3||4点）
        //
        for per_site in site_names {
            list.insert(per_site.to_string(), HashMap::new()); //1
            for per_frame in frames {
                //2
                list.get_mut(per_site)
                    .unwrap()
                    .insert(per_frame.to_owned(), HashMap::new());
                let mut s_f = list.get_mut(per_site).unwrap().get_mut(per_frame).unwrap();
                //3,
                for per_value in &values_list {
                    //vec对应的xyz的vec
                    let mut s_f_v;
                    if s_f.get(&per_value.to_string()).is_none() {
                        s_f.insert(per_value.to_string(), Vec::new());
                    }
                    s_f_v = s_f.get_mut(&per_value.to_string()).unwrap();

                    //4
                    for per_mesh in terah_network.meshs.values() {
                        //四面体集合
                        //5
                        let mut one_T4_point: Vec<Vec<f32>> = Vec::new();
                        for per_one_terahedron in &per_mesh.network {
                            let lines: [[usize; 2]; 6] =
                                [[0, 1], [1, 2], [0, 2], [0, 3], [1, 3], [2, 3]];
                            //6
                            let mut count_of_line: Vec<[usize; 2]>;
                            for per_line in lines {
                                let mut one_AB = vec![
                                    per_one_terahedron[per_line[0]],
                                    per_one_terahedron[per_line[1]],
                                ];
                                one_AB.sort();
                                let one_AB_string = one_AB
                                    .iter()
                                    .map(|n| n.to_string())
                                    .collect::<Vec<String>>()
                                    .join("-");

                                let (vec1, is_true) = get_C_by_site_frame_value__AB(
                                    &AB,
                                    per_site,
                                    per_frame,
                                    &one_AB_string,
                                    &per_value.to_string(),
                                );
                                // print!("is true:{is_true} \n");
                                if is_true {
                                    one_T4_point.push(vec1);
                                }
                            }
                        }
                        if one_T4_point.len() == 3 {
                            for i in one_T4_point {
                                for j in i {
                                    s_f_v.push(j);
                                }
                            }
                        } else if one_T4_point.len() == 4 {
                            let p1 = Vector4::new(
                                one_T4_point[0][0],
                                one_T4_point[0][1],
                                one_T4_point[0][2],
                                1.0,
                            );
                            let p2 = Vector4::new(
                                one_T4_point[1][0],
                                one_T4_point[1][1],
                                one_T4_point[1][2],
                                1.0,
                            );
                            let p3 = Vector4::new(
                                one_T4_point[2][0],
                                one_T4_point[2][1],
                                one_T4_point[2][2],
                                1.0,
                            );
                            let p4 = Vector4::new(
                                one_T4_point[3][0],
                                one_T4_point[3][1],
                                one_T4_point[3][2],
                                1.0,
                            );
                            let list = get_rectangle_index(p1, p2, p3, p4);
                            for i in list {
                                for j in i {
                                    s_f_v.push(one_T4_point[j][0]);
                                    s_f_v.push(one_T4_point[j][1]);
                                    s_f_v.push(one_T4_point[j][2]);
                                }
                            }
                        }
                    }
                }
            }
        }

        return list;
    }

    pub fn get_rectangle_index(
        p1: Vector4<f32>,
        p2: Vector4<f32>,
        p3: Vector4<f32>,
        p4: Vector4<f32>,
    ) -> Vec<[usize; 3]> {
        let mut point_a = Vector3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let mut x = point_a.normalize();
        let mut z_orgin = Vector3::new(0.0, 0.0, 1.0);
        let mut z = (z_orgin.cross(point_a.normalize())).normalize();

        let mut y = z.cross(x);
        y = y.normalize();

        // print!("x={:#?}\n", x.normalize());
        // print!("y={:#?}\n", y);
        // print!("z={:#?}\n", z);

        let mut m: Matrix4<f32> = Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        m.x[0] = x[0];
        m.x[1] = x[1];
        m.x[2] = x[2];

        m.y[0] = y[0];
        m.y[1] = y[1];
        m.y[2] = y[2];

        m.z[0] = z[0];
        m.z[1] = z[1];
        m.z[2] = z[2];

        m.w[0] = point_a[0];
        m.w[1] = point_a[1];
        m.w[2] = point_a[2];
        let mv_world = m.invert().unwrap();
        let p1n = mv_world * p1;
        let p2n = mv_world * p2;
        let p3n = mv_world * p3;
        let p4n = mv_world * p4;

        let mut point_list_2d: Vec<[f64; 2]> = Vec::new();
        point_list_2d.push([p1n.x as f64, p1n.y as f64]);
        point_list_2d.push([p2n.x as f64, p2n.y as f64]);
        point_list_2d.push([p3n.x as f64, p3n.y as f64]);
        point_list_2d.push([p4n.x as f64, p4n.y as f64]);

        return get_2d_delanay(point_list_2d);
    }

    pub fn get_2d_delanay(vec_pts: Vec<[f64; 2]>) -> Vec<[usize; 3]> {
        // let mut vec_pts: Vec<[f64; 2]> = Vec::new();
        // vec_pts.push([0.0, 0.0]);
        // vec_pts.push([1.0, 0.0]);
        // vec_pts.push([1.0, 1.0]);
        // vec_pts.push([0.0, 1.0]);
        // let mut vec_inds: Vec<usize> = Vec::new();

        let mut list: Vec<[usize; 3]> = Vec::new();
        let mut del_struct = DelaunayStructure2D::new();
        del_struct.insert_vertices(&vec_pts, true);

        for ind_triangle in 0..del_struct.get_simplicial().get_nb_triangles() {
            // print!("====={ind_triangle}:___\n");
            let tri = del_struct
                .get_simplicial()
                .get_triangle(ind_triangle)
                .unwrap();
            let [h1, h2, h3] = tri.halfedges();
            let ind_pt1: Node = h1.first_node();
            let ind_pt2 = h2.first_node();
            let ind_pt3 = h3.first_node();
            if let (Node::Value(val1), Node::Value(val2), Node::Value(val3)) =
                (ind_pt1, ind_pt2, ind_pt3)
            {
                list.push([val1, val2, val3]);
                // print!("{:#?},", val1);
                // print!("{:#?},", val2);
                // print!("{:#?}\n", val3);
            }
        }
        return list;
    }

    pub fn get_C_by_site_frame_value__AB(
        AB: &AllContoursOfAB,
        site: &str,
        frame: &str,
        ABString: &str,
        value: &str,
    ) -> (Vec<f32>, bool) {
        let mut vec1: Vec<f32> = Vec::new();
        let mut is_true = false;
        let abc = AB.get(site);
        // print!("AB:{:#?}",AB);
        // print!("AB===={ABString}");
        if AB.get(site).is_some() {
            if AB.get(site).unwrap().get(frame).is_some() {
                if AB
                    .get(site)
                    .unwrap()
                    .get(frame)
                    .unwrap()
                    .get(ABString)
                    .is_some()
                {
                    if AB
                        .get(site)
                        .unwrap()
                        .get(frame)
                        .unwrap()
                        .get(ABString)
                        .unwrap()
                        .get(value)
                        .is_some()
                    {
                        let f32_3 = AB
                            .get(site)
                            .unwrap()
                            .get(frame)
                            .unwrap()
                            .get(ABString)
                            .unwrap()
                            .get(value)
                            .unwrap();

                        for i in 0..f32_3.len() {
                            vec1.push(f32_3[i]);
                        }

                        is_true = true;
                    }
                }
            }
        }
        (vec1, is_true)
    }
    ///VA,VB与value的值,线性插值AB两点之间C,
    pub fn get_value_point_of_AB(
        A: Vector3<f32>,
        B: Vector3<f32>,
        VA: f32,
        VB: f32,
        value: f32,
    ) -> (Vector3<f32>, f32) {
        let vector1 = B - A;
        let mut v = -1.0;
        if VB == VA && VA == 0.0 {
        } else {
            v = (value - VA) / (VB - VA);
        }
        // println!("VA:{VA},VB{VB},V:{v}\n");
        (A + vector1 * v, v)
    }

    ///获取node对应的RES中的值
    pub fn get_res_by_site_frame_id(
        res: &resource_source,
        site: &str,
        frame: &str,
        id: u64,
    ) -> f32 {
        let value = res
            .result
            .get(site)
            .unwrap()
            .get(frame)
            .unwrap()
            .get(&id)
            .unwrap();
        return value.clone();
    }
    ///分成三个部分
    ///
    ///1、输出四面体：六面体-->四面体，四面体不需要
    ///
    ///2、for循环，每个四面体的6条线，进行插值
    ///
    ///3、输出等值面三角形计划，
    ///
    ///3.1、三个或四个点的属性排列，abc，abcd
    ///
    ///3.2、输出三角形，1 or 2 个
    pub fn generate_network_tetrahedron_for_gid(
        mesh: &mesh_source,
        res: &resource_source,
    ) -> TetrahedronNetwork {
        let mut outT: TetrahedronNetwork = TetrahedronNetwork {
            meshs: HashMap::new(),
        };
        //Tetrahedra Nnode 4
        //Hexahedra Nnode 8
        for per_mesh in mesh.meshs.values() {
            let mut flag_done = false; //是否有更改的标志位
                                       //新建的
            let mut per_one = OneTetrahedronNetwork {
                network: Vec::new(),
            };
            if per_mesh.element_type == "Hexahedra Nnode 8" {
                for per_Hexahedra in per_mesh.elements.values() {
                    let mut per_4: Vec<u64> = Vec::new();

                    //1
                    per_4.push(per_Hexahedra[1].clone());
                    per_4.push(per_Hexahedra[5].clone());
                    per_4.push(per_Hexahedra[6].clone());
                    per_4.push(per_Hexahedra[7].clone());

                    //2
                    per_4.push(per_Hexahedra[5].clone());
                    per_4.push(per_Hexahedra[7].clone());
                    per_4.push(per_Hexahedra[1].clone());
                    per_4.push(per_Hexahedra[4].clone());

                    //3
                    per_4.push(per_Hexahedra[5].clone());
                    per_4.push(per_Hexahedra[1].clone());
                    per_4.push(per_Hexahedra[6].clone());
                    per_4.push(per_Hexahedra[2].clone());

                    //4
                    per_4.push(per_Hexahedra[6].clone());
                    per_4.push(per_Hexahedra[0].clone());
                    per_4.push(per_Hexahedra[2].clone());
                    per_4.push(per_Hexahedra[1].clone());

                    //5
                    per_4.push(per_Hexahedra[1].clone());
                    per_4.push(per_Hexahedra[6].clone());
                    per_4.push(per_Hexahedra[0].clone());
                    per_4.push(per_Hexahedra[7].clone());

                    //6
                    per_4.push(per_Hexahedra[2].clone());
                    per_4.push(per_Hexahedra[6].clone());
                    per_4.push(per_Hexahedra[3].clone());
                    per_4.push(per_Hexahedra[0].clone());

                    per_one.network.push(per_4);
                }
                flag_done = true;
            } else if per_mesh.element_type == "Tetrahedra Nnode 4" {
                for per_Tetrahedron in per_mesh.elements.values() {
                    let mut per_4: Vec<u64> = Vec::new();
                    for i in 0..per_Tetrahedron.len() {
                        per_4.push(per_Tetrahedron[i] as u64);
                    }
                    per_one.network.push(per_4);
                }
                flag_done = true;
            }
            if flag_done {
                outT.meshs.insert(per_mesh.name.clone(), per_one);
            }
        }
        return outT;
    }

    //GID 六面体到6个四面体的节点顺序
    // 0:  [1, 5, 6, 7]
    // 1:  [5, 7, 1, 4]
    // 2:  [5, 1, 6, 2]
    // 3:  [6, 0, 2, 1]
    // 4:  [1, 6, 0, 7]
    // 5:  [2, 6, 3, 0]
    // struct Vertex {
    //     position: [f32; 3],
    //     // color: [f32; 4],
    // }
    pub fn get_hexahedra8_of_triangle(mesh: &mesh_source, res: &resource_source) -> Vec<f32> {
        let mut triangles: Vec<f32> = Vec::new();
        let nodes: &HashMap<u64, [f32; 3]> = &mesh.nodes;
        let per_hexahedra8_for_triangle: [[usize; 4]; 6] = [
            [4, 0, 1, 5], //front
            [7, 3, 2, 6], //back
            [5, 1, 2, 6], //top
            [4, 0, 3, 7], //bottom
            [7, 4, 5, 6], //left
            [3, 0, 1, 2], //right
        ];
        for per_mesh in mesh.meshs.values() {
            if per_mesh.element_type == "Hexahedra Nnode 8" {
                for per_Hexahedra in per_mesh.elements.values() {
                    for face in per_hexahedra8_for_triangle {
                        let one_positions = get_onetriangle(
                            nodes,
                            [
                                per_Hexahedra[face[0]],
                                per_Hexahedra[face[1]],
                                per_Hexahedra[face[3]],
                            ],
                        );
                        for i in 0..one_positions.len() {
                            triangles.push(one_positions[i].clone());
                        }
                        let two_positions = get_onetriangle(
                            nodes,
                            [
                                per_Hexahedra[face[1]],
                                per_Hexahedra[face[2]],
                                per_Hexahedra[face[3]],
                            ],
                        );
                        for i in 0..two_positions.len() {
                            triangles.push(two_positions[i].clone());
                        }
                    }
                }
            }
        }
        return triangles;
    }
    pub fn get_tetrahedra4_of_triangle(mesh: &mesh_source, res: &resource_source) -> Vec<f32> {
        let mut triangles: Vec<f32> = Vec::new();
        let nodes: &HashMap<u64, [f32; 3]> = &mesh.nodes;
        let per_hexahedra8_for_triangle: [[usize; 3]; 4] =
            [[0, 1, 2], [0, 1, 3], [1, 2, 3], [2, 0, 3]];
        for per_mesh in mesh.meshs.values() {
            if per_mesh.element_type == "Tetrahedra Nnode 4" {
                for per_Hexahedra in per_mesh.elements.values() {
                    for face in per_hexahedra8_for_triangle {
                        let one_positions = get_onetriangle(
                            nodes,
                            [
                                per_Hexahedra[face[0]],
                                per_Hexahedra[face[1]],
                                per_Hexahedra[face[2]],
                            ],
                        );
                        for i in 0..one_positions.len() {
                            triangles.push(one_positions[i].clone());
                        }
                    }
                }
            }
        }
        return triangles;
    }

    /// 获取一个三角形的三个点的XYZ
    /// 示例：
    /// ```rust
    /// get_onetriangle(nodes,three_point_id )
    /// ```
    /// # nodes是结构mesh_source中的nodes的引用
    /// ## 是Hashmap
    pub fn get_onetriangle(
        nodes: &HashMap<u64, [f32; 3]>,
        onetriangle_nodes: [u64; 3],
    ) -> Vec<f32> {
        let mut positions: Vec<f32> = Vec::new();
        for k in onetriangle_nodes {
            let xyz = nodes.get(&k).unwrap();
            for i in xyz {
                positions.push(*i);
            }
        }
        return positions;
    }
}
