pub trait Sampler<'a> {
    type SamplingType;
    type Error: crate::traits::error::Error<'a>;

    fn sample(
        items: &[Self::SamplingType],
        sample_size: u64,
    ) -> Result<&[Self::SamplingType], Self::Error>;
}
