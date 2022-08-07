#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};


use rocket::request::{self, Request, FromRequest};
use rocket::State;
use rocket::serde::{Deserialize, json::Json};

struct ProjectsDB {
    projects: Arc<Mutex<HashMap<String, Project>>>,
}

#[derive(Deserialize)]
struct Project {
    name: String,
}

impl Project {
    fn new(name: &str) -> Project {
        Project { name: name.to_string() }
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/projects")]
fn get_projects(projects_state: &State<ProjectsDB>) -> String {

    let shared_projects: &ProjectsDB = projects_state.inner();
    let projects = shared_projects.projects.lock().unwrap();
    let current_size = projects.len();

    for (key, project) in projects.iter() {
        println!("{}: {}", key, project.name);
    }

    format!("Projects length, {}!", current_size)
}

#[post("/projects", format = "json", data = "<project>")]
fn set_projects(project: Json<Project>, projects_state: &State<ProjectsDB>) -> String {
    let shared_projects: &ProjectsDB = projects_state.inner();

    shared_projects.projects.lock().unwrap().insert("test".to_string(), Project::new("asdas"));

    format!("Success, added {}!", project.name)
}


#[launch]
fn rocket() -> _ {

    let projects =  Arc::new(Mutex::new(HashMap::new()));

    rocket::build()
        .manage(ProjectsDB { projects:  projects})
        .mount("/", routes![index, hello, get_projects, set_projects])
}
