fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding...");
    } else {
        println!("Not proceeding...");
    }

    let height = 190;
    if height > 180 {
        println!("Tall");
    } else if height < 150 {
        println!("Short");
    } else {
        println!("Avarage");
    }
}
