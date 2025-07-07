// Cargo.toml dependencies:
/*
[dependencies]
eframe = "0.28"
egui = "0.28"
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.0", features = ["v4"] }
tokio = { version = "1.0", features = ["full"], optional = true }

[profile.dev]
opt-level = 2

[profile.release]
opt-level = 3
lto = true
*/

use eframe::egui;
use egui::{
    Color32, FontFamily, FontId, RichText, Stroke, Vec2, Ui, Context, CentralPanel, SidePanel, TopBottomPanel
};
use chrono::{DateTime, Local};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TriageLevel {
    Critical,
    High,
    Medium,
    Low,
}

impl TriageLevel {
    fn color(&self) -> Color32 {
        match self {
            TriageLevel::Critical => Color32::from_rgb(231, 76, 60),
            TriageLevel::High => Color32::from_rgb(243, 156, 18),
            TriageLevel::Medium => Color32::from_rgb(241, 196, 15),
            TriageLevel::Low => Color32::from_rgb(46, 204, 113),
        }
    }
    
    fn text(&self) -> &str {
        match self {
            TriageLevel::Critical => "CRITICAL",
            TriageLevel::High => "HIGH",
            TriageLevel::Medium => "MEDIUM",
            TriageLevel::Low => "LOW",
        }
    }
}

#[derive(Debug, Clone)]
pub struct VitalSigns {
    blood_pressure: (i32, i32),
    heart_rate: i32,
    oxygen_saturation: i32,
    temperature: f32,
}

impl VitalSigns {
    fn bp_status(&self) -> TriageLevel {
        if self.blood_pressure.0 > 180 || self.blood_pressure.1 > 120 {
            TriageLevel::Critical
        } else if self.blood_pressure.0 > 140 || self.blood_pressure.1 > 90 {
            TriageLevel::High
        } else {
            TriageLevel::Low
        }
    }
    
    fn hr_status(&self) -> TriageLevel {
        if self.heart_rate < 50 || self.heart_rate > 120 {
            TriageLevel::Critical
        } else if self.heart_rate < 60 || self.heart_rate > 100 {
            TriageLevel::High
        } else {
            TriageLevel::Low
        }
    }
    
    fn o2_status(&self) -> TriageLevel {
        if self.oxygen_saturation < 90 {
            TriageLevel::Critical
        } else if self.oxygen_saturation < 95 {
            TriageLevel::High
        } else {
            TriageLevel::Low
        }
    }
}

#[derive(Debug, Clone)]
pub struct Patient {
    id: String,
    age: u8,
    gender: String,
    chief_complaint: String,
    triage_level: TriageLevel,
    vitals: VitalSigns,
    location: String,
    eta_minutes: Option<u32>,
    ambulance_id: Option<String>,
    paramedic: Option<String>,
    notes: Vec<String>,
    timestamp: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct Hospital {
    name: String,
    available_beds: u32,
    total_beds: u32,
    distance_minutes: u32,
    specialties: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Specialist {
    name: String,
    specialty: String,
    available: bool,
    on_call: bool,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    id: Uuid,
    sender: String,
    message: String,
    timestamp: DateTime<Local>,
    urgent: bool,
}

#[derive(Debug)]
pub struct EmergencyApp {
    patients: Vec<Patient>,
    hospitals: Vec<Hospital>,
    specialists: Vec<Specialist>,
    chat_messages: Vec<ChatMessage>,
    active_tab: usize,
    chat_input: String,
    selected_patient: Option<usize>,
    ambulance_available: u32,
    ambulance_en_route: u32,
    ambulance_at_scene: u32,
}

impl Default for EmergencyApp {
    fn default() -> Self {
        Self {
            patients: create_demo_patients(),
            hospitals: create_demo_hospitals(),
            specialists: create_demo_specialists(),
            chat_messages: create_demo_messages(),
            active_tab: 0,
            chat_input: String::new(),
            selected_patient: None,
            ambulance_available: 12,
            ambulance_en_route: 8,
            ambulance_at_scene: 3,
        }
    }
}

impl eframe::App for EmergencyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Configure fonts and style
        self.configure_fonts(ctx);
        
        // Set dark theme
        ctx.set_visuals(egui::Visuals::dark());
        
        // Request repaint every second for real-time updates
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
        
        // Header
        TopBottomPanel::top("header").show(ctx, |ui| {
            self.render_header(ui);
        });
        
        // Left sidebar
        SidePanel::left("sidebar").min_width(280.0).show(ctx, |ui| {
            self.render_sidebar(ui);
        });
        
        // Right chat panel
        SidePanel::right("chat").min_width(300.0).show(ctx, |ui| {
            self.render_chat_panel(ui);
        });
        
        // Main content area
        CentralPanel::default().show(ctx, |ui| {
            self.render_main_content(ui);
        });
    }
}

