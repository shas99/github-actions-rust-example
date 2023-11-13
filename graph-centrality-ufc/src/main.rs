use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]
struct Fighter{
    name:string;
}

impl Fighter{
    fn new(name: &str) -> Self {
        Self{
            name: name.to_string();
        }
    }
}

impl fmt::Display for Fighter{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter,f32>, nodes: &[NodeIndex], a: usize, b: usize){
    graph.add_edge(nodes[a], nodes[b], 1.0)
}



fn main() {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Figheter::new("Dustin Poirier"),
        Figheter::new("Khabib Nurmagomedov"),
        Figheter::new("Jose Aldo"),
        Figheter::new("Conor McGregor"),
        Figheter::new("Nate Diaz"),
    ];

    let Figheter_nodes: Vec<NodeIndex> = fighters.iter().map(|fighter | graph.addnode(fighter)).collect();

    add_edge(&mut graph, &figheter_nodes, 0 , 1);
    add_edge(&mut graph, &figheter_nodes, 1 , 3);
    add_edge(&mut graph, &figheter_nodes, 3 , 0);
    add_edge(&mut graph, &figheter_nodes, 3 , 2);
    add_edge(&mut graph, &figheter_nodes, 3 , 4);
    add_edge(&mut graph, &figheter_nodes, 0 , 4);
    add_edge(&mut graph, &figheter_nodes, 2 , 4);

    for(i, &node) in fighter_nodes.iter().enumerate(){
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        match name.as_str(){
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the other fighters"
            )
        }
    }
}
