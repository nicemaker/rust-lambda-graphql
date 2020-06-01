pub mod skills {
    use crate::s3;
    use crate::schema::Skill;
    use serde_json;

    pub async fn write(skills: &Vec<Skill>) {
        let json = serde_json::to_string(skills).unwrap();
        s3::put_object(String::from("skills/latest.json"), json).await;
    }

    pub async fn read() -> Vec<Skill> {
        let body = s3::get_object(String::from("skills/latest.json")).await;
        let skills: Vec<Skill> = serde_json::from_str(&body).unwrap();
        return skills;
    }

    pub async fn writeOne(skill: Skill) -> Vec<Skill> {
        let mut skills = read().await;
        skills.retain(|x| x.id != skill.id);
        skills.push(skill);
        write(&skills).await;
        return skills;
    }

    pub async fn deleteOne(id: String) -> Vec<Skill> {
        let mut skills = read().await;
        skills.retain(|x| x.id != id);
        write(&skills).await;
        return skills;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Skill;

    #[tokio::test]
    async fn skill_write() {
        skills::write(&vec![Skill {
            id: String::from("123"),
            name: String::from("123"),
            group: String::from("123"),
            level: 2,
            description: None,
        }])
        .await;
    }

    #[tokio::test]
    async fn skill_read() {
        let skills = skills::read().await;
        assert_eq!(
            vec![Skill {
                id: String::from("123"),
                name: String::from("123"),
                group: String::from("123"),
                level: 2,
                description: None,
            }],
            skills
        )
    }
}
