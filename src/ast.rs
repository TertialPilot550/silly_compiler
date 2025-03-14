
// Node Definitions (parent, body)

pub fn test() {
    let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

    let num1 = Box::new(Node { parent: None, body: NodeBody::Number(5) });

    print_tree(ast);
}

#[derive(Debug)]
struct Node {
    parent: Option<Box<Node>>,
    body: NodeBody
}

#[derive(Debug)]
enum NodeBody {
   
    Variable(String, Box<Node>), // (name, value)
    Number(usize), // (value)

    Expression(Option<Box<Node>>), // ()
    BinaryOperator(char, Option<Box<Node>>, Option<Box<Node>>),
    UnaryOperator(char, Option<Box<Node>>),

    Conditional(char, Option<Box<Node>>, Option<Box<Node>>),
    While(), For(), 

}

/*
for loop
while loop
if / if else / if else if 
break, continue

*/

// Tree Definitions
#[derive(Debug)]
pub struct AbstractSyntaxTree {
    head: Box<Node>
}
impl AbstractSyntaxTree {

    fn new() -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            head: Box::new( Node { parent: None, body: NodeBody::Expression(None) })
        }
    }

}


fn print_tree(tree: AbstractSyntaxTree) {
    println!("{:?}", tree);
}

