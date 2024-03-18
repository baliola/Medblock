use crate::random::{ CallError, RandomSource };
use crate::common::UUID_MAX_SOURCE_LEN;
pub struct IdGenerator<Source: RandomSource>(Source);

impl<Source: RandomSource> IdGenerator<Source> {
    pub fn new(source: Source) -> Self {
        Self(source)
    }

    pub async fn reseed_randomness(&mut self) -> Result<(), CallError> {
        self.0.reseed().await;
        Ok(())
    }

    pub fn generate_id(&mut self) -> crate::common::Id {
        let random_bytes = self.0.get_random_bytes();

        let mut bytes = [0; UUID_MAX_SOURCE_LEN];
        bytes.copy_from_slice(&random_bytes[0..UUID_MAX_SOURCE_LEN]);

        crate::common::Id::new(&bytes)
    }
}

#[cfg(test)]
mod test {
    use std::{ collections::{ HashSet } };

    use crate::random::CanisterRandomSource;

    use super::*;

    #[test]
    fn test_generate_id() {
        let mut map = HashSet::new();

        /// 10 million iterations, should be enough to test the randomness of the id generator

        let source = CanisterRandomSource::new_with_seed(10000);
        let mut generator = IdGenerator::new(source);

        for i in 0..10_000_000 {
            println!("running {} iteration", i);
            let id = generator.generate_id();

            map.insert(id);
        }
    }
}
