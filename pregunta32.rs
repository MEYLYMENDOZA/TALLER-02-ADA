use std::time::Instant;

fn contar_subsecuencias(arr: &[i32], index: usize, suma_actual: i32, objetivo: i32) -> i32 {
    if index == arr.len() {
        return if suma_actual == objetivo { 1 } else { 0 };  
    }

    let incluir = contar_subsecuencias(arr, index + 1, suma_actual + arr[index], objetivo);

    let excluir = contar_subsecuencias(arr, index + 1, suma_actual, objetivo);

    incluir + excluir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsecuencias_objetivo_5() {
        let arr = [1, 2, 3];
        let resultado = contar_subsecuencias(&arr, 0, 0, 5);
        assert_eq!(resultado, 1); 
    }

    #[test]
    fn test_subsecuencias_objetivo_3() {
        let arr = [1, 2, 3];
        let resultado = contar_subsecuencias(&arr, 0, 0, 3);
        assert_eq!(resultado, 2);  
    }
}

fn main() {
    let tamanios = vec![3, 5, 10];

    for &n in &tamanios {
        let arr = vec![1; n]; 
        let objetivo = (n / 2) as i32;  

        let inicio = Instant::now();
        let resultado = contar_subsecuencias(&arr, 0, 0, objetivo);
        let duracion = inicio.elapsed();

        println!("Tamaño: {}, Subsecuencias que suman {}: {}, Tiempo: {:?}", n, objetivo, resultado, duracion);
    }
}