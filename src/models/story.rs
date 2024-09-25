use super::StoryStorage;

pub struct StoryModel {}

impl StoryStorage for StoryModel {
    fn get_by_id(id: i64) -> anyhow::Result<super::Story> {
        todo!()
    }

    fn list(list_req: &super::ListStory) -> anyhow::Result<Vec<super::Story>> {
        todo!()
    }

    fn create(story: &super::WriteableStory) -> anyhow::Result<super::Story> {
        todo!()
    }

    fn update(story: &super::WriteableStory) -> anyhow::Result<super::Story> {
        todo!()
    }

    fn delete(story_id: i64) -> anyhow::Result<()> {
        todo!()
    }
}
