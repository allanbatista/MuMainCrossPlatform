#!/usr/bin/env python3
"""Package the Rust client release bundle.

Usage:
    python3 scripts/package_rust_client.py \
        --client-exe port_rust/target/release/mu_client.exe \
        --asset-root port_rust/assets \
        --output-dir dist/mu-client
"""

from __future__ import annotations

import argparse
import shutil
import sys
import tempfile
import unittest
from pathlib import Path


MANIFEST_FILE_NAME = "manifest.muasset.json"
DEFAULT_ASSET_DIR_NAME = "assets"


def package_rust_client(
    client_exe: Path,
    asset_root: Path,
    output_dir: Path,
    asset_dir_name: str = DEFAULT_ASSET_DIR_NAME,
) -> Path:
    _require_file(client_exe, "client exe")
    _require_directory(asset_root, "asset root")
    _require_file(asset_root / MANIFEST_FILE_NAME, "asset manifest")

    if output_dir.exists():
        shutil.rmtree(output_dir)

    output_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy2(client_exe, output_dir / client_exe.name)
    shutil.copytree(asset_root, output_dir / asset_dir_name)

    return output_dir


def _require_file(path: Path, label: str) -> None:
    if path.is_file():
        return

    raise FileNotFoundError(f"{label} not found: {path}")


def _require_directory(path: Path, label: str) -> None:
    if path.is_dir():
        return

    raise FileNotFoundError(f"{label} not found: {path}")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--client-exe", type=Path, required=True)
    parser.add_argument("--asset-root", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument(
        "--asset-dir-name",
        default=DEFAULT_ASSET_DIR_NAME,
        help="directory name used for the staged asset root",
    )
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    bundle_dir = package_rust_client(
        client_exe=args.client_exe,
        asset_root=args.asset_root,
        output_dir=args.output_dir,
        asset_dir_name=args.asset_dir_name,
    )
    print(bundle_dir)
    return 0


class PackageRustClientTests(unittest.TestCase):
    def test_packages_client_exe_and_assets(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            client_exe = root / "mu_client.exe"
            client_exe.write_bytes(b"client-bytes")

            asset_root = root / "assets"
            (asset_root / "converted").mkdir(parents=True)
            (asset_root / MANIFEST_FILE_NAME).write_text("{}", encoding="utf-8")
            (asset_root / "converted" / "boot.bin").write_bytes(b"boot-bytes")

            output_dir = root / "dist"
            bundle_dir = package_rust_client(client_exe, asset_root, output_dir)

            self.assertEqual(bundle_dir, output_dir)
            self.assertEqual((bundle_dir / client_exe.name).read_bytes(), b"client-bytes")
            self.assertEqual(
                (bundle_dir / DEFAULT_ASSET_DIR_NAME / MANIFEST_FILE_NAME).read_text(
                    encoding="utf-8"
                ),
                "{}",
            )
            self.assertEqual(
                (bundle_dir / DEFAULT_ASSET_DIR_NAME / "converted" / "boot.bin").read_bytes(),
                b"boot-bytes",
            )

    def test_existing_output_directory_is_replaced(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            client_exe = root / "mu_client.exe"
            client_exe.write_bytes(b"client-bytes")

            asset_root = root / "assets"
            asset_root.mkdir()
            (asset_root / MANIFEST_FILE_NAME).write_text("{}", encoding="utf-8")

            output_dir = root / "dist"
            output_dir.mkdir()
            (output_dir / "stale.txt").write_text("stale", encoding="utf-8")

            package_rust_client(client_exe, asset_root, output_dir)

            self.assertFalse((output_dir / "stale.txt").exists())
            self.assertTrue((output_dir / client_exe.name).exists())

    def test_missing_manifest_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            client_exe = root / "mu_client.exe"
            client_exe.write_bytes(b"client-bytes")

            asset_root = root / "assets"
            asset_root.mkdir()

            with self.assertRaises(FileNotFoundError):
                package_rust_client(client_exe, asset_root, root / "dist")


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
