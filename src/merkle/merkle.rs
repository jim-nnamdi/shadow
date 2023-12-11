use crate::randomwalk::secp256k1;
use serde::{Deserialize, Serialize};
use std::{convert::AsRef, convert::From, fmt::Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub parent: Option<Box<Node>>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub root: Option<Box<Node>>,
    pub leaves: Vec<Option<Box<Node>>>,
    pub hash: String,
}

pub fn secret_print<T: AsRef<str> + Display>(input: T) {
    println!("{}", input);
}

impl From<Node> for Vec<Node> {
    fn from(value: Node) -> Self {
        let node_vector = vec![value];
        node_vector
    }
}

impl From<Vec<String>> for Tree {
    fn from(value: Vec<String>) -> Self {
        let root_node = &value[0];
        let last_hash = value.len() - 1;
        dbg!("root_node", root_node);
        Tree {
            root: Some(Box::new(Node::default())),
            leaves: vec![Some(Box::new(Node {
                parent: None,
                left: None,
                right: None,
                hash: last_hash.to_string(),
            }))],
            hash: last_hash.to_string(),
        }
    }
}

impl From<Vec<Node>> for Tree {
    fn from(value: Vec<Node>) -> Self {
        if value.len() > 1 {
            for x in value.into_iter() {
                let hash_node = secp256k1::_hash_merkle_nodex(Box::new(x.clone())).unwrap();
                let tree = Tree {
                    root: Some(Box::new(x.clone())),
                    leaves: vec![Some(Box::new(x))],
                    hash: hash_node,
                };
                return tree;
            }
        }
        Tree::default()
    }
}

impl Default for Tree {
    fn default() -> Self {
        Tree {
            root: None,
            leaves: vec![None],
            hash: String::from(""),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            parent: None,
            left: None,
            right: None,
            hash: String::from(""),
        }
    }
}

impl Tree {
    pub fn _new(&self) -> Self {
        let secret = secp256k1::_secp256k1().unwrap();
        let root_node = Some(Box::new(Node {
            parent: None,
            left: None,
            right: None,
            hash: secret.0.display_secret().to_string(),
        }));
        let single_node = Some(Box::new(Node {
            parent: None,
            left: None,
            right: None,
            hash: secret.0.display_secret().to_string(),
        }));
        let vector_nodes = vec![root_node.clone(), single_node];
        let tree = Tree {
            root: root_node, // change this to the build_root fn
            leaves: vector_nodes.clone(),
            hash: secret.0.display_secret().to_string(),
        };
        tree
    }

    pub fn _build_root(&self) -> Node {
        let nodes = self.leaves.clone();
        // iterating until we reach a single root node
        if nodes.len() > 1 {
            let node_default = Node::default();
            let parents = Vec::from(node_default.clone());
            let mut hashed_nodes = vec![];
            // having odd number of nodes duplicate last node
            if nodes.len() % 2 != 0 {
                nodes.clone().push(Some(Box::new(node_default)));
                // now we have equal leaves so we hash them
                for x in nodes.into_iter() {
                    let xval = x.unwrap();
                    let hash_node = secp256k1::_hash_merkle_nodex(xval).unwrap();
                    hashed_nodes.push(hash_node);
                }
            }
            let trdata = Tree::from(hashed_nodes);
            let sctdata = Tree::from(parents);
            dbg!(trdata, sctdata); 
        }
        Node::default()
    }
}
