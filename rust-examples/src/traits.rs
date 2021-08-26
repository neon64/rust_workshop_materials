
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Colour {
    Red,
    Yellow,
    Orange
}

trait Animal {
    fn name(&self) -> &str;
    fn legs(&self) -> u32;
    fn colour(&self) -> Colour;
}

struct Sheep {
    name: String,
    custom_colour: Option<Colour>
}

impl Sheep {
    fn new(name: String) -> Self {
        Sheep {
            name: name,
            custom_colour: None
        }
    }
}

impl Animal for Sheep {
    fn name(&self) -> &str {
        &self.name
    }

    fn legs(&self) -> u32 {
        4
    }

    fn colour(&self) -> Colour {
        // sheep are orange by default!
        self.custom_colour.unwrap_or(Colour::Orange)
    }
}

fn print_stats<A: Animal>(animal: A) {
    println!("{} has {} legs and is {:?}", animal.name(), animal.legs(), animal.colour());
}

#[test]
fn traits1() {
    let dolly = Sheep::new(String::from("Dolly"));

    print_stats(dolly);
}
