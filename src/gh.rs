use octocrab::Octocrab;

pub struct Release<'a> {
    pub token: &'a str,
    pub user: &'a str,
    pub repository: &'a str,
    pub tag: &'a str,
    pub description: &'a str,
}

impl<'a> Release<'a> {
    pub async fn create(&self) -> octocrab::Result<octocrab::models::repos::Release> {
        let name = format!("Release {}", self.tag);
        Octocrab::builder()
            .personal_token(self.token.to_owned())
            .build()?
            .repos(self.user, self.repository)
            .releases()
            .create(self.tag)
            .name(&name)
            .body(self.description)
            .send()
            .await
    }
}