impl EmergencyApp {
    fn configure_fonts(&self, ctx: &Context) {
        // Using default fonts for now - in production you can add custom fonts
        let fonts = egui::FontDefinitions::default();
        ctx.set_fonts(fonts);
    }
    
    fn render_header(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            
            // Logo and title
            ui.label(
                RichText::new("🏥 Dubai Health Authority - Emergency Response")
                    .font(FontId::new(18.0, FontFamily::Proportional))
                    .color(Color32::WHITE)
                    .strong()
            );
            
            ui.add_space(20.0);
            
            // Emergency status
            let emergency_count = self.patients.len();
            ui.label(
                RichText::new(format!("🚨 {} ACTIVE EMERGENCIES", emergency_count))
                    .font(FontId::new(14.0, FontFamily::Proportional))
                    .color(Color32::from_rgb(231, 76, 60))
                    .strong()
            );
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Current time
                let now = Local::now();
                ui.label(
                    RichText::new(format!("🕐 {} GST", now.format("%H:%M:%S")))
                        .color(Color32::LIGHT_GRAY)
                );
                
                ui.add_space(15.0);
                
                // User info
                ui.label(
                    RichText::new("👨‍⚕️ Dr. Ahmed Al-Mansoori - ER Director")
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(Color32::from_rgb(46, 204, 113))
                );
                
                ui.add_space(15.0);
                
                // Location
                ui.label(
                    RichText::new("📍 Dubai Healthcare City")
                        .color(Color32::LIGHT_GRAY)
                );
            });
        });
        
        ui.add_space(5.0);
        ui.separator();
    }
    
    fn render_sidebar(&mut self, ui: &mut Ui) {
        ui.add_space(10.0);
        
        // Hospitals section
        ui.label(
            RichText::new("🏥 DHA HOSPITALS")
                .font(FontId::new(14.0, FontFamily::Proportional))
                .color(Color32::LIGHT_GRAY)
                .strong()
        );
        
        ui.add_space(10.0);
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, hospital) in self.hospitals.iter().enumerate() {
                let is_selected = i == 0; // Dubai Hospital selected by default
                
                let bg_color = if is_selected {
                    Color32::from_rgb(63, 81, 181)
                } else {
                    Color32::from_rgb(52, 73, 94)
                };
                
                let frame = egui::Frame::none()
                    .fill(bg_color)
                    .rounding(6.0)
                    .inner_margin(egui::style::Margin::same(8.0));
                
                frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(&hospital.name)
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(Color32::WHITE)
                                    .strong()
                            );
                            
                            ui.horizontal(|ui| {
                                // Bed status indicator
                                let bed_color = if hospital.available_beds > 2 {
                                    Color32::from_rgb(46, 204, 113)
                                } else if hospital.available_beds > 0 {
                                    Color32::from_rgb(243, 156, 18)
                                } else {
                                    Color32::from_rgb(231, 76, 60)
                                };
                                
                                ui.painter().circle_filled(
                                    ui.next_widget_position() + Vec2::new(4.0, 4.0),
                                    4.0,
                                    bed_color,
                                );
                                ui.add_space(12.0);
                                
                                let bed_text = if hospital.available_beds > 0 {
                                    format!("{} Available", hospital.available_beds)
                                } else {
                                    "Full Capacity".to_string()
                                };
                                
                                ui.label(
                                    RichText::new(bed_text)
                                        .font(FontId::new(11.0, FontFamily::Proportional))
                                        .color(Color32::LIGHT_GRAY)
                                );
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(
                                        RichText::new(format!("{} min", hospital.distance_minutes))
                                            .font(FontId::new(11.0, FontFamily::Proportional))
                                            .color(Color32::LIGHT_GRAY)
                                    );
                                });
                            });
                        });
                    });
                });
                
                ui.add_space(8.0);
            }
            
            ui.add_space(15.0);
            
            // Specialists section
            ui.label(
                RichText::new("👨‍⚕️ SPECIALISTS ON-CALL")
                    .font(FontId::new(14.0, FontFamily::Proportional))
                    .color(Color32::LIGHT_GRAY)
                    .strong()
            );
            
            ui.add_space(10.0);
            
            for specialist in &self.specialists {
                let frame = egui::Frame::none()
                    .fill(Color32::from_rgb(61, 86, 117))
                    .rounding(6.0)
                    .inner_margin(egui::style::Margin::same(8.0));
                
                frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{} - {}", specialist.name, specialist.specialty))
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(Color32::WHITE)
                        );
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let status_color = if specialist.available {
                                Color32::from_rgb(46, 204, 113)
                            } else if specialist.on_call {
                                Color32::from_rgb(243, 156, 18)
                            } else {
                                Color32::from_rgb(231, 76, 60)
                            };
                            
                            ui.painter().circle_filled(
                                ui.next_widget_position() + Vec2::new(5.0, 5.0),
                                5.0,
                                status_color,
                            );
                            ui.add_space(15.0);
                        });
                    });
                });
                
                ui.add_space(5.0);
            }
            
            ui.add_space(15.0);
            
            // Ambulance status section
            ui.label(
                RichText::new("🚑 AMBULANCE STATUS")
                    .font(FontId::new(14.0, FontFamily::Proportional))
                    .color(Color32::LIGHT_GRAY)
                    .strong()
            );
            
            ui.add_space(10.0);
            
            let frame = egui::Frame::none()
                .fill(Color32::from_rgb(52, 73, 94))
                .rounding(6.0)
                .inner_margin(egui::style::Margin::same(10.0));
            
            frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{}", self.ambulance_available))
                                .font(FontId::new(18.0, FontFamily::Proportional))
                                .color(Color32::from_rgb(46, 204, 113))
                                .strong()
                        );
                        ui.label(
                            RichText::new("Available")
                                .font(FontId::new(10.0, FontFamily::Proportional))
                                .color(Color32::LIGHT_GRAY)
                        );
                    });
                    
                    ui.add_space(20.0);
                    
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{}", self.ambulance_en_route))
                                .font(FontId::new(18.0, FontFamily::Proportional))
                                .color(Color32::from_rgb(231, 76, 60))
                                .strong()
                        );
                        ui.label(
                            RichText::new("En Route")
                                .font(FontId::new(10.0, FontFamily::Proportional))
                                .color(Color32::LIGHT_GRAY)
                        );
                    });
                    
                    ui.add_space(20.0);
                    
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{}", self.ambulance_at_scene))
                                .font(FontId::new(18.0, FontFamily::Proportional))
                                .color(Color32::from_rgb(243, 156, 18))
                                .strong()
                        );
                        ui.label(
                            RichText::new("At Scene")
                                .font(FontId::new(10.0, FontFamily::Proportional))
                                .color(Color32::LIGHT_GRAY)
                        );
                    });
                });
            });
        });
    }
    
    fn render_main_content(&mut self, ui: &mut Ui) {
        // Tabs
        ui.horizontal(|ui| {
            let tabs = vec!["🚨 Active Emergencies", "📋 Incoming Patients", "🏥 Hospital Status", "📊 Analytics"];
            
            for (i, tab) in tabs.iter().enumerate() {
                let is_active = i == self.active_tab;
                
                if ui.selectable_label(is_active, *tab).clicked() {
                    self.active_tab = i;
                }
                
                ui.add_space(10.0);
            }
        });
        
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(15.0);
        
        // Content based on active tab
        match self.active_tab {
            0 => self.render_active_emergencies(ui),
            1 => self.render_incoming_patients(ui),
            2 => self.render_hospital_status(ui),
            3 => self.render_analytics(ui),
            _ => {}
        }
    }
    
    fn render_active_emergencies(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Clone patients to avoid borrow checker issues
            let patients = self.patients.clone();
            
            ui.vertical(|ui| {
                for (i, patient) in patients.iter().enumerate() {
                    self.render_patient_card(ui, &patient, i);
                    ui.add_space(15.0); // Add spacing between cards
                }
            });
        });
    }
    
    fn render_patient_card(&mut self, ui: &mut Ui, patient: &Patient, index: usize) {
        let triage_color = patient.triage_level.color();
        
        let frame = egui::Frame::none()
            .fill(Color32::from_gray(245))
            .stroke(Stroke::new(3.0, triage_color))
            .rounding(12.0)
            .inner_margin(egui::style::Margin::same(15.0));
        
        frame.show(ui, |ui| {
            ui.set_width(ui.available_width()); // Use full available width
            
            // Patient header
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(&patient.id)
                        .font(FontId::new(16.0, FontFamily::Proportional))
                        .color(Color32::from_gray(50))
                        .strong()
                );
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let triage_frame = egui::Frame::none()
                        .fill(triage_color)
                        .rounding(20.0)
                        .inner_margin(egui::style::Margin::symmetric(12.0, 6.0));
                    
                    triage_frame.show(ui, |ui| {
                        ui.label(
                            RichText::new(patient.triage_level.text())
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(Color32::WHITE)
                                .strong()
                        );
                    });
                });
            });
            
            ui.add_space(10.0);
            
            // Patient details - now stacked vertically
            ui.vertical(|ui| {
                // Age/Gender
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Age/Gender:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(Color32::from_gray(100))
                            .strong()
                    );
                    ui.label(
                        RichText::new(format!("{}{}", patient.age, patient.gender))
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(Color32::from_gray(50))
                    );
                });
                
                ui.add_space(5.0);
                
                // Chief Complaint
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Chief Complaint:")
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(Color32::from_gray(100))
                            .strong()
                    );
                    ui.label(
                        RichText::new(&patient.chief_complaint)
                            .font(FontId::new(13.0, FontFamily::Proportional))
                            .color(Color32::from_gray(50))
                    );
                });
                
                ui.add_space(5.0);
                
                // Ambulance (if exists)
                if let Some(ambulance) = &patient.ambulance_id {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Ambulance:")
                                .font(FontId::new(13.0, FontFamily::Proportional))
                                .color(Color32::from_gray(100))
                                .strong()
                        );
                        ui.label(
                            RichText::new(ambulance)
                                .font(FontId::new(13.0, FontFamily::Proportional))
                                .color(Color32::from_gray(50))
                        );
                    });
                    ui.add_space(5.0);
                }
                
                // Paramedic (if exists)
                if let Some(paramedic) = &patient.paramedic {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Paramedic:")
                                .font(FontId::new(13.0, FontFamily::Proportional))
                                .color(Color32::from_gray(100))
                                .strong()
                        );
                        ui.label(
                            RichText::new(paramedic)
                                .font(FontId::new(13.0, FontFamily::Proportional))
                                .color(Color32::from_gray(50))
                        );
                    });
                    ui.add_space(5.0);
                }
            });
            
            ui.add_space(8.0);
            
            // Location
            let location_frame = egui::Frame::none()
                .fill(Color32::from_rgb(220, 240, 255))
                .stroke(Stroke::new(1.0, Color32::from_rgb(52, 152, 219)))
                .rounding(6.0)
                .inner_margin(egui::style::Margin::same(8.0));
            
            location_frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("📍");
                    ui.label(
                        RichText::new(&patient.location)
                            .font(FontId::new(12.0, FontFamily::Proportional))
                            .color(Color32::from_gray(50))
                    );
                });
            });
            
            ui.add_space(8.0);
            
            // Vitals display
            let vitals_frame = egui::Frame::none()
                .fill(Color32::from_gray(236))
                .rounding(8.0)
                .inner_margin(egui::style::Margin::same(12.0));
            
            vitals_frame.show(ui, |ui| {
                egui::Grid::new(format!("vitals_{}", index))
                    .num_columns(3)
                    .spacing([10.0, 0.0])
                    .show(ui, |ui| {
                        // Blood pressure
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(format!("{}/{}", patient.vitals.blood_pressure.0, patient.vitals.blood_pressure.1))
                                    .font(FontId::new(18.0, FontFamily::Proportional))
                                    .color(patient.vitals.bp_status().color())
                                    .strong()
                            );
                            ui.label(
                                RichText::new("BP")
                                    .font(FontId::new(11.0, FontFamily::Proportional))
                                    .color(Color32::from_gray(100))
                            );
                        });
                        
                        // Heart rate
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(format!("{}", patient.vitals.heart_rate))
                                    .font(FontId::new(18.0, FontFamily::Proportional))
                                    .color(patient.vitals.hr_status().color())
                                    .strong()
                            );
                            ui.label(
                                RichText::new("HR")
                                    .font(FontId::new(11.0, FontFamily::Proportional))
                                    .color(Color32::from_gray(100))
                            );
                        });
                        
                        // Oxygen saturation
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(format!("{}%", patient.vitals.oxygen_saturation))
                                    .font(FontId::new(18.0, FontFamily::Proportional))
                                    .color(patient.vitals.o2_status().color())
                                    .strong()
                            );
                            ui.label(
                                RichText::new("O2 Sat")
                                    .font(FontId::new(11.0, FontFamily::Proportional))
                                    .color(Color32::from_gray(100))
                            );
                        });
                    });
            });
            
            ui.add_space(8.0);
            
            // ETA display
            if let Some(eta) = patient.eta_minutes {
                let eta_frame = egui::Frame::none()
                    .fill(Color32::from_rgb(52, 152, 219))
                    .rounding(6.0)
                    .inner_margin(egui::style::Margin::same(8.0));
                
                eta_frame.show(ui, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new(format!("ETA: {} minutes → Dubai Hospital", eta))
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(Color32::WHITE)
                                .strong()
                        );
                    });
                });
            } else {
                let status_frame = egui::Frame::none()
                    .fill(Color32::from_rgb(52, 152, 219))
                    .rounding(6.0)
                    .inner_margin(egui::style::Margin::same(8.0));
                
                status_frame.show(ui, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new("Currently in Triage - Room 3")
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(Color32::WHITE)
                                .strong()
                        );
                    });
                });
            }
            
            ui.add_space(10.0);
            
            // Action buttons
            ui.horizontal(|ui| {
                if ui.button(
                    RichText::new("Accept")
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(Color32::WHITE)
                ).clicked() {
                    // Handle accept action
                }
                
                ui.add_space(8.0);
                
                if ui.button(
                    RichText::new("Call Specialist")
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(Color32::WHITE)
                ).clicked() {
                    // Handle specialist call
                }
                
                ui.add_space(8.0);
                
                if ui.button(
                    RichText::new("Add Notes")
                        .font(FontId::new(12.0, FontFamily::Proportional))
                        .color(Color32::WHITE)
                ).clicked() {
                    // Handle notes
                }
            });
        });
    }
    
    fn render_chat_panel(&mut self, ui: &mut Ui) {
        ui.add_space(10.0);
        
        // Chat header
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("💬 EMERGENCY COMMUNICATION")
                    .font(FontId::new(14.0, FontFamily::Proportional))
                    .color(Color32::LIGHT_GRAY)
                    .strong()
            );
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let notification_frame = egui::Frame::none()
                    .fill(Color32::from_rgb(231, 76, 60))
                    .rounding(10.0)
                    .inner_margin(egui::style::Margin::symmetric(6.0, 3.0));
                
                notification_frame.show(ui, |ui| {
                    ui.label(
                        RichText::new("3")
                            .font(FontId::new(10.0, FontFamily::Proportional))
                            .color(Color32::WHITE)
                            .strong()
                    );
                });
            });
        });
        
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
        
        // Chat messages
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for message in &self.chat_messages {
                    let bg_color = if message.urgent {
                        Color32::from_rgba_premultiplied(231, 76, 60, 30)
                    } else {
                        Color32::from_rgb(61, 86, 117)
                    };
                    
                    let stroke = if message.urgent {
                        Stroke::new(2.0, Color32::from_rgb(231, 76, 60))
                    } else {
                        Stroke::NONE
                    };
                    
                    let frame = egui::Frame::none()
                        .fill(bg_color)
                        .stroke(stroke)
                        .rounding(8.0)
                        .inner_margin(egui::style::Margin::same(10.0));
                    
                    frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(&message.sender)
                                    .font(FontId::new(10.0, FontFamily::Proportional))
                                    .color(Color32::WHITE)
                                    .strong()
                            );
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    RichText::new(message.timestamp.format("%H:%M").to_string())
                                        .font(FontId::new(10.0, FontFamily::Proportional))
                                        .color(Color32::LIGHT_GRAY)
                                );
                            });
                        });
                        
                        ui.add_space(5.0);
                        
                        ui.label(
                            RichText::new(&message.message)
                                .font(FontId::new(12.0, FontFamily::Proportional))
                                .color(Color32::WHITE)
                        );
                    });
                    
                    ui.add_space(8.0);
                }
            });
        
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
        
        // Chat input
        ui.horizontal(|ui| {
            let text_edit = egui::TextEdit::singleline(&mut self.chat_input)
                .hint_text("Type emergency message...")
                .desired_width(ui.available_width() - 60.0);
            
            ui.add(text_edit);
            
            if ui.button(
                RichText::new("Send")
                    .font(FontId::new(12.0, FontFamily::Proportional))
                    .color(Color32::WHITE)
            ).clicked() {
                if !self.chat_input.trim().is_empty() {
                    let new_message = ChatMessage {
                        id: Uuid::new_v4(),
                        sender: "Dr. Ahmed Al-Mansoori".to_string(),
                        message: self.chat_input.clone(),
                        timestamp: Local::now(),
                        urgent: false,
                    };
                    
                    self.chat_messages.push(new_message);
                    self.chat_input.clear();
                }
            }
        });
    }
    
    fn render_incoming_patients(&self, ui: &mut Ui) {
        ui.label("📋 Incoming Patients Dashboard - To be implemented");
    }
    
    fn render_hospital_status(&self, ui: &mut Ui) {
        ui.label("🏥 Hospital Status Dashboard - To be implemented");
    }
    
    fn render_analytics(&self, ui: &mut Ui) {
        ui.label("📊 Analytics Dashboard - To be implemented");
    }
}

