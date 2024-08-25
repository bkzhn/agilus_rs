use axum::{
    routing::{get, post, put, delete},
    response::IntoResponse,
    http::StatusCode,
    Json,
    Router,
};
use axum::extract::Path;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/healthcheck", get(healthcheck))
        .route("/api/v1/users/", post(create_user))
        .route("/api/v1/users", get(list_users))
        .route(
            "/api/v1/users/:id",
            get(get_user)
                .put(update_user)
                .delete(delete_user)
        )
        .route("/api/v1/tickets/", post(create_ticket))
        .route("/api/v1/tickets", get(list_tickets))
        .route(
            "/api/v1/tickets/:id",
            get(get_ticket)
                .put(update_ticket)
                .delete(delete_ticket)
        )
        .route("/api/v1/ticket_types/", post(create_ticket_type))
        .route("/api/v1/ticket_types", get(list_ticket_types))
        .route(
            "/api/v1/ticket_types/:id",
            get(get_ticket_type)
                .put(update_ticket_type)
                .delete(delete_ticket_type)
        )
        .route("/api/v1/ticket_statuses/", post(create_ticket_status))
        .route("/api/v1/ticket_statuses", get(list_ticket_statuses))
        .route(
            "/api/v1/ticket_statuses/:id",
            get(get_ticket_status)
                .put(update_ticket_status)
                .delete(delete_ticket_status)
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    tracing::debug!("listening {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Agilus"
}

async fn healthcheck() -> &'static str {
    "OK"
}

// Users
async fn create_user(
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User {
        id: 1,
        name: payload.name,
    };

    (StatusCode::CREATED, Json(user))
}

async fn list_users() -> impl IntoResponse {
    let users: [User; 0] = [];

    (StatusCode::OK, Json(users))
}

async fn get_user(
    Path(id): Path<u64>
) -> impl IntoResponse {
    let user = User {
        id: id,
        name: format!("user {}", id)
    };

    (StatusCode::OK, Json(user))
}

async fn update_user(
    Path(id): Path<u64>,
    Json(payload): Json<UpdateUser>,
) -> impl IntoResponse {
    let user = User {
        id: id,
        name: payload.name,
    };

    (StatusCode::OK, Json(user))
}

async fn delete_user(
    Path(id): Path<u64>
) -> impl IntoResponse {
    (StatusCode::OK, format!("Deleted user {}", id))
}

// Tickets
async fn create_ticket(
    Json(payload): Json<CreateTicket>,
) -> impl IntoResponse {
    let ticket = Ticket {
        id: 1,
        title: payload.title,
        description: payload.description,
        type_id: payload.type_id,
        status_id: payload.status_id,
        author_id: payload.author_id,
        assignee_id: payload.assignee_id.unwrap(),
    };

    (StatusCode::CREATED, Json(ticket))
}

async fn list_tickets() -> impl IntoResponse {
    let tickets: [Ticket; 0] = [];

    (StatusCode::OK, Json(tickets))
}

async fn get_ticket(
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let ticket = Ticket {
        id: id,
        title: format!("Sample Title {}", id),
        description: format!("Sample Description {}", id),
        type_id: 1,
        status_id: 1,
        author_id: 1,
        assignee_id: 1,
    };

    (StatusCode::OK, Json(ticket))
}

async fn update_ticket(
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTicket>,
) -> impl IntoResponse {
    let ticket = Ticket {
        id: id,
        title: payload.title.unwrap(),
        description: payload.description.unwrap(),
        type_id: payload.type_id.unwrap(),
        status_id: payload.status_id.unwrap(),
        author_id: payload.author_id.unwrap(),
        assignee_id: payload.assignee_id.unwrap(),
    };

    (StatusCode::OK, Json(ticket))
}

async fn delete_ticket(
    Path(id): Path<u64>,
) -> impl IntoResponse {
    (StatusCode::OK, format!("Deleted ticket {}", id))
}

// Ticket Types
async fn create_ticket_type(
    Json(payload): Json<CreateTicketType>,
) -> impl IntoResponse {
    let ticket_type = TicketType {
        id: 1,
        name: payload.name,
    };

    (StatusCode::CREATED, Json(ticket_type))
}

async fn list_ticket_types() -> impl IntoResponse {
    let ticket_types: [TicketType; 0] = [];

    (StatusCode::OK, Json(ticket_types))
}

async fn get_ticket_type(
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let ticket_type = TicketType {
        id: id,
        name: format!("Sample Ticket Type {}", id),
    };

    (StatusCode::OK, Json(ticket_type))
}

async fn update_ticket_type(
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTicketType>,
) -> impl IntoResponse {
    let ticket_type = TicketType {
        id: id,
        name: payload.name,
    };

    (StatusCode::OK, Json(ticket_type))
}

async fn delete_ticket_type(
    Path(id): Path<u64>,
) -> impl IntoResponse {
    (StatusCode::OK, format!("Deleted ticket type {}", id))
}

// Ticket Statuses
async fn create_ticket_status(
    Json(payload): Json<CreateTicketStatus>,
) -> impl IntoResponse {
    let ticket_status = TicketStatus {
        id: 1,
        name: payload.name,
    };

    (StatusCode::OK, Json(ticket_status))
}

async fn list_ticket_statuses() -> impl IntoResponse {
    let ticket_statuses: [TicketStatus; 0] = [];

    (StatusCode::OK, Json(ticket_statuses))
}

async fn get_ticket_status(
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let ticket_status = TicketStatus {
        id: id,
        name: format!("Sample Ticket Status {}", id),
    };

    (StatusCode::OK, Json(ticket_status))
}

async fn update_ticket_status(
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTicketStatus>,
) -> impl IntoResponse {
    let ticket_status = TicketStatus {
        id: id,
        name: payload.name,
    };

    (StatusCode::OK, Json(ticket_status))
}

async fn delete_ticket_status(
    Path(id): Path<u64>,
) -> impl IntoResponse {
    (StatusCode::OK, format!("Deleted ticket status {}", id))
}


#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(Deserialize)]
struct UpdateUser {
    name: String,
}

#[derive(Deserialize)]
struct CreateTicket {
    title: String,
    description: String,
    type_id: u64,
    status_id: u64,
    author_id: u64,
    assignee_id: Option<u64>,
}

#[derive(Deserialize)]
struct UpdateTicket {
    title: Option<String>,
    description: Option<String>,
    type_id: Option<u64>,
    status_id: Option<u64>,
    author_id: Option<u64>,
    assignee_id: Option<u64>,
}

#[derive(Deserialize)]
struct CreateTicketType {
    name: String,
}

#[derive(Deserialize)]
struct UpdateTicketType {
    name: String,
}

#[derive(Deserialize)]
struct CreateTicketStatus {
    name: String,
}

#[derive(Deserialize)]
struct UpdateTicketStatus {
    name: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

#[derive(Serialize)]
struct TicketType {
    id: u64,
    name: String,
}

#[derive(Serialize)]
struct TicketStatus {
    id: u64,
    name: String,
}

#[derive(Serialize)]
struct Ticket {
    id: u64,
    title: String,
    description: String,
    type_id: u64,
    status_id: u64,
    author_id: u64,
    assignee_id: u64,
}
