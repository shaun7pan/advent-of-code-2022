fn main() {
    let grid = include_bytes!("../input.txt");
    println!("{:?}", grid);
    grid.split(|b| b == &b'\n');
}
