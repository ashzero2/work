fn main() {
    match work_dashboard::storage::db::open() {
        Ok(_) => println!(
            "Database ready at {:?}",
            work_dashboard::storage::paths::db_path()
        ),
        Err(err) => eprintln!("Failed to open database: {err}"),
    }
}
