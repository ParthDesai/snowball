pub trait Sampler {
    type SamplingType;
    type Error: crate::traits::error::Error;

    fn sample(
        items: Vec<Self::SamplingType>,
        sample_size: u64,
    ) -> Result<Vec<Self::SamplingType>, Self::Error>;
}
