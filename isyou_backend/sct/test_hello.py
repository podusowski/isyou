from typing import Any
import pytest
import subprocess
from pathlib import Path
import requests
import time


CARGO_ROOT = Path(__file__).parent.parent
"""Root directory of isyou_backend."""

PROJECT_ROOT = CARGO_ROOT.parent


@pytest.fixture
def isyou_backend_build():
    subprocess.check_call(["cargo", "build", "--release"], cwd=CARGO_ROOT)


def wait_until_healthy():
    while True:
        try:
            r = requests.get("http://localhost:8000/health")
            r.raise_for_status()
            break
        except requests.RequestException as e:
            print(e)
            time.sleep(1)


@pytest.fixture
def isyou_docker_backend(isyou_backend_build):
    process = subprocess.Popen(["docker-compose", "up", "--build"], cwd=PROJECT_ROOT)
    try:
        wait_until_healthy()
        yield
    finally:
        process.terminate()
        process.wait()


class IsyouBackend:
    def get_json(self, uri: str) -> Any:
        r = requests.get(f"http://localhost:8000/{uri}")
        r.raise_for_status()
        return r.json()

    def post(self, uri: str) -> None:
        r = requests.post(f"http://localhost:8000/{uri}")
        r.raise_for_status()


@pytest.fixture
def isyou_backend() -> IsyouBackend:
    process = subprocess.Popen(["cargo", "run"], cwd=CARGO_ROOT)
    try:
        wait_until_healthy()
        yield IsyouBackend()
    finally:
        process.terminate()
        process.wait()


def test_hello(isyou_backend):
    wait_until_healthy()


def test_seeks_are_empty_on_startup(isyou_backend: IsyouBackend):
    seeks = requests.get("http://localhost:8000/seeks").json()
    assert [] == seeks


def test_creating_new_seek(isyou_backend: IsyouBackend):
    # create a seek
    isyou_backend.post("/seeks")

    assert [1] == isyou_backend.get_json("/seeks")
    assert [] == isyou_backend.get_json("/seeks/1/points")

    # put some point
    isyou_backend.post("/seeks/1/points")
    assert [{"lat": 1, "lon": 1}] == isyou_backend.get_json("/seeks/1/points")

    # create another one
    isyou_backend.post("/seeks")
    assert [1, 2] == isyou_backend.get_json("/seeks")
    assert [] == isyou_backend.get_json("/seeks/2/points")
