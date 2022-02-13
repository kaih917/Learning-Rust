Struct Node<T>{
    element: T,
    previous: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
}

