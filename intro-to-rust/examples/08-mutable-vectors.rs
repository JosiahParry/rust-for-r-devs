fn main() {
    let mut x = Vec::new();
    x.push(1.0);
    x.push(2.0);
    x.push(3.0);
    x.clear();
    x.extend(vec![4.0, 5.0]);
    x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", x);
}
