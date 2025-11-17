use axum::{
    Router,
    http::{self, StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use tower_http::services::ServeDir;

mod api;
mod constants;
mod data;
mod metadata;
mod templates;

use api::{guestbook, jellyfin};
use data::badges::MIMI_BADGE;
use metadata::{ChangeFreq, RobotsTXT, Sitemap, Uri};
use tera::Context;

use crate::data::badges::{COOL_SITES, FRIENDS};

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&pool).await.unwrap();

    for var in constants::ENV_VARS {
        unsafe {
            std::env::set_var(var, secrets.get(var).unwrap());
        }
    }

    let routes = build_routes(pool);

    api::send_notification("server up".to_string()).await;

    Ok(routes.into())
}

fn build_routes(pool: sqlx::PgPool) -> Router {
    let uris = &[
        Uri::new("/", "home", true, Some(ChangeFreq::Monthly), Some(1.0)),
        Uri::new(
            "/donate",
            "donate",
            true,
            Some(ChangeFreq::Yearly),
            Some(0.6),
        ),
        Uri::new(
            "/contact",
            "contact",
            true,
            Some(ChangeFreq::Yearly),
            Some(0.8),
        ),
        Uri::new(
            "/guestbook",
            "guestbook",
            true,
            Some(ChangeFreq::Weekly),
            Some(0.6),
        ),
        Uri::new(
            "/projects",
            "construction",
            true,
            Some(ChangeFreq::Monthly),
            Some(0.8),
        ),
        Uri::new(
            "/badges",
            "badges",
            true,
            Some(ChangeFreq::Monthly),
            Some(0.6),
        ),
    ];

    let sitemap = Sitemap::from_uris(uris).to_string();
    let robots = RobotsTXT::from_uris(uris).to_string();

    let api_router: Router = Router::new()
        .route(
            "/guestbook",
            get(guestbook::get_all_handler).post(guestbook::add_handler),
        )
        .route("/jellyfin/start", post(jellyfin::start_handler))
        .route("/jellyfin/stop", post(jellyfin::stop_handler))
        .route("/jellyfin", get(jellyfin::get_handler))
        .with_state(pool);

    let mut redirect_router: Router = Router::new();
    for (uri, loc) in constants::INT_REDIRECTS.iter() {
        redirect_router = redirect_router.route(uri, get(|| redirect(loc)));
    }
    for (uri, loc) in constants::EXT_REDIRECTS.iter() {
        redirect_router = redirect_router.route(uri, get(|| redirect_temp(loc)));
    }

    let mut router = Router::new()
        .route(
            "/sitemap.xml",
            get(([(header::CONTENT_TYPE, "application/xml")], sitemap)),
        )
        .route(
            "/robots.txt",
            get(([(header::CONTENT_TYPE, "text/plain")], robots)),
        )
        .merge(redirect_router)
        .nest("/api", api_router)
        .nest_service("/img", ServeDir::new("img"))
        .nest_service("/static", ServeDir::new("static"))
        .route("/healthz", get("alive :3"))
        .fallback(fallback_handler);

    let mut ctx = Context::new();
    ctx.insert("host", constants::HOST);
    ctx.insert("main_host", constants::MAIN_HOST);
    ctx.insert("git_url", constants::GIT_URL);
    ctx.insert("mimi_badge", &MIMI_BADGE);
    ctx.insert("uris", uris);

    for uri in uris {
        ctx.insert(
            "canonical",
            format!("{}{}", constants::HOST, uri.uri).trim_end_matches('/'),
        );

        if uri.template == "badges" {
            ctx.insert("friend_badges", &FRIENDS);
            ctx.insert("cool_sites_badges", &COOL_SITES);
        }

        router = router.route(uri.uri, get(render(uri.template, &ctx)));
    }

    router
}

async fn redirect(location: &str) -> Response {
    (
        StatusCode::PERMANENT_REDIRECT,
        [(header::LOCATION, location)],
        "redirecting...",
    )
        .into_response()
}

async fn redirect_temp(location: &str) -> Response {
    (
        StatusCode::TEMPORARY_REDIRECT,
        [(header::LOCATION, location)],
        "redirecting...",
    )
        .into_response()
}

fn render(page_name: &str, ctx: &Context) -> Html<String> {
    let path = format!("pages/{page_name}.tera");

    match templates::TEMPLATES.render(&path, ctx) {
        Ok(html) => Html(html),
        Err(err) => {
            eprintln!("failed to render template {page_name}: {err}");
            Html(format!("template error: {err} :c"))
        }
    }
}

async fn fallback_handler(uri: http::Uri) -> Response {
    let path = uri.path();

    if path != "/" && path.ends_with('/') {
        let new_path = path.trim_end_matches('/');
        let new_loc = format!("{}{}", constants::HOST, new_path);

        return redirect(&new_loc).await;
    }

    (StatusCode::NOT_FOUND, "not found :c").into_response()
}
