use std::time::Instant;

fn intercalar(arr1: &[i32], arr2: &[i32], index: usize, resultado: &mut Vec<i32>) {
    if index == arr1.len() {
        return; 
    }

    resultado.push(arr1[index]);  
    resultado.push(arr2[index]);  

    intercalar(arr1, arr2, index + 1, resultado);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intercalar_basico() {
        let arr1 = [1, 3, 5];
        let arr2 = [2, 4, 6];
        let mut resultado = Vec::new();
        intercalar(&arr1, &arr2, 0, &mut resultado);
        assert_eq!(resultado, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_intercalar_negativos() {
        let arr1 = [-1, -3];
        let arr2 = [-2, -4];
        let mut resultado = Vec::new();
        intercalar(&arr1, &arr2, 0, &mut resultado);
        assert_eq!(resultado, vec![-1, -2, -3, -4]);
    }
}

fn main() {
    let tamanios = vec![2, 5, 10]; 

    for &n in &tamanios {
        let arr1: Vec<i32> = (1..=n as i32).collect();
        let arr2: Vec<i32> = (n as i32 + 1..=2 * n as i32).collect();
        let mut resultado = Vec::new();

        let inicio = Instant::now();
        intercalar(&arr1, &arr2, 0, &mut resultado);
        let duracion = inicio.elapsed();

        println!("Tamaño: {}, Tiempo de ejecución: {:?}", n, duracion);
    }
}
