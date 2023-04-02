use std::time::Instant;
use std::vec::Vec;

use rand::Rng;

fn main() {
    let mut estado_inicial: Vec<[i32; 50]> = Vec::new();

    estado_inicial.push([0; 50]);

    let inicio = Instant::now();

    let resultado = busqueda_voraz(&mut estado_inicial);

    if resultado {
        println!("Se logró el resultado");
        println!("Tiempo de ejecución: {:?}", inicio.elapsed());
    }
}

fn expand(estado_actual: [i32; 50]) -> Vec<[i32; 50]> {
    let n_reinas = estado_actual.len();
    let mut os: Vec<[i32; 50]> = Vec::new();

    for i in 0..n_reinas {
        for j in 1..n_reinas {
            let movimiento = estado_actual[i] as usize + j;
            let mut configuracion_clon: [i32; 50] = estado_actual.clone();

            if movimiento < n_reinas {
                configuracion_clon[i] = movimiento as i32;
                os.push(configuracion_clon);
                continue;
            }

            let movimiento: i32 = (movimiento - n_reinas) as i32;

            configuracion_clon[i] = movimiento;
            os.push(configuracion_clon)
        }
    }

    return os;
}

fn checar_ataques(estado_actual: &[i32; 50]) -> i32 {
    let n_reinas = estado_actual.len();
    let mut numero_ataques = 0;

    for i in 0..n_reinas {
        let posicion_a = estado_actual[i];

        for j in i + 1..n_reinas {
            let posicion_b = estado_actual[j];

            if posicion_a == posicion_b {
                numero_ataques += 2;
                continue;
            }

            if ((j - i) as i32).abs() == (posicion_a - posicion_b).abs() {
                numero_ataques += 2;
                continue;
            }
        }
    }

    return numero_ataques;
}

fn goaltest(estado_actual: &[i32; 50]) -> bool {
    let numero_ataques = checar_ataques(&estado_actual);

    println!("{}", numero_ataques);

    if numero_ataques == 0 {
        return true;
    }

    return false;
}

fn evaluate(os: &mut Vec<[i32; 50]>) {
    return os.sort_by_key(|c| checar_ataques(&c));
}

fn busqueda_voraz(frontera: &mut Vec<[i32; 50]>) -> bool {
    if frontera.is_empty() {
        return false;
    }

    let estado_actual: [i32; 50] = match frontera.pop() {
        Some(result) => result,
        None => [0; 50],
    };

    if goaltest(&estado_actual) {
        return true;
    }

    let mut os = expand(estado_actual);

    evaluate(&mut os);

    let minimo = checar_ataques(&os[0]);

    let mut empates: i32 = 0;

    for i in 0..os.len() {
        if minimo != checar_ataques(&os[i]) {
            break;
        }
        empates += 1;
    }

    frontera.push(os[rand::thread_rng().gen_range(0..empates) as usize]);

    return busqueda_voraz(frontera);
}
