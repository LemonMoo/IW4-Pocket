#!/usr/bin/env python3
"""Source-level contract checks. These do NOT compile or run Rust."""
from pathlib import Path
import plistlib
import re
import unittest

ROOT = Path(__file__).resolve().parents[1]


def text(path):
    return (ROOT / path).read_text()


class Contract(unittest.TestCase):
    def test_versions_agree(self):
        cargo = re.search(r'^version = "([^"]+)"$', text("crates/launcher/Cargo.toml"), re.M).group(1)
        plist = plistlib.loads((ROOT / "ios/Info.plist").read_bytes())
        self.assertEqual(plist["CFBundleShortVersionString"], cargo)
        self.assertEqual(plist["CFBundleIdentifier"], "com.markussela.iw4l")
        lock = re.search(r'name = "launcher"\nversion = "([^"]+)"', text("Cargo.lock"))
        self.assertIsNotNone(lock)
        self.assertEqual(lock.group(1), cargo)

    def test_markers_exist_in_sources(self):
        sources = "\n".join(p.read_text() for p in (ROOT / "crates").rglob("*.rs"))
        verify = text("tools/verify_ios_ipa.py")
        for marker in re.findall(r'b"([^"]+)"', verify.split("MARKERS", 1)[1].split(")", 1)[0]):
            self.assertIn(marker, sources, marker)

    def test_env_allowlist_is_closed(self):
        src = text("crates/diag/src/memory_settings.rs")
        body = src.split("pub fn valid_file_setting", 1)[1].split("#[cfg(test)]", 1)[0]
        self.assertIn("_ => false", body)
        for name in ("IW4L_GAMES", "IW4L_ARTIFACTS_DIR"):
            self.assertNotIn(f'"{name}"', body)

    def test_ios_defaults_are_conservative(self):
        src = text("crates/diag/src/memory_settings.rs")
        self.assertIn("IW4L_FPV_RETAIN_MIB", src)
        self.assertIn("IW4L_SHADER_WORKERS", src)

    def test_workflow_verifies_before_publish(self):
        wf = text(".github/workflows/ios-release.yml")
        verify = wf.index("tools/verify_ios_ipa.py")
        self.assertLess(verify, wf.index("actions/upload-artifact"))
        self.assertLess(verify, wf.index("gh release create"))
        self.assertIn("--profile ios", wf)
        self.assertIn("$GITHUB_SHA", wf)

    def test_no_secret_shaped_strings(self):
        pattern = re.compile(r"gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{30,}|AKIA[0-9A-Z]{16}")
        for rel in ("tools", ".github", "crates/diag", "crates/launcher", "RELEASE_NOTES.md"):
            base = ROOT / rel
            files = [base] if base.is_file() else [p for p in base.rglob("*") if p.is_file() and p.suffix in {".py", ".yml", ".rs", ".md"}]
            for p in files:
                self.assertIsNone(pattern.search(p.read_text()), str(p))


if __name__ == "__main__":
    unittest.main(verbosity=2)
