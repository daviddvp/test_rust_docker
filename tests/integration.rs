use test_rust_docker::*;

#[test]
fn test_operaciones_basicas() {
    assert_eq!(sumar(10.0, 5.0), 15.0);
    assert_eq!(restar(10.0, 5.0), 5.0);
    assert_eq!(multiplicar(10.0, 5.0), 50.0);
    assert_eq!(dividir(10.0, 5.0).unwrap(), 2.0);
}

#[test]
fn test_division_por_cero() {
    assert!(dividir(10.0, 0.0).is_err());
}