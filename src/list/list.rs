use node;

pub List<Node<T>> {
    len: usize,
    head: Node<T>,
    tail: Node<T>,
    next: Option<Box<node>>,
}

pub fn<Node<T>> add_element(element: Node<T>){
    tail = element;
}