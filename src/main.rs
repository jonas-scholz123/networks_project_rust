
const NETWORK_MODE: &str = "growing_prop_k";
const NETWORK_SIZE: usize = 1e4 as usize;
const RESULTS_PATH: &str = "./results/";
const M: usize = 3;

fn main() {

    let network = network::build_network(NETWORK_SIZE, NETWORK_MODE, M);
    let results_fname = [
        RESULTS_PATH,
        NETWORK_MODE,
        "_",
        & NETWORK_SIZE.to_string(),
        "_m",
        & M.to_string(),
        ".csv"
        ].concat();

    io::save_network(& network, &results_fname);

    let distribution = analysis::make_neighbour_distribution(&network);
    let distribution_fname = [
        RESULTS_PATH,
        "distribution_",
        NETWORK_MODE,
        "_",
        & NETWORK_SIZE.to_string(),
        "_m",
        & M.to_string(),
        ".csv"
        ].concat();

    io::save_distribution(&distribution, &distribution_fname);
}

pub mod network{
    use rand::Rng;

    pub fn build_network(network_size: usize, network_mode: &str, m: usize) -> Vec<Vec<usize>> {
        println!("Network size: {}", network_size);

        let mut network: Vec<Vec<usize>> = vec![vec![]; network_size];
        let mut all_connections: Vec<usize> = vec![0; 2 * m * (network_size - 1)];

        let mut connection_idx = 0;

        // begin at 1 to avoid initial node
        for new_node in 1..network_size {

            if network_mode == "growing" {
                connect_nodes(& mut network, new_node, get_random_node(new_node, new_node));

            } else if network_mode == "growing_prop_k" {

                for _ in 0..m {
                    let random_node = get_random_item(&mut all_connections[..=connection_idx], new_node);
                    connect_nodes(& mut network, new_node, random_node);
                    // append newly connected nodes to all_connections
                    all_connections[connection_idx] = new_node;
                    connection_idx = connection_idx + 1;

                    all_connections[connection_idx] = random_node;
                    connection_idx = connection_idx + 1;
                }
            }
        }
        network
    }

    fn get_random_node(max_index: usize, except_index: usize) -> usize {

        let mut rand_node = except_index;
        while rand_node == except_index {
            rand_node = rand::thread_rng().gen_range(0, max_index)
        }
        rand_node
    }

    fn get_random_item(arr: & mut [usize], forbidden_value: usize) -> usize {
        let mut rand_node = forbidden_value;

        while rand_node == forbidden_value{
            let rand_idx = rand::thread_rng().gen_range(0, arr.len());
            rand_node = arr[rand_idx];
        }
        rand_node
    }

    fn connect_nodes(network: & mut Vec<Vec<usize>>, n1: usize, n2: usize) {
        network[n1].push(n2);
        network[n2].push(n1);
    }
}

pub mod io {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    fn write_to_file(str: &str, fpath: &str) {
        let path = Path::new(fpath);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(str.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
            Ok(_) => println!("Successfully wrote to {}", display),
        }
    }

    pub fn save_network(network: & Vec<Vec<usize>>, fpath: &str){
        println!("Saving network...");
        let mut network_strings = vec![String::new(); network.len()];

        for (i, node_connections) in network.iter().enumerate() {

            let node_string: Vec<_> = node_connections
                .into_iter()
                .map(|n| n.to_string())
                .collect();

            network_strings[i] = node_string.join(", ");
        }

        let network_string = network_strings.join("\n");
        write_to_file(&network_string, fpath);
    }

    pub fn save_distribution(distribution: &Vec<usize>, fpath: &str) {

        println!("Saving distribution...");

        let str = &distribution
            .iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write_to_file(str, fpath);
    }
}

pub mod analysis {
    pub fn make_neighbour_distribution(network: &Vec<Vec<usize>>) -> Vec<usize> {
    network
    .iter()
    .map(|node_neighbours| node_neighbours.len())
    .collect()
    }
}
