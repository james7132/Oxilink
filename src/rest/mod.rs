mod audio_loading;
mod info;
mod websocket;
mod utils;

use actix_web::web;

pub(super) fn config(cfg: &mut web::ServiceConfig) {
    audio_loading::scoped_config(cfg);
    info::scoped_config(cfg);
    websocket::scoped_config(cfg);
}
