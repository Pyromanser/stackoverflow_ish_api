mod handlers_inner;

use rocket::serde::json::Json;

use crate::models::{Answer, AnswerDetail, AnswerId, Question, QuestionDetail, QuestionId};

#[post("/question", data = "<question>")]
pub async fn create_question(question: Json<Question>) -> Json<QuestionDetail> {
    Json(QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: question.title.clone(),
        description: question.description.clone(),
        created_at: "created_at".to_owned(),
    })
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    Json(vec![QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(question_uuid: Json<QuestionId>) {}

#[post("/answer", data = "<answer>")]
pub async fn create_answer(answer: Json<Answer>) -> Json<AnswerDetail> {
    Json(AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: answer.question_uuid.clone(),
        content: answer.content.clone(),
        created_at: "created_at".to_owned(),
    })
}

#[get("/answers")]
pub async fn read_answers() -> Json<Vec<AnswerDetail>> {
    Json(vec![AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: "question_uuid".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(answer_uuid: Json<AnswerId>) {}
