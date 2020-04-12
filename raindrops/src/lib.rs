pub fn raindrops(n: u32) -> String {
    let mut string = String::new();

    let by_3 = n % 3 == 0;
    let by_5 = n % 5 == 0;
    let by_7 = n % 7 == 0;

    if by_3 {
        string += "Pling";
    }

    if by_5 {
        string += "Plang";
    }

    if by_7 {
        string += "Plong";
    }

    if !(by_3 || by_5 || by_7) {
        return n.to_string();
    }

    string
}
