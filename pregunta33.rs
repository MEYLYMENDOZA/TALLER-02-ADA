use std::time::Instant;
fn esta_ordenado(arr: &[i32], index: usize) -> bool {
    if index >= arr.len() - 1 {
        return true;
    }
    if arr[index] >= arr[index + 1] {
        return false;
    }
    esta_ordenado(arr, index + 1)
}

#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    fn test_arreglo_creciente() {
        let arr = [0, 1, 2, 3, 4, 5];
        let ordenado = esta_ordenado(&arr, 0);
        assert!(ordenado);
    }

    #[test]
    fn test_arreglo_enorme() {
        let arr: Vec<i32> = (0..10000).collect();
        let ordenado = esta_ordenado(&arr, 0);
        assert!(ordenado);
    }
}

// Función para medir tiempos
fn test_tiempos() {
    let sizes = [100, 1000, 5000];

    for &size in sizes.iter() {
        let arr: Vec<i32> = (0..size as i32).collect(); 
        
        let inicio = Instant::now();
        let orden = esta_ordenado(&arr, 0); 
        let duracion = inicio.elapsed();
        println!("Tamaño: {}, Ordenado: {}, Tiempo: {:?}", size, orden, duracion);
    }
}

fn main() {
    test_tiempos();
}
