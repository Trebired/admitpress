use super::Ticket;
use serde::{Deserialize, Serialize};

pub const FALLBACK_PAGE_WIDTH_MM: f64 = 50.0;
pub const FALLBACK_PAGE_HEIGHT_MM: f64 = 70.0;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterProfile {
    pub queue: String,
    pub width_mm: f64,
    pub height_mm: f64,
    pub offset_x_mm: f64,
    pub offset_y_mm: f64,
    pub scale_x_percent: f64,
    pub scale_y_percent: f64,
    pub recorded_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct PrintCalibration {
    pub offset_x_mm: f64,
    pub offset_y_mm: f64,
    pub scale_x_percent: f64,
    pub scale_y_percent: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct PageSize {
    pub width_mm: f64,
    pub height_mm: f64,
}

impl Default for PrintCalibration {
    fn default() -> Self {
        Self::none()
    }
}

impl PrintCalibration {
    pub const fn none() -> Self {
        Self {
            offset_x_mm: 0.0,
            offset_y_mm: 0.0,
            scale_x_percent: 100.0,
            scale_y_percent: 100.0,
        }
    }

    pub fn normalized(self) -> Self {
        Self {
            offset_x_mm: clamp_offset_mm(self.offset_x_mm),
            offset_y_mm: clamp_offset_mm(self.offset_y_mm),
            scale_x_percent: clamp_scale_percent(self.scale_x_percent),
            scale_y_percent: clamp_scale_percent(self.scale_y_percent),
        }
    }

    pub fn is_identity(&self) -> bool {
        let normalized = self.normalized();
        normalized.offset_x_mm.abs() < f64::EPSILON
        && normalized.offset_y_mm.abs() < f64::EPSILON
        && (normalized.scale_x_percent - 100.0).abs() < f64::EPSILON
        && (normalized.scale_y_percent - 100.0).abs() < f64::EPSILON
    }

    pub fn scale_x(&self) -> f64 {
        self.normalized().scale_x_percent / 100.0
    }

    pub fn scale_y(&self) -> f64 {
        self.normalized().scale_y_percent / 100.0
    }
}

impl PageSize {
    pub const fn fallback() -> Self {
        Self {
            width_mm: FALLBACK_PAGE_WIDTH_MM,
            height_mm: FALLBACK_PAGE_HEIGHT_MM,
        }
    }
}

impl Ticket {
    pub fn page_size(&self) -> PageSize {
        PageSize {
            width_mm: self.width_mm,
            height_mm: self.height_mm,
        }
    }
}

impl PrinterProfile {
    pub fn calibration(&self) -> PrintCalibration {
        PrintCalibration {
            offset_x_mm: self.offset_x_mm,
            offset_y_mm: self.offset_y_mm,
            scale_x_percent: self.scale_x_percent,
            scale_y_percent: self.scale_y_percent,
        }
        .normalized()
    }

}

pub fn clamp_offset_mm(value: f64) -> f64 {
    if value.is_finite() {
        value.clamp(-50.0, 50.0)
    } else {
        0.0
    }
}

pub fn clamp_scale_percent(value: f64) -> f64 {
    if value.is_finite() {
        value.clamp(80.0, 120.0)
    } else {
        100.0
    }
}
