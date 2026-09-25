// Recompiles this crate whenever a file under migrations/ changes, so sqlx::migrate!() embeds
// every migration script, including one added since the last build.
fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
