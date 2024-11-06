pub use gid::getMsh;
pub use gid::getRES;
pub use gid::hello;
pub use gid::mesh_source;
mod gid {
    // use std::fs;
    // use std::fs::File;
    // use std::fs::OpenOptions;
    // use std::io::Write;
    // use std::io::{Read, Write};
    use std::{
        collections::HashMap,
        convert::TryInto,
        fs::{self, File, OpenOptions},
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
        // nodes: HashMap<u64, [f32; 3]>,
        element_type: String,
        elements: HashMap<u128, Vec<u64>>,
    }
    pub struct position {}
    pub struct mesh_source {
        nodes: HashMap<u128, [f32; 3]>,
        meshs: HashMap<String, mesh_struct>,
    }

    //每帧的a.b的数值
    pub struct resource_kv {
        values: HashMap<u128, f32>,
    }

    //a.b 的帧数（号）对应的节点与数值
    pub struct resource_frame_data {
        // frames: Vec<String>, //帧列表
        frame_hashmap: HashMap<String, resource_kv>, //每帧对应的节点与值的hashmap
    }
    pub struct resource_source {
        // result_componet: Vec<String>, //abc.u ,...
        result_componet: HashMap<String, String>, //abc.u ,...
        result: HashMap<String, resource_frame_data>,
        frames: Vec<String>, //帧列表,步长与名称是统一的
    }

    pub fn get_data_from_file(filename: &str) -> std::io::Lines<BufReader<File>> {
        let file = File::open(filename).unwrap();
        let lines = BufReader::new(file).lines(); //遍历所有行
        lines
    }
    pub fn getMsh(filename: &str) -> mesh_source {
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
                        let k = line_space[1].to_string().parse::<u128>().unwrap();

                        let v = [
                            line_space[2].to_string().parse::<f32>().unwrap(),
                            line_space[3].to_string().parse::<f32>().unwrap(),
                            line_space[4].to_string().parse::<f32>().unwrap(),
                        ];
                        source.nodes.insert(k, v);
                        print!("{k},{:?}\n", v);
                    } else if line_space.len() == 5 {
                        let k = line_space[1].to_string().parse::<u128>().unwrap();

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
                        let k = line_space[1].to_string().parse::<u128>().unwrap();
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
    pub fn getRES(filename: &str) -> resource_source {
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
                                let mut a_resource_kv: resource_kv = resource_kv {
                                    values: HashMap::new(),
                                };
                                a_resource_kv.values.insert(1, 6.6);
                                let mut b_resource_frame_data: resource_frame_data =
                                    resource_frame_data {
                                        frame_hashmap: HashMap::new(),
                                    };
                                b_resource_frame_data
                                    .frame_hashmap
                                    .insert(frame_name.clone(), a_resource_kv);
                                source.result.insert(name_of_ab, b_resource_frame_data);

                                // let mut a_b = source.result.insert(
                                //     //insert  k= a.b ,v=resource_component_frame的hashmap
                                //     name_of_ab.clone(), //场名称
                                //     resource_frame_data {
                                //         frame_hashmap: HashMap::new(),
                                //     }, //k=frame的名称，v=nodeid与仿真值的HashMap
                                // );
                                // let mut a_b_c = source
                                //     .result
                                //     .get_mut(&name_of_ab)
                                //     .unwrap()
                                //     .frame_hashmap
                                //     .insert(
                                //         frame_name.clone(),
                                //         resource_kv {
                                //             values: HashMap::new(),
                                //         },
                                //     );
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
                            let k = line_space[1].to_string().parse::<u128>().unwrap();
                            let v = line_space[i + 2].to_string().parse::<f32>().unwrap();
                            let mut is_SC = source.result.get_mut(&name_of_ab);
                            let mut is_kv = source
                                .result
                                .get_mut(&name_of_ab)
                                .unwrap()
                                .frame_hashmap
                                .get_mut(&frame_name)
                                .unwrap();
                            let ttt = is_kv.values.insert(k, v);
                            let t1 = is_kv.values.get(&k);
                            print!("{}", t1.unwrap());
                            // print!("{name_of_ab}:{k},{v}");
                            // if ttt.is_some() {
                            //     print!("  ===={}", ttt.unwrap());
                            // } else {
                            //     print!("  xxxxxx");
                            // }
                            // print!("\n");
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
}
