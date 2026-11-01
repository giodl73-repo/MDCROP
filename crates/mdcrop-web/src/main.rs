use std::{env, fs::File, io::Read};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = env::args()
        .nth(1)
        .ok_or("usage: mdcrop-explore REQUEST.json | --fixture | --matrix")?;
    if arg == "--fixture" {
        println!("{}", mdcrop_web::fixture_json());
    } else if arg == "--matrix" {
        let mut rows = Vec::new();
        for seed in 0..4 {
            for budget in 1..=256 {
                let request =
                    serde_json::json!({"seed":seed.to_string(),"budget":budget.to_string()})
                        .to_string();
                rows.push(serde_json::json!({"request":request,"canonical":mdcrop_web::explore(&request)}));
            }
        }
        println!("{}", serde_json::to_string(&rows)?);
    } else {
        let mut input = String::new();
        File::open(arg)?
            .take(mdcrop_web::MAX_REQUEST_BYTES as u64 + 1)
            .read_to_string(&mut input)?;
        println!("{}", mdcrop_web::explore(&input));
    }
    Ok(())
}
