mod oscillators;
pub mod parametric_equation;

use crate::oscillators::ParametricOscillator;

use nih_plug::prelude::*;
use std::sync::Arc;
use nih_plug_egui::{create_egui_editor, egui, widgets, EguiState};
use crate::parametric_equation::{EquationA, ParametricEquation};

/// A test tone generator that can either generate a sine wave based on the plugin's parameters or
/// based on the current MIDI input.
pub struct Paramic {
    params: Arc<ParamicParams>,

    sample_rate: f32,

    /// The underlying oscillator
    oscillator: oscillators::ParametricOscillatorA,

    /// The MIDI note ID of the active note, if triggered by MIDI.
    midi_note_id: u8,
    /// The frequency if the active note, if triggered by MIDI.
    midi_note_freq: f32,
    /// A simple attack and release envelope to avoid clicks. Controlled through velocity and
    /// aftertouch.
    ///
    /// Smoothing is built into the parameters, but you can also use them manually if you need to
    /// smooth soemthing that isn't a parameter.
    midi_note_gain: Smoother<f32>,
}

#[derive(Params)]
struct ParamicParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,

    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "freq"]
    pub frequency: FloatParam,

    #[id = "usemid"]
    pub use_midi: BoolParam,

    #[id = "a"]
    pub a: IntParam,

    #[id = "b"]
    pub b: IntParam,

    #[id = "c"]
    pub c: IntParam,

    #[id = "d"]
    pub d: IntParam,

    #[id = "j"]
    pub j: IntParam,

    #[id = "k"]
    pub k: IntParam,
}

impl Default for Paramic {

    fn default() -> Self {
        Self {
            params: Arc::new(ParamicParams::default()),

            sample_rate: 1.0,

            oscillator: oscillators::ParametricOscillatorA::new(
                1.0, EquationA {
                    a: ParamicParams::default().a.value(),
                    b: ParamicParams::default().b.value(),
                    c: ParamicParams::default().c.value(),
                    d: ParamicParams::default().d.value(),
                    j: ParamicParams::default().j.value(),
                    k: ParamicParams::default().k.value(),
                }),

            midi_note_id: 0,
            midi_note_freq: 1.0,
            midi_note_gain: Smoother::new(SmoothingStyle::Linear(5.0)),
        }
    }
}

impl Default for ParamicParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(300, 600),

            gain: FloatParam::new(
                    "Gain",
                    -20.0,
                    FloatRange::Linear {
                        min: -30.0,
                        max: 0.0,
                    },
                ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit(" dB"),

            frequency: FloatParam::new(
                "Frequency",
                420.0,
                FloatRange::Skewed {
                    min: 1.0,
                    max: 20_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
                ).with_smoother(SmoothingStyle::Linear(10.0))
                // We purposely don't specify a step size here, but the parameter should still be
                // displayed as if it were rounded. This formatter also includes the unit.
                .with_value_to_string(formatters::v2s_f32_hz_then_khz(0))
                .with_string_to_value(formatters::s2v_f32_hz_then_khz()),

            use_midi: BoolParam::new("Use MIDI", false),

            a : IntParam::new(
                "a",
                1,
                IntRange::Linear {
                    min: 1,
                    max: 100,
                },
            ),
            b : IntParam::new(
                "b",
                7,
                IntRange::Linear {
                    min: 1,
                    max: 100,
                },
            ),
            c : IntParam::new(
                "c",
                1,
                IntRange::Linear {
                    min: 1,
                    max: 100,
                },
            ),
            d : IntParam::new(
                "d",
                7,
                IntRange::Linear {
                    min: 1,
                    max: 100,
                },
            ),
            j : IntParam::new(
                "j",
                3,
                IntRange::Linear {
                    min: 1,
                    max: 100,
                },
            ),
            k : IntParam::new(
                "k",
                3,
                IntRange::Linear {
                    min: 1,
                    max: 100,
                },
            ),
        }
    }
}

impl Paramic {
    fn calculate_sample(&mut self, frequency: f32) -> f32 {
        self.oscillator.set_frequency(frequency);
        self.oscillator.sample()
    }
}

