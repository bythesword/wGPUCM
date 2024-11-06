//! GID解析
pub use gid::{generate_network_tetrahedron_for_gid, get_hexahedra8_of_triangle, get_msh, get_res};
pub use gid::{mesh_source, resource_source};
mod gid {

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
        result_componet: HashMap<String, String>, //abc.u ,...
        result: HashMap<String, HashMap<String, HashMap<u64, f32>>>,
        frames: Vec<String>, //帧列表,步长与名称是统一的
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

        let mut source: resource_source = resource_source {
            result_componet: HashMap::new(),
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
                            source
                                .result_componet
                                .insert(name_of_ab.clone(), name_of_ab.clone());
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
        source
    }

    pub fn hello() {
        print!("hello GID");
    }

    ///四面体重构数据
    ///network：四面体数据，无顺序，嵌套的Vec是节点列表（4个）
    ///
    pub struct OneTetrahedronNetwork {
        network: Vec<Vec<u64>>, //四面体节点数据index（a,b,c,d）
        lines: HashMap<String, HashMap<f32, [f32; 3]>>, //所有lines，插值后的插值，String（a，b）节点的排序；hashmap的f32是插值结果（可能多个）与对应的xyz
        out_triangles:HashMap<f32,Vec<f32>>,//输出
    }
    pub struct TetrahedronNetwork {
        meshs: HashMap<String, OneTetrahedronNetwork>,
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
        mesh: mesh_source,
        res: resource_source,
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
                lines: HashMap::new(),
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
    pub fn get_hexahedra8_of_triangle(mesh: mesh_source, res: resource_source) -> Vec<f32> {
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
