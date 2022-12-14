use inquire::Text;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn entrypoint() -> Result<(), &'static str> {
    let result = Text::new("Enter list items through a space")
        .prompt()
        .map(|input| {
            input
                .split(" ")
                .map(|i| i.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|mut items| {
            items.shuffle(&mut thread_rng());
            items
        });

    match result {
        Ok(shuffled) => {
            for item in shuffled {
                println!("{}", item);
            }

            Ok(())
        }
        Err(_) => Err("Error ocurred"),
    }
}
