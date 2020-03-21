use indexmap::IndexSet;

fn main() {
    let mut set: IndexSet<usize> = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(6);

    println!("{:?}", set.get(&1));
}