// Demo data creation functions
fn create_demo_patients() -> Vec<Patient> {
    vec![
        Patient {
            id: "PATIENT-001".to_string(),
            age: 45,
            gender: "M".to_string(),
            chief_complaint: "Chest Pain".to_string(),
            triage_level: TriageLevel::Critical,
            vitals: VitalSigns {
                blood_pressure: (180, 120),
                heart_rate: 45,
                oxygen_saturation: 89,
                temperature: 37.2,
            },
            location: "Sheikh Zayed Road, near DIFC Metro Station".to_string(),
            eta_minutes: Some(7),
            ambulance_id: Some("AMB-DXB-047".to_string()),
            paramedic: Some("Hassan Al-Rashid".to_string()),
            notes: vec![],
            timestamp: Local::now(),
        },
        Patient {
            id: "PATIENT-002".to_string(),
            age: 28,
            gender: "F".to_string(),
            chief_complaint: "Motor Vehicle Accident".to_string(),
            triage_level: TriageLevel::High,
            vitals: VitalSigns {
                blood_pressure: (140, 85),
                heart_rate: 95,
                oxygen_saturation: 96,
                temperature: 36.8,
            },
            location: "Al Khaleej Road, near Dubai Mall".to_string(),
            eta_minutes: Some(12),
            ambulance_id: Some("AMB-DXB-112".to_string()),
            paramedic: Some("Fatima Al-Zahra".to_string()),
            notes: vec![],
            timestamp: Local::now(),
        },
        Patient {
            id: "PATIENT-003".to_string(),
            age: 8,
            gender: "M".to_string(),
            chief_complaint: "Respiratory Distress".to_string(),
            triage_level: TriageLevel::Medium,
            vitals: VitalSigns {
                blood_pressure: (110, 70),
                heart_rate: 125,
                oxygen_saturation: 91,
                temperature: 38.5,
            },
            location: "Jumeirah Beach Road, near Jumeirah Beach".to_string(),
            eta_minutes: Some(18),
            ambulance_id: Some("AMB-DXB-093".to_string()),
            paramedic: Some("John Mitchell".to_string()),
            notes: vec![],
            timestamp: Local::now(),
        },
        Patient {
            id: "PATIENT-004".to_string(),
            age: 35,
            gender: "F".to_string(),
            chief_complaint: "Minor Laceration".to_string(),
            triage_level: TriageLevel::Low,
            vitals: VitalSigns {
                blood_pressure: (120, 80),
                heart_rate: 72,
                oxygen_saturation: 99,
                temperature: 36.5,
            },
            location: "Dubai Hospital - Triage Room 3".to_string(),
            eta_minutes: None,
            ambulance_id: None,
            paramedic: None,
            notes: vec![],
            timestamp: Local::now(),
        },
    ]
}

