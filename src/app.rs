use crate::ui::ui;
use crate::ising::Ising;
use crate::parameter::{Parameter, ParameterType};

use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rand::rngs::ThreadRng;
use ratatui::{backend::Backend, Terminal};


#[derive(Debug, PartialEq)]
pub enum Page {
    Main,
    // Edit,
    Exit,
}

#[derive(Debug)]
pub enum MCOrder {
    Linear,
    Linearithmic,
    Quadratic,
}

#[derive(Debug)]
pub struct App {
    pub page: Page,
    pub current_parameter: ParameterType,
    pub ising: Ising,
    pub mc_order: MCOrder,
    pub thread_rng: ThreadRng,
    pub paused: bool,
    pub temp_param: Parameter,
    pub coupling_param: Parameter,
    pub mag_moment_param: Parameter,
    pub mag_field_strength_param: Parameter,
    pub magnetization: f64,
    pub energy: f64,
}

impl App {
    pub fn new(size: usize) -> Self {
        Self {
            page: Page::Main,
            current_parameter: ParameterType::Temp,
            ising: Ising::new(size),
            mc_order: MCOrder::Linearithmic,
            thread_rng: ThreadRng::default(),
            paused: false,
            temp_param: Parameter::new(1024, 4, (0, 1024), 2.269 * 2.0),
            coupling_param: Parameter::new(1024, 4, (0, 1024), 1.0),
            mag_moment_param: Parameter::new(0, 4, (0, 1024), 0.1),
            mag_field_strength_param: Parameter::new(0, 4, (0, 1024), 1.0),
            magnetization: 0.0,
            energy: 0.0,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            if self.page == Page::Main {
                let n = &self.ising.size;
                let n_steps: usize;
                match self.mc_order {
                    MCOrder::Linear => {
                        n_steps = *n;
                    }
                    MCOrder::Linearithmic => {
                        n_steps = n * (*n as f64).log2() as usize;
                    }
                    MCOrder::Quadratic => {
                        n_steps = n.pow(2);
                    }
                }

                self.ising.temperature = self.temp_param.scaled();
                self.ising.coupling_constant = self.coupling_param.scaled();
                self.ising.magnetic_moment = self.mag_moment_param.scaled();
                self.ising.magnetic_field_strength = self.mag_field_strength_param.scaled();

                if !self.paused {   
                    for _ in 0..n_steps {
                        self.ising.monte_carlo_step(&mut self.thread_rng);
                    }
                }
            }

            terminal.draw(|f| ui(f, self))?;

            if event::poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key_event) = event::read()? {
                    match self.page {
                        Page::Main => match key_event.code {
                            KeyCode::Char('q') => self.page = Page::Exit,
                            KeyCode::Char(' ') => self.paused = !self.paused,
                            KeyCode::Char('+') => match self.mc_order {
                                MCOrder::Linear => self.mc_order = MCOrder::Linearithmic,
                                MCOrder::Linearithmic => self.mc_order = MCOrder::Quadratic,
                                MCOrder::Quadratic => {}
                            },
                            KeyCode::Char('-') => match self.mc_order {
                                MCOrder::Linear => {}
                                MCOrder::Linearithmic => self.mc_order = MCOrder::Linear,
                                MCOrder::Quadratic => self.mc_order = MCOrder::Linearithmic,
                            },
                            KeyCode::Up => match self.current_parameter {
                                ParameterType::Temp => {},
                                ParameterType::Coupling => self.current_parameter = ParameterType::Temp,
                                ParameterType::MagMoment => self.current_parameter = ParameterType::Coupling,
                                ParameterType::MagFieldStrength => self.current_parameter = ParameterType::MagMoment,
                            }
                            KeyCode::Down => match self.current_parameter {
                                ParameterType::Temp => self.current_parameter = ParameterType::Coupling,
                                ParameterType::Coupling => self.current_parameter = ParameterType::MagMoment,
                                ParameterType::MagMoment => self.current_parameter = ParameterType::MagFieldStrength,
                                ParameterType::MagFieldStrength => {},
                            }
                            KeyCode::Left => match self.current_parameter {
                                ParameterType::Temp => {
                                    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                                        self.temp_param.decrease_value(*self.temp_param.step());
                                    } else {
                                        self.temp_param.decrease_value(self.temp_param.step() * 8);
                                    }
                                },
                                ParameterType::Coupling => {
                                    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                                        self.coupling_param.decrease_value(*self.coupling_param.step());
                                    } else {
                                        self.coupling_param.decrease_value(self.coupling_param.step() * 8);
                                    }
                                },
                                ParameterType::MagMoment => {
                                    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                                        self.mag_moment_param.decrease_value(*self.mag_moment_param.step());
                                    } else {
                                        self.mag_moment_param.decrease_value(self.mag_moment_param.step() * 8);
                                    }
                                },
                                ParameterType::MagFieldStrength => {
                                    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                                        self.mag_field_strength_param.decrease_value(*self.mag_field_strength_param.step());
                                    } else {
                                        self.mag_field_strength_param.decrease_value(self.mag_field_strength_param.step() * 8);
                                    }
                                },
                            }
                            KeyCode::Right => match self.current_parameter {
                                ParameterType::Temp => {
                                    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                                        self.temp_param.increase_value(*self.temp_param.step());
                                    } else {
                                        self.temp_param.increase_value(self.temp_param.step() * 8);
                                    }
                                },
                                ParameterType::Coupling => {
                                    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                                        self.coupling_param.increase_value(*self.coupling_param.step());
                                    } else {
                                        self.coupling_param.increase_value(self.coupling_param.step() * 8);
                                    }
                                },
                                ParameterType::MagMoment => {
                                    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                                        self.mag_moment_param.increase_value(*self.mag_moment_param.step());
                                    } else {
                                        self.mag_moment_param.increase_value(self.mag_moment_param.step() * 8);
                                    }
                                },
                                ParameterType::MagFieldStrength => {
                                    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                                        self.mag_field_strength_param.increase_value(*self.mag_field_strength_param.step());
                                    } else {
                                        self.mag_field_strength_param.increase_value(self.mag_field_strength_param.step() * 8);
                                    }
                                },
                            }
                            _ => {}
                        }
                        // Page::Edit => match key.code {
                        //     KeyCode::Char('q') => self.page = Page::Main,
                        //     _ => {}
                        // },
                        Page::Exit => match key_event.code {
                                KeyCode::Char('y') | KeyCode::Char('q') => return Ok(()),
                                KeyCode::Char('n') => self.page = Page::Main,
                                _ => {}
                            },
                        };
                    }
                }
            }
        }
    }

