
// Run tests:
// cargo test datastructures::stack -- --nocapture


/*
Stack implementation with methods:

new()
print()
push(value: i32)
peek() -> i32
pop() -> i32
size() -> u32
is_empty() -> bool
empty()
*/

// yay type aliases!
type Link = Option<Box<Node>>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
struct Node {
    elem: i32,
//        next: Option<Box<Node>>,
    next: Link,
}

impl Node {
    pub fn new(value: i32) -> Box<Node> {
        Box::new(Node {
            elem: value,
            next: None,
        })
    }
}


#[derive(Debug)]
struct Stack {
//        head: Option<Box<Node>>,
    head: Link,
    size: u32,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { head: None, size: 0 }
    }

    /// Prints the entire stack from top to bottom.
    pub fn print(&mut self) {
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let mut current_link = self.head.clone();
        let mut count: u32 = 0;
        println!("Printing the stack (top to bottom) of size ({}):", self.size);

        // Approach 1
//            while !current_link.is_none() {
//                println!("Item ({}) has value: {:?}", count, current_link.clone().unwrap().elem);
//                count += 1;
//                current_link = &current_link.clone().unwrap().next;
//            }

        // Approach 2
        while let Some(this_node) = current_link {
            println!("Item ({}) has value {:?}", count, this_node.elem);
            current_link = this_node.clone().next.take();
            count += 1;
        }
    }

    /// https://rust-unofficial.github.io/too-many-lists/second-option.html
    /// Adds an element (here: i32) at the top of the stack.
    pub fn push(&mut self, elem: i32) {
        let new_node: Node = Node {
            elem: elem,
            // Set node.next to self.head
            next: self.head.take(),
        };
        // Wrap Node inside a Box
        let new_box_node: Box<Node> = Box::new(new_node);

        // Set self.head to new_box_node
        self.head = Some(new_box_node);

        // Or in one go, not sure if the compiler optimizes it
//            self.head = Some(Box::new(Node {elem: elem, next: self.head.take()}))
        self.size += 1;
    }

    /// Returns the option of the first element in the stack. Option will be None if stack is empty.
    pub fn peek(&mut self) -> Option<i32> {
        if let Some(top) = self.head.clone().take() {
            return Some(top.elem);
        } else {
            return None;
        }
    }

    /// Removes and returns the option of the first element in the stack. Option will be None if stack is empty.
    pub fn pop(&mut self) -> Option<i32> {
        // Approach 1 with map and closure
//            self.head.take().map(|node| {
//                self.head = node.next;
//                node.elem
//            })

        // Approach 2 with if
        if let Some(top) = self.head.take() {
            self.size -= 1;
            self.head = top.next;
            return Some(top.elem);
        } else {
            // Stack is empty
            return None;
        }
    }

    /// Returns how many elements there are in the stack
    fn size(&self) -> u32 {
        return self.size;
    }

    /// Returns a boolean if the stack is empty
    fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    // Clear the whole stack
    fn empty(&mut self) {
        self.size = 0;

        // Uncomment me for approach 1-3
        let mut cur_link = self.head.take();

        // Approach 1
//            while let Some(mut boxed_node) = cur_link {
//                cur_link = boxed_node.next.take();
//            }

        // Approach 2
//            loop {
//                match cur_link.take() {
//                    // Create new variable boxed_node and wrap in option, if not None, do the following
//                    Some(mut boxed_node) => {
//                        cur_link = boxed_node.next.take();
//                    },
//                    None => break,
//                    // The following can be replaced with the line above, in this case at least
////                    _ => break,
//                }
//            }

        // Approach 3
        loop {
            // Assign current link to the mutable option variable "boxed_node" ?
            if let Some(mut boxed_node) = cur_link {
//                    cur_link: Option<Box<Node>> = boxed_node.next.take();
                cur_link = boxed_node.next.take();
            } else {
                break;
            }
        }

        // Approach 4
//            self.head = None;
    }
}

