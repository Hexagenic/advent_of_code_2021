use crate::solutions::Solution;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Graph<'a>(HashMap<Node<'a>, Vec<Node<'a>>>);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Node<'a> {
    Start,
    Small(&'a str),
    Big(&'a str),
    End,
}

fn parse_node(node: &str) -> Node {
    if node == "start" {
        Node::Start
    } else if node == "end" {
        Node::End
    } else if node.chars().next().unwrap().is_ascii_uppercase() {
        Node::Big(node)
    } else {
        Node::Small(node)
    }
}

fn parse_edge(edge: &str) -> (Node, Node) {
    let parts = edge.split('-').collect::<Vec<_>>();

    (parse_node(parts[0]), parse_node(parts[1]))
}

fn parse_edges<'a>(edges: &'a str) -> Graph<'a> {
    let edges: Vec<(Node, Node)> = edges.lines().map(parse_edge).collect();
    let mut nodes: HashMap<Node<'a>, Vec<Node<'a>>> = HashMap::new();

    let mut add_path = |from: &Node<'a>, to: &Node<'a>| {
        if nodes.contains_key(from) {
            nodes.get_mut(from).unwrap().push(*to);
        } else {
            nodes.insert(*from, vec![*to]);
        }
    };

    for (n1, n2) in edges {
        add_path(&n1, &n2);
        add_path(&n2, &n1);
    }

    Graph(nodes)
}

fn recursive_travel<'a>(graph: &Graph<'a>, current: &Node<'a>, path: &mut Vec<Node<'a>>) -> i64 {
    let mut count = 0;
    path.push(*current);
    for n in &graph.0[current] {
        count += match n {
            Node::Big(_) => recursive_travel(graph, n, path),
            Node::Small(_) if !path.contains(n) => recursive_travel(graph, n, path),
            Node::End => 1,
            _ => 0,
        };
    }
    path.pop();

    count
}

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(recursive_travel(
        &parse_edges(file),
        &Node::Start,
        &mut Vec::with_capacity(100),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
    const MEDIUM_EXAMPLE: &str =
        "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
    const LARGE_EXAMPLE: &str = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";

    #[test]
    fn parse_nodes_test() {
        assert_eq!(Node::Start, parse_node("start"));
        assert_eq!(Node::End, parse_node("end"));
        assert_eq!(Node::Big("A"), parse_node("A"));
        assert_eq!(Node::Small("a"), parse_node("a"));
        assert_eq!(Node::Big("BB"), parse_node("BB"));
    }

    #[test]
    fn parse_edges_test() {
        assert_eq!(6, parse_edges(&SMALL_EXAMPLE).0.len());
        assert_eq!(7, parse_edges(&MEDIUM_EXAMPLE).0.len());
        assert_eq!(10, parse_edges(&LARGE_EXAMPLE).0.len());
    }

    #[test]
    fn part_a_examples() {
        assert_eq!(Solution::Integer(10), part_a(&SMALL_EXAMPLE));
        assert_eq!(Solution::Integer(19), part_a(&MEDIUM_EXAMPLE));
        assert_eq!(Solution::Integer(226), part_a(&LARGE_EXAMPLE));
    }
}
