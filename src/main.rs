mod sll;

fn main() {
    let mut llist: sll::LinkedList<int> = sll::LinkedList::new(0i);
    llist.push(1i);
    llist.pop();
    llist.insert(3i, 0);
    llist.insert(2i, 0);
    llist.insert(1i, 0);
    llist.remove(-1);
}
