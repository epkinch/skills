#[derive(Debug)]
enum Tree<T: Ord> {
    Node {
        data: T,
        left_child: Box<Tree<T>>,
        right_child: Box<Tree<T>>,
        },
    Empty,
}

impl<T: Ord> Tree<T> {
    pub fn insert_node(&mut self, data: T) {
        match self {
            Tree::Node { data: node_data, left_child, right_child } => {
                if *node_data == data {
                    return
                }

                let new_node = if data w< *node_data { 
                    left_child 
                } else { 
                    right_child 
                };
                
                new_node.insert_node(data);
            }
            Tree::Empty => {
                *self = Tree::Node {
                    data,
                    left_child: Box::new(Tree::Empty),
                    right_child: Box::new(Tree::Empty),
                };
            }
        }
    }
}

fn main() {
    let root = String::from("1");

    let mut tree = Tree::Node {
        data: root,
        left_child: Box::new(Tree::Empty),
        right_child: Box::new(Tree::Empty),
    };

    let new_value = String::from("2");
    tree.insert_node(new_value);

    println!("{:#?}",&tree);
}



// Empty allows for the representation of there being no node.