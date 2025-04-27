use std::time::Instant;

fn producto_cruzado(a: &[i32], b: &[i32], i: usize, j: usize) {
    if i == a.len() {
        return; 
    }

    if j == b.len() {
        producto_cruzado(a, b, i + 1, 0);
        return;
    }

    println!("a[{}] * b[{}] = {}", i, j, a[i] * b[j]);

    producto_cruzado(a, b, i, j + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producto_cruzado_pequeno() {
        let a = [1, 2];
        let b = [3, 4];
        producto_cruzado(&a, &b, 0, 0); 
    }

    #[test]
    fn test_producto_cruzado_con_negativos() {
        let a = [2, -1];
        let b = [-3, 5];
        producto_cruzado(&a, &b, 0, 0);
    }
}

fn main() {
    let tamanios = vec![2, 10, 100]; 

    for &n in &tamanios {
        let a: Vec<i32> = (1..=n as i32).collect();
        let b: Vec<i32> = (1..=n as i32).collect();

        let inicio = Instant::now();
        producto_cruzado(&a, &b, 0, 0);
        let duracion = inicio.elapsed();

        println!("Tamaño de arreglos: {}, Tiempo de ejecución: {:?}", n, duracion);
    }
}