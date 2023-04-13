use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn es_primo(numero: u64) -> bool {
    if numero <= 1 {
        return false;
    }

    let limite = (numero as f64).sqrt() as u64 + 1;
    for i in 2..limite {
        if numero % i == 0 {
            return false;
        }
    }
    true
}

fn suma_primos(n: u64) -> u64 {
    let mut suma = 0;
    let mut contador = 0;
    let mut numero = 2;

    while contador < n {
        if es_primo(numero) {
            suma += numero;
            contador += 1;
        }
        numero += 1;
    }

    suma
}

async fn manual_suma_primos() -> impl Responder  {
    let n = 1000000; // Número de primos a sumar
    let resultado = suma_primos(n);
    let s = format!("La suma de los primeros {} números primos es: {}", n, resultado);
    HttpResponse::Ok().body(s)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/primo", web::get().to(manual_suma_primos))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}