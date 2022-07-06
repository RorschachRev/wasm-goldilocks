extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

use crate::curve::twedwards::extended::ExtendedPoint;
use std::fmt;

#[wasm_bindgen]
pub struct RistrettoPoint(ExtendedPoint);

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct CompressedRistretto([u8; 56]);

impl fmt::Debug for CompressedRistretto {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0[..].fmt(formatter)
    }
}

#[wasm_bindgen]
impl CompressedRistretto {
    pub fn as_bytes(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(&self.0[..])
    }

    pub fn ct_eq(&self, other: &CompressedRistretto) -> bool {
        self.as_bytes().to_vec() == other.as_bytes().to_vec()
    }

    fn eq(&self, other: &CompressedRistretto) -> bool {
        self.ct_eq(other).into()
    }
}

#[wasm_bindgen]
impl RistrettoPoint {
    pub fn identity() -> RistrettoPoint {
        RistrettoPoint(ExtendedPoint::identity())
    }

    pub fn equals(&self, other: &RistrettoPoint) -> bool {
        let XY = self.0.X * other.0.Y;
        let YX = self.0.Y * other.0.X;
        XY == YX
    }

    pub fn encode(&self) -> CompressedRistretto {
        todo!()
    }
}

#[wasm_bindgen]
impl CompressedRistretto {
    pub fn identity() -> CompressedRistretto {
        CompressedRistretto([0; 56])
    }

    pub fn decode(&self) -> Option<RistrettoPoint> {
        todo!()
    }
}
