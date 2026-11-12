use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Advinhe o número!!!");

    let secret_number = rand::rng().random_range(1..=10);

    loop {
        println!("Por favor, insira sua sugestão de número:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você chutou: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Valor muito abaixo!"),
            Ordering::Greater => println!("Valor muito alto!"),
            Ordering::Equal => {
                println!("Você venceu!");
                break;
            }
        }
    }
}
