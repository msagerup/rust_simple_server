use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Result, Write};
use std::sync::{Mutex, MutexGuard};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DB {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl DB {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // TASK FUNCTIONS
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }
    // Get task by id
    fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }
    // Get all tasks
    fn get_all(&self, id: &u64) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    // delete task
    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }
    // update task
    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // USER FUNCTIONS
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    // Search for username, with find method.
    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // DB saving.
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(self)?;
        let mut file: fs::File = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("database.json")?;
        let db = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<DB>,
}

async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: MutexGuard<DB> = app_state.db.lock().unwrap();
    db.insert(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> Result<()> {
    let db: DB = match DB::load_from_file() {
        Ok(db) => db,
        Err(_) => DB::new(),
    };

    let data: web::Data<AppState> = web::Data::new(AppState { db: Mutex::new(db) });
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(
                        |origin: &header::HeaderValue, _req_head: &actix_web::dev::RequestHead| {
                            origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                        },
                    )
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/task", web::post().to(create_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
