use crate::SubsystemSlot;

pub struct ModernShipGeometry;

impl ModernShipGeometry {
    pub const RADIUS: f32 = 14.0;
    pub const SPEED_LIMIT: f32 = 3.4;
    pub const SCANNER_MAXIMUM_ANGLE_OFFSET: f32 = 22.5;
    pub const INTERCEPTOR_MAXIMUM_ANGLE_OFFSET: f32 = 45.0;

    pub const ENGINE_SLOTS: [SubsystemSlot; 8] = [
        SubsystemSlot::ModernEngineN,
        SubsystemSlot::ModernEngineNE,
        SubsystemSlot::ModernEngineE,
        SubsystemSlot::ModernEngineSE,
        SubsystemSlot::ModernEngineS,
        SubsystemSlot::ModernEngineSW,
        SubsystemSlot::ModernEngineW,
        SubsystemSlot::ModernEngineNW,
    ];

    pub const SCANNER_SLOTS: [SubsystemSlot; 8] = [
        SubsystemSlot::ModernScannerN,
        SubsystemSlot::ModernScannerNE,
        SubsystemSlot::ModernScannerE,
        SubsystemSlot::ModernScannerSE,
        SubsystemSlot::ModernScannerS,
        SubsystemSlot::ModernScannerSW,
        SubsystemSlot::ModernScannerW,
        SubsystemSlot::ModernScannerNW,
    ];

    pub const SHOT_LAUNCHER_SLOTS: [SubsystemSlot; 8] = [
        SubsystemSlot::StaticShotLauncherN,
        SubsystemSlot::StaticShotLauncherNE,
        SubsystemSlot::StaticShotLauncherE,
        SubsystemSlot::StaticShotLauncherSE,
        SubsystemSlot::StaticShotLauncherS,
        SubsystemSlot::StaticShotLauncherSW,
        SubsystemSlot::StaticShotLauncherW,
        SubsystemSlot::StaticShotLauncherNW,
    ];

    pub const SHOT_MAGAZINE_SLOTS: [SubsystemSlot; 8] = [
        SubsystemSlot::StaticShotMagazineN,
        SubsystemSlot::StaticShotMagazineNE,
        SubsystemSlot::StaticShotMagazineE,
        SubsystemSlot::StaticShotMagazineSE,
        SubsystemSlot::StaticShotMagazineS,
        SubsystemSlot::StaticShotMagazineSW,
        SubsystemSlot::StaticShotMagazineW,
        SubsystemSlot::StaticShotMagazineNW,
    ];

    pub const SHOT_FABRICATOR_SLOTS: [SubsystemSlot; 8] = [
        SubsystemSlot::StaticShotFabricatorN,
        SubsystemSlot::StaticShotFabricatorNE,
        SubsystemSlot::StaticShotFabricatorE,
        SubsystemSlot::StaticShotFabricatorSE,
        SubsystemSlot::StaticShotFabricatorS,
        SubsystemSlot::StaticShotFabricatorSW,
        SubsystemSlot::StaticShotFabricatorW,
        SubsystemSlot::StaticShotFabricatorNW,
    ];

    pub const RAILGUN_SLOTS: [SubsystemSlot; 8] = [
        SubsystemSlot::ModernRailgunN,
        SubsystemSlot::ModernRailgunNE,
        SubsystemSlot::ModernRailgunE,
        SubsystemSlot::ModernRailgunSE,
        SubsystemSlot::ModernRailgunS,
        SubsystemSlot::ModernRailgunSW,
        SubsystemSlot::ModernRailgunW,
        SubsystemSlot::ModernRailgunNW,
    ];

    pub const fn try_get_local_angle(slot: SubsystemSlot) -> Option<f32> {
        Some(match slot {
            SubsystemSlot::ModernEngineN
            | SubsystemSlot::ModernScannerN
            | SubsystemSlot::StaticShotLauncherN
            | SubsystemSlot::StaticShotMagazineN
            | SubsystemSlot::StaticShotFabricatorN
            | SubsystemSlot::ModernRailgunN => 0.0,
            SubsystemSlot::ModernEngineNE
            | SubsystemSlot::ModernScannerNE
            | SubsystemSlot::StaticShotLauncherNE
            | SubsystemSlot::StaticShotMagazineNE
            | SubsystemSlot::StaticShotFabricatorNE
            | SubsystemSlot::ModernRailgunNE => 315.0,
            SubsystemSlot::ModernEngineE
            | SubsystemSlot::ModernScannerE
            | SubsystemSlot::StaticShotLauncherE
            | SubsystemSlot::StaticShotMagazineE
            | SubsystemSlot::StaticShotFabricatorE
            | SubsystemSlot::StaticInterceptorLauncherE
            | SubsystemSlot::StaticInterceptorMagazineE
            | SubsystemSlot::StaticInterceptorFabricatorE
            | SubsystemSlot::ModernRailgunE => 270.0,
            SubsystemSlot::ModernEngineSE
            | SubsystemSlot::ModernScannerSE
            | SubsystemSlot::StaticShotLauncherSE
            | SubsystemSlot::StaticShotMagazineSE
            | SubsystemSlot::StaticShotFabricatorSE
            | SubsystemSlot::ModernRailgunSE => 225.0,
            SubsystemSlot::ModernEngineS
            | SubsystemSlot::ModernScannerS
            | SubsystemSlot::StaticShotLauncherS
            | SubsystemSlot::StaticShotMagazineS
            | SubsystemSlot::StaticShotFabricatorS
            | SubsystemSlot::ModernRailgunS => 135.0,
            SubsystemSlot::ModernEngineW
            | SubsystemSlot::ModernScannerW
            | SubsystemSlot::StaticShotLauncherW
            | SubsystemSlot::StaticShotMagazineW
            | SubsystemSlot::StaticShotFabricatorW
            | SubsystemSlot::StaticInterceptorLauncherW
            | SubsystemSlot::StaticInterceptorMagazineW
            | SubsystemSlot::StaticInterceptorFabricatorW
            | SubsystemSlot::ModernRailgunW => 90.0,
            SubsystemSlot::ModernEngineNW
            | SubsystemSlot::ModernScannerNW
            | SubsystemSlot::StaticShotLauncherNW
            | SubsystemSlot::StaticShotMagazineNW
            | SubsystemSlot::StaticShotFabricatorNW
            | SubsystemSlot::ModernRailgunNW => 45.0,
            _ => return None,
        })
    }

    pub fn normalize_angle(angle: f32) -> f32 {
        let angle = angle % 360.0;
        if angle < 0.0 {
            angle + 360.0
        } else {
            angle
        }
    }

    pub fn normalized_signed_angle(angle: f32) -> f32 {
        let angle = Self::normalize_angle(angle + 180.0) - 180.0;
        if angle <= -180.0 {
            angle + 360.0
        } else {
            angle
        }
    }

    pub fn get_absolute_angle(ship_angle: f32, slot: SubsystemSlot) -> f32 {
        let local_angle = Self::try_get_local_angle(slot)
            .expect("No modern-ship angle mapping found for the slot");
        Self::normalize_angle(ship_angle + local_angle)
    }
}
