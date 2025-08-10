//! Authentication and authorization module for web API

use crate::web::models::{UserClaims, ActorRole, AuthRequest, AuthResponse, ApiError};
use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// JWT secret key (in production, this should be loaded from environment)
const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";

/// User database (in production, this would be a proper database)
type UserDatabase = Arc<RwLock<HashMap<String, UserInfo>>>;

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub username: String,
    pub password_hash: String, // In production, use proper password hashing
    pub role: ActorRole,
}

#[derive(Clone)]
pub struct AuthState {
    pub users: UserDatabase,
}

impl AuthState {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        
        // Add default users for demo purposes
        users.insert("admin".to_string(), UserInfo {
            username: "admin".to_string(),
            password_hash: "admin123".to_string(), // In production, use bcrypt
            role: ActorRole::Admin,
        });
        
        users.insert("farmer1".to_string(), UserInfo {
            username: "farmer1".to_string(),
            password_hash: "farmer123".to_string(),
            role: ActorRole::Farmer,
        });
        
        users.insert("processor1".to_string(), UserInfo {
            username: "processor1".to_string(),
            password_hash: "processor123".to_string(),
            role: ActorRole::Processor,
        });

        Self {
            users: Arc::new(RwLock::new(users)),
        }
    }
}

/// Generate JWT token for authenticated user
pub fn generate_token(username: &str, role: &ActorRole) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = UserClaims {
        sub: username.to_string(),
        role: role.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

/// Validate JWT token and extract claims
pub fn validate_token(token: &str) -> Result<UserClaims, jsonwebtoken::errors::Error> {
    decode::<UserClaims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

/// Authentication handler
pub async fn authenticate(
    State(auth_state): State<AuthState>,
    Json(auth_request): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    let users = auth_state.users.read().await;
    
    if let Some(user_info) = users.get(&auth_request.username) {
        // In production, use proper password verification (bcrypt)
        if user_info.password_hash == auth_request.password {
            match generate_token(&auth_request.username, &user_info.role) {
                Ok(token) => {
                    let expires_at = Utc::now() + Duration::hours(24);
                    Ok(Json(AuthResponse {
                        token,
                        expires_at,
                        user_role: user_info.role.to_string(),
                    }))
                }
                Err(_) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError {
                        error: "token_generation_failed".to_string(),
                        message: "Failed to generate authentication token".to_string(),
                        timestamp: Utc::now(),
                    }),
                )),
            }
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    error: "invalid_credentials".to_string(),
                    message: "Invalid username or password".to_string(),
                    timestamp: Utc::now(),
                }),
            ))
        }
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                error: "invalid_credentials".to_string(),
                message: "Invalid username or password".to_string(),
                timestamp: Utc::now(),
            }),
        ))
    }
}

/// Middleware to verify JWT token
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ApiError>)> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            match validate_token(token) {
                Ok(claims) => {
                    // Add user claims to request extensions
                    request.extensions_mut().insert(claims);
                    Ok(next.run(request).await)
                }
                Err(_) => Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ApiError {
                        error: "invalid_token".to_string(),
                        message: "Invalid or expired authentication token".to_string(),
                        timestamp: Utc::now(),
                    }),
                )),
            }
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    error: "invalid_auth_header".to_string(),
                    message: "Invalid authorization header format".to_string(),
                    timestamp: Utc::now(),
                }),
            ))
        }
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                error: "missing_auth_header".to_string(),
                message: "Authorization header is required".to_string(),
                timestamp: Utc::now(),
            }),
        ))
    }
}

/// Role-based authorization middleware
pub fn require_role(required_role: ActorRole) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, (StatusCode, Json<ApiError>)>> + Send>> + Clone {
    move |request: Request, next: Next| {
        let required_role = required_role.clone();
        Box::pin(async move {
            if let Some(claims) = request.extensions().get::<UserClaims>() {
                // Admin role can access everything
                if claims.role == "admin" || claims.role == required_role.to_string() {
                    Ok(next.run(request).await)
                } else {
                    Err((
                        StatusCode::FORBIDDEN,
                        Json(ApiError {
                            error: "insufficient_permissions".to_string(),
                            message: format!("Role '{}' required for this operation", required_role),
                            timestamp: Utc::now(),
                        }),
                    ))
                }
            } else {
                Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ApiError {
                        error: "authentication_required".to_string(),
                        message: "Authentication is required for this operation".to_string(),
                        timestamp: Utc::now(),
                    }),
                ))
            }
        })
    }
}
