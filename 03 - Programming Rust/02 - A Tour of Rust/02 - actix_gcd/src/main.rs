use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;


#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}


fn main() {
    
    /* Create a new server object. */
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    
    println!("Serving on http://localhost:3000...");
    
    /* Run the server. */
    server.bind("127.0.0.1:3000")
        .expect("Error binding server to address.")
        .run()
        .expect("Error running server.");
}


fn get_index() -> HttpResponse {
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,  
        )
}


fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    
    /* Check for invalid values */
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.")
    }
    
    /* Create the response of the gcd calculation */
    let response = 
        format!(
            "The greatest common divisor of the numbers {} and {} \
            is <b>{}<b>\n",
            form.n, form.m, gcd(form.n, form.m)
        );
        
    /* Send the response back */
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}


fn gcd(mut a: u64, mut b: u64) -> u64 {

    /* Check the input variables */
    assert!(a != 0 && b != 0);
    
    /* Until the remainder is zero. */
    while b != 0 {
        
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    
    return a;
}

