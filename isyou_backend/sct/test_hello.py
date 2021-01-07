import pytest
import subprocess
from pathlib import Path
import requests


@pytest.fixture
def isyou_backend():
    cargo_root = Path(__file__).parent.parent
    process = subprocess.Popen(["cargo", "run"], cwd=cargo_root)
    try:
        yield
    finally:
        process.terminate()
        process.wait()


def wait_until_healthy():
    while True:
        try:
            r = requests.get("http://localhost:8000/health")
            r.raise_for_status()
            break
        except Exception as e:
            print(e)


def test_hello(isyou_backend):
    wait_until_healthy()
