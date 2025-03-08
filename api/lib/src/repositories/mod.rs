use actix_web::web;
use sea_orm::DatabaseConnection;

use base_repository::BaseRepository;
use movie_repository::MovieRepository;
use user_repository::UserRepository;
use user_password_repository::UserPasswordRepository;
use user_session_repository::UserSessionRepository;

pub mod base_repository;
pub mod movie_repository;
pub mod version_repository;
pub mod user_repository;
pub mod user_session_repository;
pub mod user_password_repository;

pub type Repository = web::Data<Box<dyn AppRepository>>;

pub trait AppRepository: UserRepository +
    UserPasswordRepository +
    UserSessionRepository +
    MovieRepository +
    Send +
    Sync +
    'static {}

impl AppRepository for DatabaseConnection {}
