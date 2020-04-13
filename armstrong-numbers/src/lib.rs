fn order(mut num: u32) -> u32 {
    let mut order = 0;
    while num > 0 {
        order += 1;
        num = num / 10;
    }

    order
}

pub fn is_armstrong_number(num: u32) -> bool {
    let order = order(num);

    let mut sum: u32 = 0;
    let mut res = num;

    while res > 0 {
        let digit = res % 10;
        sum += digit.pow(order);
        res = res / 10;
    }

    num == sum
}
