pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    if !list.is_empty() {
        proverb += &list.windows(2)
            .map(|win| format!("For want of a {} the {} was lost.\n", win[0], win[1]))
            .collect::<String>();

        proverb += &format!("And all for the want of a {}.", list[0]);
    }
    proverb
}
