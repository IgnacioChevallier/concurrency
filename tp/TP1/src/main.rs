use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Instant;

/// Calcula Pi usando la Serie de Leibniz hasta el término `n`
fn calcular_pi(n: u64) -> f64 {
    let mut pi = 0.0;
    for k in 0..n {
        let term = 4.0 * (-1.0_f64).powi(k as i32) / (2 * k + 1) as f64;
        pi += term;
    }
    pi
}

/// Maneja una conexión HTTP
fn manejar_conexion(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(bytes_leidos) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..bytes_leidos]);

        // Extraer `i` del request (ejemplo: "GET /pi/100 HTTP/1.1")
        if let Some(pos) = request.find("/pi/") {
            if let Some(end) = request[pos + 4..].find(' ') {
                if let Ok(n) = request[pos + 4..pos + 4 + end].parse::<u64>() {
                    let inicio = Instant::now();
                    let pi = calcular_pi(n);
                    let duracion = inicio.elapsed().as_secs_f64();

                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nValor de Pi para el término {}: {:.10} (Tiempo: {:.5} segundos)\n",
                        n, pi, duracion
                    );

                    stream.write_all(response.as_bytes()).unwrap();
                    return;
                }
            }
        }

        // Si no se encontró una URL válida
        let error_response = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nSolicitud inválida";
        stream.write_all(error_response.as_bytes()).unwrap();
    }
}

/// Inicia el servidor HTTP en el puerto 3030
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").expect("No se pudo iniciar el servidor");
    println!("Servidor corriendo en http://localhost:3030");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(|| manejar_conexion(stream)); // Manejo concurrente
        }
    }
}
