use std::time::Instant;

fn suma_dividir_conquistar(arr: &[i32], inicio: usize, fin: usize) -> i32 {
    if inicio > fin {
        return 0;
    }
    if inicio == fin {
        return arr[inicio]; 
    }
    let medio = (inicio + fin) / 2;
    let suma_izquierda = suma_dividir_conquistar(arr, inicio, medio);
    let suma_derecha = suma_dividir_conquistar(arr, medio + 1, fin);
    suma_izquierda + suma_derecha
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arreglo_pequeno() {
        let arr = [1, 2, 3, 4, 5];
        let suma = suma_dividir_conquistar(&arr, 0, arr.len() - 1);
        assert_eq!(suma, 15);
    }

    #[test]
    fn test_arreglo_negativos() {
        let arr = [-1, -2, -3, -4];
        let suma = suma_dividir_conquistar(&arr, 0, arr.len() - 1);
        assert_eq!(suma, -10); 
    }
}

fn main() {
    let tamanios = vec![10, 1_000, 100_000, 1_000_000];

    for &n in &tamanios {
        let arr = vec![1; n];

        let inicio_tiempo = Instant::now();
        let suma = suma_dividir_conquistar(&arr, 0, arr.len() - 1);
        let duracion = inicio_tiempo.elapsed();

        println!("Tamaño: {}, Suma: {}, Tiempo: {:?}", n, suma, duracion);
    }
}