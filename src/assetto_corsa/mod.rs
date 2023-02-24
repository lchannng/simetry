pub use crate::assetto_corsa::data::{
    FlagType, Graphics, Penalty, Physics, SessionType, StaticData, Status,
};
use crate::assetto_corsa::shared_memory_data::{PageFileGraphics, PageFilePhysics, PageFileStatic};
use crate::{BasicTelemetry, MomentImpl, RacingFlags};
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::f64::{AngularVelocity, Velocity};
use uom::si::velocity::kilometer_per_hour;

mod conversions;
mod data;
mod shared_memory_data;
pub(crate) mod util;

#[derive(Clone, Debug, Default)]
pub struct AssettoCorsaApiVersion;

impl util::AcApiVersion for AssettoCorsaApiVersion {
    const MAJOR_MIN: u16 = 1;
    const MAJOR_MAX: u16 = 1;
    const MINOR_MIN: u16 = 0;
    const MINOR_MAX: u16 = 7;
    type PageStatic = PageFileStatic;
    type DataStatic = StaticData;
    type PagePhysics = PageFilePhysics;
    type DataPhysics = Physics;
    type PageGraphics = PageFileGraphics;
    type DataGraphics = Graphics;
}

impl util::WithPacketId for PageFilePhysics {
    fn packet_id(&self) -> i32 {
        self.packet_id
    }
}

impl util::WithPacketId for Physics {
    fn packet_id(&self) -> i32 {
        self.packet_id
    }
}

impl util::WithPacketId for PageFileGraphics {
    fn packet_id(&self) -> i32 {
        self.packet_id
    }
}

impl util::WithPacketId for Graphics {
    fn packet_id(&self) -> i32 {
        self.packet_id
    }
}

pub type Client = util::SharedMemoryClient<AssettoCorsaApiVersion>;
pub type SimState = util::SimState<AssettoCorsaApiVersion>;

impl MomentImpl for SimState {
    fn car_left(&self) -> bool {
        false
    }

    fn car_right(&self) -> bool {
        false
    }

    fn basic_telemetry(&self) -> Option<BasicTelemetry> {
        Some(BasicTelemetry {
            gear: (self.physics.gear - 1) as i8,
            speed: Velocity::new::<kilometer_per_hour>(self.physics.speed_kmh as f64),
            engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                self.physics.rpm as f64,
            ),
            max_engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                self.static_data.max_rpm as f64,
            ),
            pit_limiter_engaged: self.physics.pit_limiter_on != 0,
            in_pit_lane: self.graphics.is_in_pit_lane != 0,
        })
    }

    fn shift_point(&self) -> Option<AngularVelocity> {
        None
    }

    fn flags(&self) -> RacingFlags {
        let mut flags = RacingFlags::default();
        match self.graphics.flag {
            FlagType::None => {}
            FlagType::Blue => flags.blue = true,
            FlagType::Yellow => flags.yellow = true,
            FlagType::Black => flags.black = true,
            FlagType::White => flags.white = true,
            FlagType::Checkered => flags.checkered = true,
            FlagType::Penalty => flags.black = true,
            FlagType::Green => flags.green = true,
            FlagType::Orange => flags.meatball = true,
        }
        flags
    }
}
