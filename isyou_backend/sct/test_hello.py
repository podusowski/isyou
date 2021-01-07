import pytest
import subprocess
from pathlib import Path


@pytest.fixture
def isyou_backend():
    cargo_root = Path(__file__).parent.parent
    process = subprocess.Popen(["cargo", "run"], cwd=cargo_root)
    yield
    import time
    time.sleep(0.5)
    process.terminate()
    process.wait()


def test_hello(isyou_backend):
    pass