fn create_demo_hospitals() -> Vec<Hospital> {
    vec![
        Hospital {
            name: "Dubai Hospital".to_string(),
            available_beds: 3,
            total_beds: 25,
            distance_minutes: 12,
            specialties: vec!["Emergency Medicine".to_string(), "Cardiology".to_string()],
        },
        Hospital {
            name: "Rashid Hospital".to_string(),
            available_beds: 0,
            total_beds: 30,
            distance_minutes: 8,
            specialties: vec!["Trauma Surgery".to_string(), "Neurology".to_string()],
        },
        Hospital {
            name: "American Hospital".to_string(),
            available_beds: 2,
            total_beds: 20,
            distance_minutes: 15,
            specialties: vec!["General Medicine".to_string(), "Pediatrics".to_string()],
        },
        Hospital {
            name: "NMC Healthcare".to_string(),
            available_beds: 1,
            total_beds: 18,
            distance_minutes: 20,
            specialties: vec!["Orthopedics".to_string(), "Cardiology".to_string()],
        },
    ]
}

fn create_demo_specialists() -> Vec<Specialist> {
    vec![
        Specialist {
            name: "Dr. Sarah Johnson".to_string(),
            specialty: "Cardiology".to_string(),
            available: true,
            on_call: false,
        },
        Specialist {
            name: "Dr. Mohammad Khalil".to_string(),
            specialty: "Neurology".to_string(),
            available: false,
            on_call: true,
        },
        Specialist {
            name: "Dr. Lisa Chen".to_string(),
            specialty: "Trauma Surgery".to_string(),
            available: true,
            on_call: false,
        },
        Specialist {
            name: "Dr. Ahmed Rashid".to_string(),
            specialty: "Orthopedics".to_string(),
            available: false,
            on_call: false,
        },
        Specialist {
            name: "Dr. Fatima Al-Zahra".to_string(),
            specialty: "Pediatrics".to_string(),
            available: true,
            on_call: false,
        },
    ]
}

