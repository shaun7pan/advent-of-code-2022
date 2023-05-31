fn main() {
    let content = include_bytes!("../input.txt");

    let index = content
        .windows(4)
        .position(|x| {
            for i in 0..3 {
                for j in i + 1..4 {
                    if x[i] == x[j] {
                        return false;
                    }
                }
            }
            return true;
        })
        .unwrap()
        + 4;

    println!("{}", index);

    let index2 = content
        .windows(14)
        .position(|x| {
            for i in 0..13 {
                for j in i + 1..14 {
                    if x[i] == x[j] {
                        return false;
                    }
                }
            }
            return true;
        })
        .unwrap()
        + 14;

    println!("{}", index2);
}
