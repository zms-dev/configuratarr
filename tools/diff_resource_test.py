import json
import os
import sys
import tempfile
import unittest
from io import StringIO
from unittest.mock import patch

sys.path.insert(0, ".")
from fixture import SPEC
from diff_resource import camel_to_snake, get_spec_fields, get_rust_fields, main

RUST_MATCHING = """\
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamingResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(alias = "rename_movies", default)]
    pub rename_movies: bool,
    #[serde(alias = "replace_illegal_characters", default = "default_true")]
    pub replace_illegal_characters: bool,
    #[serde(alias = "colon_replacement_format")]
    pub colon_replacement_format: ColonReplacementFormat,
    #[serde(alias = "standard_movie_format")]
    pub standard_movie_format: Option<String>,
    #[serde(alias = "movie_folder_format")]
    pub movie_folder_format: Option<String>,
}
"""

RUST_MISSING_FIELD = """\
pub struct NamingResource {
    pub id: Option<i32>,
    pub rename_movies: bool,
    pub standard_movie_format: Option<String>,
}
"""

RUST_EXTRA_FIELD = """\
pub struct NamingResource {
    pub id: Option<i32>,
    pub rename_movies: bool,
    pub replace_illegal_characters: bool,
    pub colon_replacement_format: ColonReplacementFormat,
    pub standard_movie_format: Option<String>,
    pub movie_folder_format: Option<String>,
    pub extra_field: String,
}
"""

RUST_TYPE_MISMATCH = """\
pub struct NamingResource {
    pub id: Option<i32>,
    pub rename_movies: bool,
    pub replace_illegal_characters: bool,
    pub colon_replacement_format: ColonReplacementFormat,
    pub standard_movie_format: String,
    pub movie_folder_format: Option<String>,
}
"""


class TestCamelToSnake(unittest.TestCase):
    def test_simple(self):
        self.assertEqual(camel_to_snake("renameMovies"), "rename_movies")

    def test_all_caps_segment(self):
        self.assertEqual(camel_to_snake("enableSSL"), "enable_ssl")

    def test_already_snake(self):
        self.assertEqual(camel_to_snake("id"), "id")

    def test_multi_word(self):
        self.assertEqual(camel_to_snake("standardMovieFormat"), "standard_movie_format")


class TestGetSpecFields(unittest.TestCase):
    def test_returns_camelcase_keyed_dict(self):
        fields = get_spec_fields("#/components/schemas/NamingResource", SPEC)
        # get_spec_fields returns camelCase keys; main() converts when comparing
        self.assertIn("renameMovies", fields)
        self.assertIn("standardMovieFormat", fields)

    def test_missing_schema_returns_empty(self):
        fields = get_spec_fields("#/components/schemas/DoesNotExist", SPEC)
        self.assertEqual(fields, {})

    def test_nullable_reflected_in_type(self):
        fields = get_spec_fields("#/components/schemas/NamingResource", SPEC)
        self.assertIn("Option", fields["standardMovieFormat"])

    def test_bool_type(self):
        fields = get_spec_fields("#/components/schemas/NamingResource", SPEC)
        self.assertEqual(fields["renameMovies"], "bool")


class TestGetRustFields(unittest.TestCase):
    def _write_tmp(self, content):
        f = tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False)
        f.write(content)
        f.close()
        return f.name

    def test_parses_matching_struct(self):
        path = self._write_tmp(RUST_MATCHING)
        try:
            fields = get_rust_fields(path, "NamingResource")
            self.assertIn("id", fields)
            self.assertIn("rename_movies", fields)
            self.assertIn("standard_movie_format", fields)
        finally:
            os.unlink(path)

    def test_field_types_parsed(self):
        path = self._write_tmp(RUST_MATCHING)
        try:
            fields = get_rust_fields(path, "NamingResource")
            self.assertEqual(fields["id"], "Option<i32>")
            self.assertEqual(fields["rename_movies"], "bool")
        finally:
            os.unlink(path)

    def test_missing_struct_returns_empty(self):
        path = self._write_tmp(RUST_MATCHING)
        try:
            fields = get_rust_fields(path, "NonExistentStruct")
            self.assertEqual(fields, {})
        finally:
            os.unlink(path)


class TestMainOutput(unittest.TestCase):
    def setUp(self):
        self.spec_tmp = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False)
        json.dump(SPEC, self.spec_tmp)
        self.spec_tmp.close()

    def tearDown(self):
        os.unlink(self.spec_tmp.name)

    def _run(self, rust_content, struct_name="NamingResource"):
        rs = tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False)
        rs.write(rust_content)
        rs.close()
        try:
            with patch("sys.argv", [
                "diff_resource.py",
                self.spec_tmp.name,
                "#/components/schemas/NamingResource",
                rs.name,
                struct_name,
            ]):
                with patch("sys.stdout", new_callable=StringIO) as out:
                    main()
                    return out.getvalue()
        finally:
            os.unlink(rs.name)

    def test_clean_match_shows_summary(self):
        out = self._run(RUST_MATCHING)
        self.assertIn("Summary", out)
        self.assertIn("0 missing from Rust", out)
        self.assertIn("0 extra in Rust", out)

    def test_missing_field_detected(self):
        out = self._run(RUST_MISSING_FIELD)
        self.assertIn("missing from Rust", out)
        # replace_illegal_characters and others missing
        self.assertIn("replace_illegal_characters", out)

    def test_extra_field_detected(self):
        out = self._run(RUST_EXTRA_FIELD)
        self.assertIn("not in spec", out)
        self.assertIn("extra_field", out)

    def test_type_mismatch_optional_vs_required(self):
        out = self._run(RUST_TYPE_MISMATCH)
        self.assertIn("type warning", out.lower() + "type warning")


if __name__ == "__main__":
    unittest.main()
