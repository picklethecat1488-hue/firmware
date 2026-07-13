use controller::system_feature::{FeatureList, SystemFeature};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

struct MockFeatureA;
impl SystemFeature<CriticalSectionRawMutex, 4> for MockFeatureA {
    const HAS_THERMAL_THRESHOLDS: bool = true;
    fn thermal_overheating_temp_threshold(&self) -> i32 {
        40000
    }
    fn thermal_critical_temp_threshold(&self) -> i32 {
        55000
    }
    fn default_boot_trap_mask(&self) -> u32 {
        0x01
    }
}

struct MockFeatureB;
impl SystemFeature<CriticalSectionRawMutex, 4> for MockFeatureB {
    fn default_boot_trap_mask(&self) -> u32 {
        0x02
    }
}

#[test]
fn test_feature_list_single_thermal() {
    let features = (MockFeatureA, MockFeatureB);
    assert_eq!(features.thermal_overheating_temp_threshold(), 40000);
    assert_eq!(features.thermal_critical_temp_threshold(), 55000);
}

#[test]
fn test_feature_list_no_thermal() {
    let features = (MockFeatureB, MockFeatureB);
    assert_eq!(features.thermal_overheating_temp_threshold(), 45000);
    assert_eq!(features.thermal_critical_temp_threshold(), 60000);
}

#[test]
fn test_feature_list_combines_boot_traps() {
    let features = (MockFeatureA, MockFeatureB);
    assert_eq!(features.default_boot_trap_mask(), 0x03);
}
