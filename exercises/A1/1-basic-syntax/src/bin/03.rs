fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let a = match input.iter().max() {
        Some(num) => num, // 如果是 Some，就返回里面的值
        None => panic!("input is empty"), // 如果是 None，就 panic 并给出原因
    };
    
    let b = match input.iter().min() {
        Some(num) => num, // 如果是 Some，就返回里面的值
        None => panic!("input is empty"), // 如果是 None，就 panic 并给出原因
    };

    println!("{} is largest and {} is smallest", a, b);
}
