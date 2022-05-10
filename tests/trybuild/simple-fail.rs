use actix_web_codegen_const_routes::*;

#[get("/one", other)]
async fn one() -> String {
    "Hello World!".to_owned()
}

#[post(/two)]
async fn two() -> String {
    "Hello World!".to_owned()
}

static PATCH_PATH: &str = "/three";

#[patch(PATCH_PATH)]
async fn three() -> String {
    "Hello World!".to_owned()
}

#[delete("/four", "/five")]
async fn four() -> String {
    "Hello World!".to_owned()
}

#[delete("/five", method="GET")]
async fn five() -> String {
    "Hello World!".to_owned()
}

const SIX: &str = "/six";

#[get("/six", path="SIX")]
async fn six() -> String {
    "Hello World!".to_owned()
}

fn main() {}
