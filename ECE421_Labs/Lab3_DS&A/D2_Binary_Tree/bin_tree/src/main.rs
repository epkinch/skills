// #[derive(Debug)]
// struct TreeNode {
//     data: &str,
//     left_child: Option<TreeNode>,
//     right_child: Option<TreeNode>,
// }

// no, by itself this code will not run. It is missing
// a main function and recursive structs require smart pointers in Rust.
// a borrowed value is also in the struct... need a lifetime

#[derive(Debug)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>,
}

impl<'a> TreeNode<'a> {
    pub fn insert_node(&mut self, data: &'a str) {
        if self.data == data {
            return
        }
    
        let mut new_node = if data < self.data { 
            &mut self.left_child 
        } else { 
            &mut self.right_child 
        };
    
        match new_node {
            None => {
                let tmp = TreeNode {
                    data,
                    left_child: None,
                    right_child: None
                };
                *new_node = Some(Box::new(tmp));
            }
            Some(node) => {
                node.insert_node(data);
            }
        }
    }
}

fn main() {
    let root = String::from("1");

    let mut tree = TreeNode {
        data: &root,
        left_child: None,
        right_child: None
    };

    let new_value = String::from("2");
    tree.insert_node(&new_value);

    println!("{:#?}",&tree);
}


