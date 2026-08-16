import importlib.metadata
import unittest
from unittest.mock import patch

from tools import check_host


class HostCheckTests(unittest.TestCase):
    def test_command_checks_report_missing_tool(self):
        found = {"cmake": "/bin/cmake", "clang": None, "clang++": "/bin/clang++", "lld-link": "/bin/lld-link"}
        checks = check_host.command_checks(found.get)
        self.assertFalse(next(check for check in checks if check.name == "command:clang").ok)

    def test_package_checks_report_installed_and_missing(self):
        def version(name):
            if name == "pefile":
                raise importlib.metadata.PackageNotFoundError(name)
            return "1.0"

        checks = check_host.package_checks(version)
        self.assertFalse(next(check for check in checks if check.name == "python:pefile").ok)
        self.assertTrue(next(check for check in checks if check.name == "python:pyxbe").ok)

    def test_compile_probe_passes_expected_target(self):
        class Result:
            returncode = 0
            stdout = ""
            stderr = ""

        calls = []

        def run(command, **kwargs):
            calls.append((command, kwargs))
            return Result()

        check = check_host.compile_probe(run)
        self.assertTrue(check.ok)
        self.assertIn("i386-pc-win32", calls[0][0])
        self.assertIn("-fuse-ld=lld-link", calls[0][0])

    @patch.object(check_host, "package_checks")
    @patch.object(check_host, "command_checks")
    def test_missing_prerequisite_skips_compile(self, commands, packages):
        commands.return_value = [check_host.Check("command:clang", False, "missing")]
        packages.return_value = []
        report = check_host.inspect_host()
        self.assertFalse(report["valid_build_host"])
        self.assertEqual(report["checks"][-1]["detail"], "skipped: prerequisites missing")


if __name__ == "__main__":
    unittest.main()
