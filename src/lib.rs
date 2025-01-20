pub fn sumar(a: f64, b: f64) -> f64 {
    a + b
}

pub fn restar(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiplicar(a: f64, b: f64) -> f64 {
    a * b
}

pub fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("División por cero no permitida"))
    } else {
        Ok(a / b)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_sumar() {
//         assert_eq!(sumar(2.0, 3.0), 5.0);
//     }

//     #[test]
//     fn test_restar() {
//         assert_eq!(restar(5.0, 3.0), 2.0);
//     }

//     #[test]
//     fn test_multiplicar() {
//         assert_eq!(multiplicar(2.0, 3.0), 6.0);
//     }

//     #[test]
//     fn test_dividir() {
//         assert_eq!(dividir(6.0, 3.0).unwrap(), 2.0);
//     }

//     #[test]
//     fn test_dividir_por_cero() {
//         assert!(dividir(6.0, 0.0).is_err());
//     }
// }