impl Plugin for Paramic {
    const NAME: &'static str = "Oscillator test";
    const VENDOR: &'static str = "Leonard Eyer";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            // This is also the default and can be omitted here
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        create_egui_editor(
            self.params.editor_state.clone(),
            (),
            |_, _| {},
            move |egui_ctx, setter, _state| {
                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    ui.label("Gain");
                    ui.add(widgets::ParamSlider::for_param(&params.gain, setter));

                    ui.label("Frequency (Hz)");
                    ui.add(widgets::ParamSlider::for_param(&params.frequency, setter));

                    ui.label("Parametric Equation");
                    ui.label("(a, b, c, d, j, k)");
                    ui.add(widgets::ParamSlider::for_param(&params.a, setter));
                    ui.add(widgets::ParamSlider::for_param(&params.b, setter));
                    ui.add(widgets::ParamSlider::for_param(&params.c, setter));
                    ui.add(widgets::ParamSlider::for_param(&params.d, setter));
                    ui.add(widgets::ParamSlider::for_param(&params.j, setter));
                    ui.add(widgets::ParamSlider::for_param(&params.k, setter));

                    ui.label("Use MIDI");
                    ui.add(widgets::ParamSlider::for_param(&params.use_midi, setter));

                    let equation = EquationA {
                        a: params.a.value(),
                        b: params.b.value(),
                        c: params.c.value(),
                        d: params.d.value(),
                        j: params.j.value(),
                        k: params.k.value(),
                    };

                    let period = equation.get_period();
                    
                    let curvepoints: Vec<[f64; 2]> = (0..1000).map(|i| {
                        let t = (i as f64 / 1000.) * period;
                        let (x, y) = equation.get_position(t);
                        [x, y]
                    }).collect();
                    
                    let curve: egui::widgets::plot::PlotPoints = curvepoints.iter()
                        .map(|[x, y]| [*x, *y]).collect();
                    let line = egui::widgets::plot::Line::new(curve);

                    ui.label("Parametric curve");
                    egui::widgets::plot::Plot::new("Plot")
                        .width(100.)
                        .view_aspect(1.0)
                        .show_axes([false, false])
                        .show_x(false)
                        .show_y(false)
                        .show(ui, |plot_ui| plot_ui.line(line));


                    let signal: egui::widgets::plot::PlotPoints = curvepoints.iter().enumerate().map(|(i, [x, y])| {
                        [i as f64, ((x.powi(2) + y.powi(2)).sqrt() - 1.0)]
                    }).collect();

                    let line = egui::widgets::plot::Line::new(signal);
                    ui.label("Signal");
                    egui::widgets::plot::Plot::new("Plot")
                        .width(100.)
                        .view_aspect(1.0)
                        .show_axes([false, false])
                        .show_x(false)
                        .show_y(false)
                        .show(ui, |plot_ui| plot_ui.line(line));
                });
            },
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        self.oscillator.set_sample_rate(buffer_config.sample_rate);

        true
    }

    fn reset(&mut self) {
        self.midi_note_id = 0;
        self.midi_note_freq = 1.0;
        self.midi_note_gain.reset(0.0);
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();

        self.oscillator.set_equation(EquationA {
            a: self.params.a.value(),
            b: self.params.b.value(),
            c: self.params.c.value(),
            d: self.params.d.value(),
            j: self.params.j.value(),
            k: self.params.k.value(),
        });

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Smoothing is optionally built into the parameters themselves
            let gain = self.params.gain.smoothed.next();

            // This plugin can be either triggered by MIDI or controleld by a parameter
            let wave = if self.params.use_midi.value() {
                // Act on the next MIDI event
                while let Some(event) = next_event {
                    if event.timing() > sample_id as u32 {
                        break;
                    }

                    match event {
                        NoteEvent::NoteOn { note, velocity, .. } => {
                            self.midi_note_id = note;
                            self.midi_note_freq = util::midi_note_to_freq(note);
                            self.midi_note_gain.set_target(self.sample_rate, velocity);
                        }
                        NoteEvent::NoteOff { note, .. } if note == self.midi_note_id => {
                            self.midi_note_gain.set_target(self.sample_rate, 0.0);
                        }
                        NoteEvent::PolyPressure { note, pressure, .. }
                        if note == self.midi_note_id =>
                            {
                                self.midi_note_gain.set_target(self.sample_rate, pressure);
                            }
                        _ => (),
                    }

                    next_event = context.next_event();
                }

                // This gain envelope prevents clicks with new notes and with released notes
                self.calculate_sample(self.midi_note_freq) * self.midi_note_gain.next()
            } else {
                let frequency = self.params.frequency.smoothed.next();
                self.calculate_sample(frequency)
            };

            for sample in channel_samples {
                *sample = wave * util::db_to_gain_fast(gain);
            }
        }

        ProcessStatus::KeepAlive
    }
}

impl ClapPlugin for Paramic {
    const CLAP_ID: &'static str = "com.leonard-eyer.oscillator";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("An optionally MIDI controlled oscillator test tone");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for Paramic {
    const VST3_CLASS_ID: [u8; 16] = *b"OsillcatorTestPl";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Tools,
    ];
}

nih_export_clap!(Paramic);
nih_export_vst3!(Paramic);
