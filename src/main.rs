use nih_plug::prelude::*;

use test_vst_rust::OscillatorTest;
fn main() {
    nih_export_standalone::<OscillatorTest>();
}