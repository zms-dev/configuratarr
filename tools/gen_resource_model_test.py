import json
import os
import sys
import tempfile
import unittest
from io import StringIO
from unittest.mock import patch

sys.path.insert(0, ".")
from fixture import SPEC
from gen_resource_model import camel_to_snake, rust_type, field_attrs, collect_default_fns, gen_struct, main


class TestCamelToSnake(unittest.TestCase):
    def test_simple(self):
        self.assertEqual(camel_to_snake("renameMovies"), "rename_movies")

    def test_single_word(self):
        self.assertEqual(camel_to_snake("id"), "id")

    def test_multi_word(self):
        self.assertEqual(camel_to_snake("standardMovieFormat"), "standard_movie_format")


class TestRustType(unittest.TestCase):
    def test_id_always_option_i32(self):
        t, _ = rust_type("id", {"type": "integer"}, SPEC)
        self.assertEqual(t, "Option<i32>")

    def test_nullable_string(self):
        t, _ = rust_type("name", {"type": "string", "nullable": True}, SPEC)
        self.assertEqual(t, "Option<String>")

    def test_bool(self):
        t, _ = rust_type("enableRss", {"type": "boolean"}, SPEC)
        self.assertEqual(t, "bool")

    def test_integer(self):
        t, _ = rust_type("order", {"type": "integer"}, SPEC)
        self.assertEqual(t, "i32")

    def test_array_of_ref(self):
        prop = {"type": "array", "items": {"$ref": "#/components/schemas/Field"}}
        t, _ = rust_type("fields", prop, SPEC)
        self.assertEqual(t, "Vec<Field>")

    def test_array_of_primitive(self):
        prop = {"type": "array", "items": {"type": "integer"}, "nullable": True}
        t, _ = rust_type("tags", prop, SPEC)
        self.assertEqual(t, "Vec<i32>")

    def test_ref_enum(self):
        prop = {"$ref": "#/components/schemas/ColonReplacementFormat"}
        t, _ = rust_type("colonReplacementFormat", prop, SPEC)
        self.assertEqual(t, "ColonReplacementFormat")

    def test_ref_struct(self):
        prop = {"$ref": "#/components/schemas/Field"}
        t, _ = rust_type("message", prop, SPEC)
        self.assertEqual(t, "Field")

    def test_nullable_ref(self):
        prop = {"$ref": "#/components/schemas/Field", "nullable": True}
        t, _ = rust_type("message", prop, SPEC)
        self.assertEqual(t, "Option<Field>")


class TestFieldAttrs(unittest.TestCase):
    def test_id_gets_skip_serializing_if(self):
        attrs = field_attrs("id", {"type": "integer"}, SPEC, "Option<i32>")
        self.assertTrue(any("skip_serializing_if" in a for a in attrs))

    def test_multiword_gets_alias(self):
        attrs = field_attrs("renameMovies", {"type": "boolean"}, SPEC, "bool")
        self.assertTrue(any('alias = "rename_movies"' in a for a in attrs))

    def test_single_word_no_alias(self):
        attrs = field_attrs("name", {"type": "string", "nullable": True}, SPEC, "Option<String>")
        self.assertFalse(any("alias" in a for a in attrs))

    def test_bool_false_default(self):
        attrs = field_attrs("renameMovies", {"type": "boolean", "default": False}, SPEC, "bool")
        combined = " ".join(attrs)
        self.assertIn("default", combined)
        self.assertNotIn("default_true", combined)

    def test_bool_true_default(self):
        attrs = field_attrs("replaceIllegalCharacters", {"type": "boolean", "default": True}, SPEC, "bool")
        combined = " ".join(attrs)
        self.assertIn("default_true", combined)

    def test_readonly_gets_skip_serializing(self):
        attrs = field_attrs("implementationName", {"type": "string", "nullable": True, "readOnly": True}, SPEC, "Option<String>")
        self.assertTrue(any("skip_serializing" in a and "skip_serializing_if" not in a for a in attrs))

    def test_readonly_multiword_gets_alias_and_skip(self):
        attrs = field_attrs("implementationName", {"type": "string", "nullable": True, "readOnly": True}, SPEC, "Option<String>")
        combined = " ".join(attrs)
        self.assertIn("alias", combined)
        self.assertIn("skip_serializing", combined)


class TestCollectDefaultFns(unittest.TestCase):
    def test_generates_default_true(self):
        schema = SPEC["components"]["schemas"]["NamingResource"]
        fns = collect_default_fns(schema, SPEC)
        self.assertTrue(any("default_true" in f for f in fns))

    def test_generates_integer_default(self):
        schema = SPEC["components"]["schemas"]["IndexerResource"]
        fns = collect_default_fns(schema, SPEC)
        self.assertTrue(any("default_priority" in f and "25" in f for f in fns))

    def test_default_true_appears_once(self):
        schema = SPEC["components"]["schemas"]["NamingResource"]
        fns = collect_default_fns(schema, SPEC)
        true_fns = [f for f in fns if f == "fn default_true() -> bool { true }"]
        self.assertEqual(len(true_fns), 1)


class TestGenStruct(unittest.TestCase):
    def test_naming_struct_output(self):
        schema = SPEC["components"]["schemas"]["NamingResource"]
        out = gen_struct("NamingResource", schema, SPEC)
        self.assertIn("pub struct NamingResource", out)
        self.assertIn("pub id: Option<i32>", out)
        self.assertIn("pub rename_movies: bool", out)
        self.assertIn("pub standard_movie_format: Option<String>", out)

    def test_rename_all_camelcase(self):
        schema = SPEC["components"]["schemas"]["NamingResource"]
        out = gen_struct("NamingResource", schema, SPEC)
        self.assertIn('rename_all = "camelCase"', out)

    def test_derive_attrs(self):
        schema = SPEC["components"]["schemas"]["NamingResource"]
        out = gen_struct("NamingResource", schema, SPEC)
        self.assertIn("Debug", out)
        self.assertIn("Clone", out)
        self.assertIn("Serialize", out)
        self.assertIn("Deserialize", out)

    def test_serde_use_import(self):
        schema = SPEC["components"]["schemas"]["NamingResource"]
        out = gen_struct("NamingResource", schema, SPEC)
        self.assertIn("use serde", out)


class TestMainOutput(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False)
        json.dump(SPEC, self.tmp)
        self.tmp.close()

    def tearDown(self):
        os.unlink(self.tmp.name)

    def _run(self, ref):
        with patch("sys.argv", ["gen_resource_model.py", self.tmp.name, ref]):
            with patch("sys.stdout", new_callable=StringIO) as out:
                main()
                return out.getvalue()

    def test_naming_struct_generated(self):
        out = self._run("#/components/schemas/NamingResource")
        self.assertIn("pub struct NamingResource", out)

    def test_enum_generated(self):
        out = self._run("#/components/schemas/ColonReplacementFormat")
        self.assertIn("pub enum ColonReplacementFormat", out)
        self.assertIn("Delete", out)
        self.assertIn("Dash", out)

    def test_missing_schema_exits_nonzero(self):
        with patch("sys.argv", ["gen_resource_model.py", self.tmp.name, "#/components/schemas/DoesNotExist"]):
            with self.assertRaises(SystemExit) as cm:
                main()
            self.assertNotEqual(cm.exception.code, 0)

    def test_no_output_file_arg(self):
        # stdout only — main accepts exactly 2 args (spec + ref)
        import inspect
        import gen_resource_model
        sig = inspect.signature(gen_resource_model.main)
        self.assertEqual(len(sig.parameters), 0)


if __name__ == "__main__":
    unittest.main()
