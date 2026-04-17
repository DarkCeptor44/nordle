// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(clippy::needless_for_each)]

use crate::server::{Service, utils::is_valid_word};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use chrono::Local;
use cuid2::slug;
use log::debug;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    info(title = "Nordle API v1", version = "1.0.0"),
    paths(check, guess, health, new),
    components(schemas(
        CheckResponse,
        GuessRequest,
        GuessResponse,
        GuessError,
        NewResponse,
        NewError
    ))
)]
pub struct ApiDocV1;

pub fn routes() -> Router<Arc<Service>> {
    Router::new()
        .route("/check/{word}", get(check))
        .route("/guess", post(guess))
        .route("/health", get(health))
        .route("/new", get(new))
}

#[derive(Serialize, ToSchema)]
struct CheckResponse<'a> {
    /// The word checked
    word: &'a str,

    /// Whether the word is valid or not
    valid: bool,
}

/// Check if a word is a valid guess
#[utoipa::path(get, path = "/api/v1/check/{word}",params(
    ("word" = String, Path, description = "The word to check", example = "abore"),
),responses(
    (status = 200, description = "Valid word", body = CheckResponse, example = json!(CheckResponse{word:"abore",valid:true})),
    (status = 200, description = "Invalid word", body = CheckResponse, example = json!(CheckResponse{word:"adskl",valid:false})),
))]
async fn check(Path(word): Path<String>) -> impl IntoResponse {
    let valid = is_valid_word(&word);

    debug!("validation check for: word={word} valid={valid}");
    (StatusCode::OK, Json(CheckResponse { word: &word, valid })).into_response()
}

#[derive(Deserialize, ToSchema)]
struct GuessRequest {
    /// Game session ID
    id: String,

    /// Guess
    guess: String,
}

#[derive(Serialize, ToSchema)]
struct GuessResponse<'a> {
    /// The results of the guess
    result: Vec<&'a str>,

    /// Whether the game has been won
    won: bool,
}

#[derive(Serialize, ToSchema)]
struct GuessError<'a> {
    /// Error message
    message: &'a str,
}

/// Guess a word
#[utoipa::path(post, path = "/api/v1/guess", request_body(content = GuessRequest, description = "The object sent to the endpoint", examples(
    ("Example" = (value = json!({"id":"lprcp4yara","guess":"abore"}))),
)),responses(
    (status = 200, description = "Successful guess", body = GuessResponse, example = json!({"result":["hit","hit","hit","hit","hit"],"won":true})),
    (status = 404, description = "Game session expired", body = GuessError, example = json!({"message":"Game session expired"})),
))]
async fn guess(
    State(service): State<Arc<Service>>,
    Json(payload): Json<GuessRequest>,
) -> impl IntoResponse {
    let mut cache = service.cache.lock();
    let target = if let Some(w) = cache.get(&payload.id) {
        w.to_lowercase()
    } else {
        debug!("game session expired: id={}", payload.id);
        return (
            StatusCode::NOT_FOUND,
            Json(GuessError {
                message: "Game session expired",
            }),
        )
            .into_response();
    };

    let guess = payload.guess.to_lowercase();
    let mut results = vec!["miss"; 5];
    let mut target_counts = HashMap::new();

    for c in target.chars() {
        *target_counts.entry(c).or_insert(0) += 1;
    }

    let target_chars: Vec<char> = target.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();

    for i in 0..5 {
        if guess_chars[i] == target_chars[i] {
            results[i] = "hit";
            *target_counts.get_mut(&guess_chars[i]).unwrap() -= 1;
        }
    }

    for i in 0..5 {
        if results[i] != "hit" {
            if let Some(count) = target_counts.get_mut(&guess_chars[i]) {
                if *count > 0 {
                    results[i] = "partial";
                    *count -= 1;
                }
            }
        }
    }

    let won = guess == target;

    debug!(
        "guess made: id={} guess={} target={} result={:?} won={}",
        payload.id, payload.guess, target, results, won
    );

    (
        StatusCode::OK,
        Json(GuessResponse {
            result: results,
            won,
        }),
    )
        .into_response()
}

/// Health check
#[utoipa::path(get, path = "/api/v1/health", responses(
    (status = 200, description = "Health check successful", body = String, example = "2026-03-31T11:34:49.810125500-03:00")
))]
async fn health() -> String {
    let now = Local::now().to_rfc3339();
    debug!("health check at {now}");

    now
}

#[derive(Serialize, ToSchema)]
struct NewResponse<'a> {
    /// Game ID
    game_id: &'a str,
}

#[derive(Serialize, ToSchema)]
struct NewError<'a> {
    /// Error message
    message: &'a str,
}

/// Start a new game
#[utoipa::path(get, path = "/api/v1/new",responses(
    (status = 200, description = "Returns a new game ID", body = NewResponse, example = json!({"game_id":"lprcp4yara"})),
    (status = 500, description = "No words available", body = NewError, example = json!({"message":"No words available"})),
))]
async fn new(State(service): State<Arc<Service>>) -> impl IntoResponse {
    let id = slug();
    let mut rng = rand::rng();
    if let Some(word) = service.words.choose(&mut rng) {
        service.cache.lock().put(id.clone(), word.to_string());
        debug!("new game created: id={id} word={word}");

        (StatusCode::OK, Json(NewResponse { game_id: &id })).into_response()
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(NewError {
                message: "No words available",
            }),
        )
            .into_response()
    }
}
