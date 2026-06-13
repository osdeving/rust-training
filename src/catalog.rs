use crate::content::load_modules;
use crate::domain::{Exercise, TrainingModule};

pub fn all_modules() -> Vec<TrainingModule> {
    load_modules().unwrap_or_else(|error| {
        eprintln!("Falha ao carregar catalogo de exercicios: {error}");
        Vec::new()
    })
}

pub fn find_exercise(id: &str) -> Option<Exercise> {
    all_modules()
        .into_iter()
        .flat_map(|module| module.exercises)
        .find(|exercise| exercise.id == id)
}