fn create_demo_messages() -> Vec<ChatMessage> {
    vec![
        ChatMessage {
            id: Uuid::new_v4(),
            sender: "Ambulance AMB-047".to_string(),
            message: "Patient showing signs of cardiac arrest. Administered epinephrine. Need cardiologist on standby.".to_string(),
            timestamp: Local::now() - chrono::Duration::minutes(1),
            urgent: true,
        },
        ChatMessage {
            id: Uuid::new_v4(),
            sender: "Dr. Sarah Johnson".to_string(),
            message: "En route to hospital. ETA 3 minutes. Preparing cath lab.".to_string(),
            timestamp: Local::now() - chrono::Duration::minutes(2),
            urgent: false,
        },
        ChatMessage {
            id: Uuid::new_v4(),
            sender: "ER Nurse Station".to_string(),
            message: "Trauma Bay 1 is ready. Blood bank notified for O-negative units.".to_string(),
            timestamp: Local::now() - chrono::Duration::minutes(3),
            urgent: false,
        },
        ChatMessage {
            id: Uuid::new_v4(),
            sender: "Ambulance AMB-112".to_string(),
            message: "MVA patient stable but requesting Arabic-speaking physician for family communication.".to_string(),
            timestamp: Local::now() - chrono::Duration::minutes(4),
            urgent: true,
        },
    ]
}

// Main function to run the application
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1200.0, 800.0])
            .with_title("Dubai Healthcare Emergency Response System"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Dubai Healthcare Emergency Response System",
        options,
        Box::new(|_cc| Box::new(EmergencyApp::default())),
    )
}