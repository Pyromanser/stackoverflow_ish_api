#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use dotenvy::dotenv;

use sqlx::postgres::PgPoolOptions;

mod cors;
mod handlers;
mod models;
mod persistence;

use crate::persistence::answers_dao::{AnswersDao, AnswersDaoImpl};
use crate::persistence::questions_dao::{QuestionsDao, QuestionsDaoImpl};
use cors::CORS;
use handlers::{
    create_answer, create_question, delete_answer, delete_question, read_answers, read_questions,
};

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().expect(".env file not found");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .expect("Failed to connect to Postgres");

    let questions_dao = QuestionsDaoImpl::new(pool.clone());
    let answers_dao = AnswersDaoImpl::new(pool.clone());

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
        .manage(Box::new(questions_dao) as Box<dyn QuestionsDao + Send + Sync>)
        .manage(Box::new(answers_dao) as Box<dyn AnswersDao + Send + Sync>)
}
