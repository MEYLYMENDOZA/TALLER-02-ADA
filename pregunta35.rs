use std::time::Instant;

fn eliminar_duplicados(arr: &[i32], index: usize, resultado: &mut Vec<i32>) {
    if index == arr.len() {
        return;  
    }

    if !resultado.contains(&arr[index]) {
        resultado.push(arr[index]);
    }

    eliminar_duplicados(arr, index + 1, resultado);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sin_duplicados() {
        let arr = [1, 2, 3, 2, 4, 1];
        let mut resultado = Vec::new();
        eliminar_duplicados(&arr, 0, &mut resultado);
        assert_eq!(resultado, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_todo_duplicado() {
        let arr = [5, 5, 5, 5];
        let mut resultado = Vec::new();
        eliminar_duplicados(&arr, 0, &mut resultado);
        assert_eq!(resultado, vec![5]);
    }
}

fn main() {
    let tamanios = vec![10, 100, 500];  

    for &n in &tamanios {
        let arr: Vec<i32> = (0..n as i32).chain(0..(n as i32) / 2).collect();  
        let mut resultado = Vec::new();

        let inicio = Instant::now();
        eliminar_duplicados(&arr, 0, &mut resultado);
        let duracion = inicio.elapsed();

        println!("Tamaño: {}, Tiempo: {:?}", n, duracion);
    }
}