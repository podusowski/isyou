import pytest
import subprocess
from pathlib import Path
import requests


def wait_until_healthy():
    while True:
        try:
            r = requests.get("http://localhost:8000/health")
            r.raise_for_status()
            break
        except requests.RequestException as e:
            print(e)


@pytest.fixture
def isyou_backend():
    cargo_root = Path(__file__).parent.parent
    process = subprocess.Popen(["cargo", "run"], cwd=cargo_root)
    try:
        wait_until_healthy()
        yield
    finally:
        process.terminate()
        process.wait()


def test_hello(isyou_backend):
    wait_until_healthy()
