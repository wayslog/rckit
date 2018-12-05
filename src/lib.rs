extern crate clap;
extern crate redis;

mod add;
mod cluster;
mod conn;
mod create;

use add::Add;
use clap::{App, Arg, SubCommand};
use create::Create;
use std::{thread, time};
pub fn run() {
    let matches = App::new("rckit")
        .about("redis cluster tool")
        .subcommand(
            SubCommand::with_name("create")
                .about("create rredis cluster")
                .arg(
                    Arg::with_name("node")
                        .help("cluster nodes")
                        .short("n")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("replicate")
                        .short("s")
                        .default_value("1")
                        .help("slave replicate number "),
                )
                .arg(
                    Arg::with_name("master")
                        .short("m")
                        .default_value("0")
                        .takes_value(true)
                        .help("mster number"),
                ),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add  node to existing cluster")
                .arg(
                    Arg::with_name("cluster")
                        .required(true)
                        .short("c")
                        .help("-c clusterip:port, spec cluster ip and  port")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("node")
                        .required(true)
                        .short("n")
                        .help(
                            "add new node  to cluster,
                        -n <master,slave> <master>",
                        )
                        .takes_value(true),
                ),
        )
        .get_matches();
    if let Some(sub_m) = matches.subcommand_matches("create") {
        let slave_count = clap::value_t!(sub_m.value_of("replicate"), usize).unwrap();
        let mut master_count = clap::value_t!(sub_m.value_of("master"), usize).unwrap();
        let node: Vec<&str> = sub_m.values_of("node").unwrap().collect();
        println!(
            "create cluster with replicate {} node{:?}",
            slave_count, &node
        );
        let mut create = Create::new(node, master_count, slave_count).unwrap();
        create.cluster.check().expect("check node err");
        create.init_slots();
        create.add_slots();
        create.set_config_epoch();
        create.join_cluster();
        while !create.consistent() {
            eprintln!("wait consistent fail");
            thread::sleep(time::Duration::from_secs(1));
        }
        create.set_slave().expect("set slave err");
    }
    if let Some(sub_m) = matches.subcommand_matches("add") {
        let cluster = sub_m
            .value_of("cluster")
            .expect("must spec existing cluster node");
        let nodes: Vec<&str> = sub_m
            .values_of("node")
            .expect("must spec at least one node be add to cluster")
            .collect();
        println!("add node {:?} to cluster {}", nodes, cluster);
        let mut add = Add::new(
            cluster.to_string(),
            nodes.iter().map(|x| x.to_string()).collect(),
        )
        .unwrap();
        add.cluster.check().expect("check cluste nodes fail");
        let _: () = add.add_node().expect("add node fail");
        while !add.cluster.consistency() {
            eprintln!("wait consistent fail");
            thread::sleep(time::Duration::from_secs(1));
        }
        add.set_slave();
    }
}
