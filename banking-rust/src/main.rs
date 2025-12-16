use postgres::{Client, NoTls};
pub mod gen_number;


fn main() {
    let mut client = Client::connect(
        "host=localhost user=postgres password=YOUR_PASSWORD dbname=testdb",
        NoTls,
    ).expect("failed to connect");

    let row = client.query_one("SELECT 42;", &[])
        .expect("query failed");

    let num: i32 = row.get(0);
    println!("Result: {}", num);
}
