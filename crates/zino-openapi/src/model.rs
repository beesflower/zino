use ahash::{HashMap, HashMapExt};
use convert_case::{Case, Casing};
use zino_core::{LazyLock, Map, extension::JsonObjectExt, model::Translation};

/// Translates the model data.
pub fn translate_model_entry(model: &mut Map, model_name: &str) {
    let mut data = Map::new();
    let model_name_prefix = [model_name, "."].concat();
    for (key, translation) in MODEL_TRANSLATIONS.iter() {
        if let Some(field) = key.strip_prefix(&model_name_prefix) {
            if let Some(value) = model.get(field) {
                let translated_field = [field, "_translated"].concat();
                let translated_value = translation
                    .translate(value)
                    .unwrap_or_else(|| value.to_owned());
                data.upsert(translated_field, translated_value);
            }
        }
    }
    model.append(&mut data);
}

/// Model translations.
static MODEL_TRANSLATIONS: LazyLock<HashMap<&str, Translation>> = LazyLock::new(|| {
    let mut model_translations = HashMap::new();
    if let Some(definitions) = super::MODEL_DEFINITIONS.get() {
        for (model_name, fields) in definitions.iter() {
            for (field, value) in fields {
                let translation = value.as_table().map(Translation::with_config);
                if let Some(translation) = translation.filter(|t| t.is_ready()) {
                    let model_name = model_name.to_case(Case::Snake);
                    let model_key = format!("{model_name}.{field}").leak() as &'static str;
                    model_translations.insert(model_key, translation);
                }
            }
        }
    }
    model_translations
});