// Test functions
pub fn simple_option_take_example() {
    let mut a = Node::new(5);
    let mut b = Node::new(8);
    a.next = Some(b.clone());
    // c should be same as b now
    let c = a.clone().next.take();
    // d should be None
    let d = b.next.take();

    // Check kif node c (which was a) has value 8 and c.next is equal to None

    // Approach 1 with match
    match c.clone().take() {
        Some(e) => {
            assert_eq!(8, e.elem);
            assert_eq!(None, e.next);
        }
        None => {
            // This should not be executed
            assert!(false);
        }
    }

    // Approach 2 with if
//        if let Some(mut e) = c.take() {
    if let Some(e) = c.clone().take() {
        assert_eq!(8, e.elem);
        assert_eq!(None, e.next);
    } else {
        // This should not be executed
        assert!(false);
    }

    // Check a.elem == 5, a.next == b
    assert_eq!(5, a.elem);
    assert_eq!(Some(b.clone()), a.next);

    // Check b.next == None, b.next.is_none() == true
    assert_eq!(None, b.next);
    assert_eq!(true, b.next.is_none());
    // Check d.is_none() == true
    assert_eq!(true, d.is_none());
}

fn stack_create() -> Stack {
    let new_stack = Stack::new();
    return new_stack;
}

fn stack_add_items() -> Stack {
    let mut new_stack = Stack::new();
    for i in 0..=9 {
        new_stack.push(i);
    }
    return new_stack;
}

fn stack_peek_items() -> Stack {
    let mut new_stack = Stack::new();
    for i in 0..=9 {
        new_stack.push(i);
    }
    for _i in 0..=9 {
        new_stack.peek();
    }
    return new_stack;
}

fn stack_empty() -> Stack{
    let mut new_stack = Stack::new();
    for i in 0..=9 {
        new_stack.push(i);
    }
    new_stack.head = None;
    return new_stack;
}

fn stack_empty2() -> Stack{
    let mut new_stack = Stack::new();
    for i in 0..=9 {
        new_stack.push(i);
    }
    new_stack.empty();
    return new_stack;
}

fn stack_pop_items() -> Stack{
    let mut new_stack = Stack::new();
    for i in 0..=9 {
        new_stack.push(i);
    }
    for _i in 0..=9 {
        new_stack.pop();
    }
    return new_stack;
}

fn stack_test_items() {
    let mut new_stack = Stack::new();
    new_stack.push(5);
    new_stack.push(8);

    assert_eq!(2, new_stack.size);
    assert_eq!(2, new_stack.size());

    new_stack.print();

    let mut value = new_stack.pop();
    assert!(value.take() == Some(8));

    new_stack.print();

    value = new_stack.pop();
    assert!(value.take() == Some(5));

    new_stack.push(53);
    new_stack.push(54);
    new_stack.push(55);
    let a = new_stack.peek();
    let b = new_stack.peek();
    assert_eq!(a, b);
    new_stack.print();
//    new_stack.print();
    new_stack.empty();
    assert!(new_stack.is_empty());
    new_stack.print();
}


#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn bench_stack_test_items() {
        stack_test_items();
    }

    #[test]
    fn bench_simple_option_take_example() {
        simple_option_take_example();
    }

    #[bench]
    fn bench_stack_create(b: &mut Bencher) {
        b.iter(|| stack_create());
    }

    #[bench]
    fn bench_stack_add_items(b: &mut Bencher) {
        b.iter(|| stack_add_items());
    }

    #[bench]
    fn bench_stack_empty(b: &mut Bencher) {
        b.iter(|| stack_empty());
    }

    #[bench]
    fn bench_stack_empty2(b: &mut Bencher) {
        b.iter(|| stack_empty2());
    }

    #[bench]
    fn bench_stack_peek_items(b: &mut Bencher) {
        b.iter(|| stack_peek_items());
    }

    #[bench]
    fn bench_stack_pop_items(b: &mut Bencher) {
        b.iter(|| stack_pop_items());
    }
}