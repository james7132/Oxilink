use actix_web::{web, get, post, HttpRequest, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
struct TrackInfo {
    title: String,
    author: String,
    length: u64,
    identifier: String,
    #[serde(rename = "isStream")]
    is_stream: bool,
    #[serde(rename = "isSeekable")]
    is_seekable: bool,
    uri: String,
    position: u64
}

#[derive(Deserialize)]
struct TrackQuery {
    track: String
}

#[get("/loadtracks")]
async fn load_tracks(req: HttpRequest) -> impl Responder {
    let tracks: Vec<TrackInfo> = Vec::new();
    HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .json(tracks)
}

#[get("/decodetrack")]
async fn decode_track(enc: web::Query<TrackQuery>) -> impl Responder {
    //let track = decode_single_track(enc.track)?;
    HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .json("{}")
}

#[post("/decodetracks")]
async fn decode_tracks(encoded: web::Json<Vec<String>>) -> impl Responder {
    let decoded: Vec<TrackInfo> = Vec::new();
    for enc in encoded.iter() {
        //decoded.push(decode_single_track(enc)?);
    }
    HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .json(decoded)
}

//fn decode_single_track(encoded: impl Into<String>) -> Result<TrackInfo, ()> {
    //Err(());
//}

pub(super) fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(load_tracks);
    cfg.service(decode_track);
    cfg.service(decode_tracks);
}
