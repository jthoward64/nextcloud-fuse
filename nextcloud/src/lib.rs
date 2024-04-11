mod client;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut config = include_str!("../../.config").split_whitespace();
        let username = config.next().unwrap();
        let password = config.next().unwrap();
        let base_url = config.next().unwrap();
        let dav_path = config.next().unwrap();

        let provider = client::Nextcloud::new(
            base_url.to_string(),
            dav_path.to_string(),
            username.to_string(),
            password.to_string(),
        );

        println!("{:?}", provider);
    }
}
