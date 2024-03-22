pub struct SampleRegion {
    pub pos: IVec2,
    pub size: IVec2,
}

const BLOOD_BALL: SampleRegion = SampleRegion {
    pos: IVec2 { x: 63, y: 38 },
    size: IVec2 { x: 16, y: 17 },
};

pub fn get_sample_region(particle_type: ParticleType, counter: u32) -> &'static SampleRegion {
    match particle_type {
        ParticleType::Explosion => match counter {
            6..=7 => &EXPLOSION_FRAME_1,
            4..=5 => &EXPLOSION_FRAME_2,
            2..=3 => &EXPLOSION_FRAME_3,
            0..=1 => &EXPLOSION_FRAME_4,
            _ => &BLOOD_BALL,
        },
        ParticleType::Smoke => &SMOKE,
        ParticleType::BloodBall => &BLOOD_BALL,
    }
}
