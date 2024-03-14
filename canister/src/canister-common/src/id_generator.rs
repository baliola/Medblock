use crate::random::{ CallError, RandomSource };
use crate::common::UUID_MAX_SOURCE_LEN;
pub struct IdGenerator<Source: RandomSource>(Source);

impl<Source: RandomSource> IdGenerator<Source> {
    pub fn new(source: Source) -> Self {
        Self(source)
    }

    pub async fn generate_id(&self) -> Result<crate::common::Id, CallError> {
        let random_bytes = self.0.get_random_bytes::<UUID_MAX_SOURCE_LEN>().await?;
        Ok(crate::common::Id::new(&random_bytes))
    }
}

#[cfg(test)]
mod test {
    use std::future;

    use super::*;

    #[tokio::test]
    async fn test_generate_id() {
        struct MockRandomSource;

        impl RandomSource for MockRandomSource {
            async fn get_random_bytes<const N: usize>(&self) -> Result<[u8; N], CallError> {
                let mut bytes = [0; N];
                bytes.fill(0);
                Ok(bytes)
            }
        }

        let generator = IdGenerator::new(MockRandomSource);
        let id = generator.generate_id().await.unwrap();
        assert_eq!(id.to_string().len(), 36);
    }
}
