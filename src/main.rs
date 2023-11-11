#[macro_use] extern crate rocket;

use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use csv::Reader;
use rand::Rng;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Person {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub email: String,
    pub gender: String
}

fn read_to_vec(file_path: &str) -> Vec<String>{
    let mut results: Vec<String> = vec![];
    let mut rdr = Reader::from_path(file_path).unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        results.push(String::from(record.get(0).unwrap()));
    }
    return results;
}

#[get("/people?<size>")]
fn index(size: u32) -> Json<Vec<Person>> {
    let first_names: Vec<String> = read_to_vec("male_first_names.csv");
    let last_names:Vec<String> = read_to_vec("last_names.csv");

    let mut rng = rand::thread_rng();

    let mut results: Vec<Person> = vec![];
    for _ in 0..size {
        let first_name = &first_names[rng.gen_range(0..first_names.len())];
        let middle_name =&first_names[rng.gen_range(0..first_names.len())];
        let last_name = &last_names[rng.gen_range(0..last_names.len())];
        results.push(Person {
            first_name: first_name.clone(),
            middle_name: middle_name.clone(),
            last_name: last_name.clone(),
            gender: String::from("male"),
            email: format!("{first_name}.{last_name}@email.com")
        })
    }
    return Json(results);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index])
}