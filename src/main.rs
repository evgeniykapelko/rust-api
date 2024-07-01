use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;

#[derive(Deserialize, Serialize, Clone)]
struct Movie {
    id: Uuid,
    title: String,
    director: String,
}

struct AppState {
    movies: Mutex<Vec<Movie>>,
}

async fn get_movies(data: web::Data<AppState>) -> impl Responder {
    let movies = data.movies.lock().unwrap();
    HttpResponse::Ok().json(&*movies)
}

async fn get_movie(data: web::Data<AppState>, movie_id: web::Path<Uuid>) -> impl Responder {
    let movies = data.movies.lock().unwrap();
    if let Some(movie) = movies.iter().find(|m| m.id == *movie_id) {
        HttpResponse::Ok().json(movie)
    } else {
        HttpResponse::NotFound().body("Movie not found")
    }
}

async fn create_movie(data: web::Data<AppState>, new_movie: web::Json<Movie>) -> impl Responder {
    let mut movies = data.movies.lock().unwrap();
    let movie = Movie {
        id: Uuid::new_v4(),
        title: new_movie.title.clone(),
        director: new_movie.director.clone(),
    };
    movies.push(movie.clone());
    HttpResponse::Created().json(movie)
}

async fn update_movie(data: web::Data<AppState>, movie_id: web::Path<Uuid>, updated_movie: web::Json<Movie>) -> impl Responder {
    let mut movies = data.movies.lock().unwrap();
    if let Some(movie) = movies.iter_mut().find(|m| m.id == *movie_id) {
        movie.title = updated_movie.title.clone();
        movie.director = updated_movie.director.clone();
        HttpResponse::Ok().json(movie)
    } else {
        HttpResponse::NotFound().body("Movie not found")
    }
}

async fn delete_movie(data: web::Data<AppState>, movie_id: web::Path<Uuid>) -> impl Responder {
    let mut movies = data.movies.lock().unwrap();
    if let Some(pos) = movies.iter().position(|m| m.id == *movie_id) {
        movies.remove(pos);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("Movie not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        movies: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/movies", web::get().to(get_movies))
            .route("/movies/{id}", web::get().to(get_movie))
            .route("/movies", web::post().to(create_movie))
            .route("/movies/{id}", web::put().to(update_movie))
            .route("/movies/{id}", web::delete().to(delete_movie))
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}

