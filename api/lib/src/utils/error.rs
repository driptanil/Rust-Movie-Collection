use actix_web::{
    body::BoxBody,
    error::{ JsonPayloadError, QueryPayloadError },
    http::StatusCode,
    HttpRequest,
    HttpResponse,
    Responder,
    ResponseError,
};
use std::fmt::Debug;
use super::message::MessageSchema;

#[derive(thiserror::Error, Debug, Clone)]
pub enum ApiError {
    #[error("{0}")] InternalServer(String),
    #[error("{0}")] BadRequest(String),
    #[error("{0}")] NotFound(String),
    #[error("{0}")] Forbidden(String),
    #[error("{0}")] Unauthorized(String),
    #[error("Too many requests, retry in {0}s")] TooManyRequests(u64),
}

pub trait ApiErrorTrait {
    type Output;

    fn server_err(self, message: &str) -> Self::Output;
    fn bad_request_err(self, message: &str) -> Self::Output;
    fn not_found_err(self, message: &str) -> Self::Output;
    fn unauthorized_err(self, message: &str) -> Self::Output;

    fn database_err(self) -> Self::Output;
    fn already_username_err(self, username: &str) -> Self::Output;
    fn key_creation_err(self) -> Self::Output;
    fn invalid_token_err(self) -> Self::Output;
    fn incorrect_user_err(self) -> Self::Output;
}

impl<T, E> ApiErrorTrait for std::result::Result<T, E> {
    type Output = ApiResult<T>;

    fn server_err(self, message: &str) -> Self::Output {
        self.map_err(|_| ApiError::InternalServer(message.to_string()))
    }

    fn bad_request_err(self, message: &str) -> Self::Output {
        self.map_err(|_| ApiError::BadRequest(message.to_string()))
    }

    fn not_found_err(self, message: &str) -> Self::Output {
        self.map_err(|_| ApiError::NotFound(message.to_string()))
    }

    fn unauthorized_err(self, message: &str) -> Self::Output {
        self.map_err(|_| ApiError::Unauthorized(message.to_string()))
    }

    fn database_err(self) -> Self::Output {
        self.server_err("Database error")
    }

    fn already_username_err(self, username: &str) -> Self::Output {
        self.bad_request_err(&format!("Username `{}` already exists", username))
    }

    fn key_creation_err(self) -> Self::Output {
        self.server_err("Error while creating the key")
    }

    fn invalid_token_err(self) -> Self::Output {
        self.unauthorized_err("The token is invalid")
    }

    fn incorrect_user_err(self) -> Self::Output {
        self.bad_request_err("The username or password is incorrect")
    }
}

impl<T> ApiErrorTrait for Option<T> {
    type Output = ApiResult<T>;

    fn server_err(self, message: &str) -> Self::Output {
        self.ok_or_else(|| ApiError::InternalServer(message.to_string()))
    }

    fn bad_request_err(self, message: &str) -> Self::Output {
        self.ok_or_else(|| ApiError::BadRequest(message.to_string()))
    }

    fn not_found_err(self, message: &str) -> Self::Output {
        self.ok_or_else(|| ApiError::NotFound(message.to_string()))
    }

    fn unauthorized_err(self, message: &str) -> Self::Output {
        self.ok_or_else(|| ApiError::Unauthorized(message.to_string()))
    }

    fn database_err(self) -> Self::Output {
        self.server_err("Database error")
    }

    fn already_username_err(self, username: &str) -> Self::Output {
        self.bad_request_err(&format!("Username `{}` already exists", username))
    }

    fn key_creation_err(self) -> Self::Output {
        self.server_err("Error while creating the key")
    }

    fn invalid_token_err(self) -> Self::Output {
        self.unauthorized_err("The token is invalid")
    }

    fn incorrect_user_err(self) -> Self::Output {
        self.bad_request_err("The username or password is incorrect")
    }
}

impl From<QueryPayloadError> for ApiError {
    fn from(err: QueryPayloadError) -> Self {
        match err {
            QueryPayloadError::Deserialize(err) => Self::BadRequest(err.to_string()),
            _ => Self::BadRequest("The parameters query are invalid".to_string()),
        }
    }
}

impl From<JsonPayloadError> for ApiError {
    fn from(err: JsonPayloadError) -> Self {
        match err {
            JsonPayloadError::ContentType => {
                Self::BadRequest("The content type is not `application/json`".to_string())
            }
            JsonPayloadError::Deserialize(err) => {
                Self::BadRequest(format!("The request body is invalid: {}", err))
            }
            _ => Self::BadRequest("The request body is invalid".to_string()),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MessageSchema::from(self.clone()))
    }
}

impl Responder for ApiError {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;
