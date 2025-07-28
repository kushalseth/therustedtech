use dialoguer::FuzzySelect;

fn main() {
    let items = vec!["apple", "banana", "grape", "orange", "watermelon"];

    let selection = FuzzySelect::new()
        .with_prompt("Pick your favorite fruit")
        .items(&items)
        .interact()
        .unwrap();

    println!("You selected: {}", items[selection]);
}