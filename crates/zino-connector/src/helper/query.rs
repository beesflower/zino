use regex::{Captures, Regex};
use std::borrow::Cow;
use zino_core::{JsonValue, LazyLock, Map};

/// Formats the query using interpolation of the parameters.
///
/// The interpolation parameter is represented as `${param}`,
/// in which `param` can only contain restricted chracters `[a-zA-Z]+[\w\.]*`.
pub(crate) fn format_query<'a>(query: &'a str, params: Option<&'a Map>) -> Cow<'a, str> {
    if let Some(params) = params.filter(|_| query.contains('$')) {
        INTERPOLATION_PATTERN.replace_all(query, |captures: &Captures| {
            let key = &captures[1];
            params
                .get(key)
                .map(|value| match value {
                    JsonValue::String(s) => s.to_owned(),
                    _ => value.to_string(),
                })
                .unwrap_or_else(|| ["${", key, "}"].concat())
        })
    } else {
        Cow::Borrowed(query)
    }
}

/// Regex for the interpolation parameter.
static INTERPOLATION_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\$\{\s*([a-zA-Z]+[\w\.]*)\s*\}")
        .expect("fail to create a regex for the interpolation parameter")
});

#[cfg(test)]
mod tests {
    use zino_core::{Map, extension::JsonObjectExt};

    #[test]
    fn it_formats_query_params() {
        let query = "SELECT ${fields} FROM users WHERE name = 'alice' AND age >= #{age};";
        let mut params = Map::new();
        params.upsert("fields", "id, name, age");
        params.upsert("age", 18);

        let sql = super::format_query(query, Some(&params));
        assert_eq!(
            sql,
            "SELECT id, name, age FROM users WHERE name = 'alice' AND age >= #{age};"
        );
    }
}
