use std::io;
use test_rust_docker::*;

fn main() {
    println!("Calculadora simple en Rust");

    loop {
        println!("\nSeleccione una operación:");
        println!("1. Sumar");
        println!("2. Restar");
        println!("3. Multiplicar");
        println!("4. Dividir");
        println!("5. Salir");

        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer la entrada");
        
        let opcion: u32 = match opcion.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, introduce un número válido.");
                continue;
            }
        };

        if opcion == 5 {
            println!("¡Adiós!");
            break;
        }

        println!("Introduce el primer número:");
        let num1 = leer_numero();
        println!("Introduce el segundo número:");
        let num2 = leer_numero();

        match opcion {
            1 => println!("Resultado: {}", sumar(num1, num2)),
            2 => println!("Resultado: {}", restar(num1, num2)),
            3 => println!("Resultado: {}", multiplicar(num1, num2)),
            4 => match dividir(num1, num2) {
                Ok(resultado) => println!("Resultado: {}", resultado),
                Err(err) => println!("Error: {}", err),
            },
            _ => println!("Opción no válida. Por favor, selecciona entre 1 y 5."),
        }
    }
}

fn leer_numero() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, introduce un número válido.");
            leer_numero()
        }
    }
}