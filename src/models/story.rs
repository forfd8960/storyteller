use super::{ListStory, Story, StoryStorage, WriteableStory};

pub struct StoryModel {}

impl StoryStorage for StoryModel {
    fn get_by_id(id: i64) -> anyhow::Result<Story> {
        todo!()
    }

    fn list(list_req: &ListStory) -> anyhow::Result<Vec<Story>> {
        todo!()
    }

    fn create(story: &WriteableStory) -> anyhow::Result<Story> {
        todo!()
    }

    fn update(story: &Story) -> anyhow::Result<Story> {
        todo!()
    }

    fn delete(story_id: i64) -> anyhow::Result<()> {
        todo!()
    }
}
