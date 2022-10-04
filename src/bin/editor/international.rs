use bevy::prelude::*;
use bevy_egui::egui;
use aerugo::international::AerugoImanity;
use crate::*;


pub fn edit_international(
    commands: &mut Commands,
    egui_ctx: &egui::Context,
    aerugo: &mut Aerugo,
    internationale: &mut Internationale,
    lang: &mut ImanityLangs,
)
{
    let mut ru = AerugoImanity { lang: ImanityLangs::Ru, localized: vec![] };
    let mut en = AerugoImanity { lang: ImanityLangs::En, localized: vec![] };

    for interaction in &mut internationale.defs {
        match interaction.lang {
            ImanityLangs::Ru => {
                ru.localized.extend_from_slice(&interaction.localized);
            }
            ImanityLangs::En => {
                en.localized.extend_from_slice(&interaction.localized);
            }
        }
    }

    assert_eq!(ru.localized.len(), en.localized.len());

    egui::CentralPanel::default().show(
        egui_ctx,
        |mut ui| {
            let ru_en = ru.localized.iter_mut()
                .zip(en.localized.iter_mut());
            egui::ScrollArea::vertical().auto_shrink([false, true]).show(
                &mut ui,
                |ui| {
                    for (ru, en) in ru_en {
                        ui.horizontal(|ui| {
                            ui.label(ru.id.to_string());
                            if ru.id == en.id {
                                ui.label("OK");
                            } else {
                                ui.label("NOT");
                            };
                            match (&mut ru.inner, &mut en.inner) {
                                (Steps::Text {
                                    author: ru_a, texts: ru_t
                                }, Steps::Text {
                                    author: en_a, texts: en_t
                                }) => {
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("RU");
                                            ui.text_edit_singleline(ru_a);
                                            ui.text_edit_multiline(ru_t);
                                        });
                                        ui.horizontal(|ui| {
                                            ui.label("EN");
                                            ui.text_edit_singleline(en_a);
                                            ui.text_edit_multiline(en_t);
                                        });
                                    });
                                }
                                (Steps::Phrase { phrases: ru },
                                    Steps::Phrase { phrases: en }) => {
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("RU");
                                            ui.separator();
                                            ui.label("EN");
                                        });
                                        for (ru, en) in ru.iter_mut()
                                            .zip(en.iter_mut()) {
                                            ui.horizontal(|ui| {
                                                ui.text_edit_singleline(&mut ru.1);
                                                ui.separator();
                                                ui.text_edit_singleline(&mut en.1);
                                            });
                                        }
                                    });
                                }
                                (_, _) => {}
                            }
                        });
                        ui.separator();
                    }
                },
            )
        },
    );

    internationale.defs.clear();
    internationale.defs.push(ru);
    internationale.defs.push(en);
}
