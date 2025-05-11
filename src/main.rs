use actix_files as files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use askama::Template;
use serde::Deserialize;

mod config;
mod gemini;
use config::Settings;
use gemini::GeminiClient;

#[derive(Template)]
#[template(path = "dynamic.html")]
struct DynamicTemplate {
    title: String,
    message: String,
    show_content: bool,
    content: String,
}

#[derive(Deserialize)]
struct UpdateContent {
    content: String,
}

#[derive(Deserialize)]
struct GeminiRequest {
    prompt: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let template = DynamicTemplate {
        title: "Dynamify Demo".to_string(),
        message: "Welcome to Dynamify Demo".to_string(),
        show_content: true,
        content: r#"
            <h2 class="text-2xl font-semibold text-gray-700 mb-4">Dynamify Demo</h2>
            <p class="text-gray-600 mb-4">「イメージ」を入力すると、そのイメージに添った形でコンテンツを提供します</p>
            <form id="geminiForm" class="space-y-4">
                <div>
                    <label for="prompt" class="block text-sm font-medium text-gray-700">Prompt</label>
                    <textarea id="prompt" placeholder="一陣の風が吹き、草の波が揺れる" name="prompt" rows="4" class="p-4 mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"></textarea>
                </div>
                <button
                 type="submit"
                 class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                >
                    Generate
                </button>
            </form>
            <div id="result" class="mt-4">あのイーハトーヴォのすきとおった風、夏でも底に冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。</div>
        "#.to_string(),
    };

    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(_) => HttpResponse::InternalServerError().body("Template error"),
    }
}

#[post("/generate")]
async fn generate_text(
    data: web::Json<GeminiRequest>,
    gemini_client: web::Data<GeminiClient>,
) -> impl Responder {
    match gemini_client.generate_text(&data.prompt).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // アプリケーション起動時に設定を読み込む
    let settings = Settings::new().expect("Failed to load settings");
    println!("Environment: {}", settings.environment);

    // Gemini APIクライアントの初期化
    let gemini_client = web::Data::new(GeminiClient::new(settings.api_key));

    HttpServer::new(move || {
        App::new()
            .app_data(gemini_client.clone())
            .service(index)
            .service(generate_text)
            .service(files::Files::new("/static", "static").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
