import sys
from pathlib import Path

from ._core import *


def _folder():
    """The folder of the running solution, wherever it was started from."""
    return Path(sys.modules["__main__"].__file__).parent


def read_input():
    """The contents of the solution's input.txt."""
    return (_folder() / "input.txt").read_text()


def write_output(answer):
    """Print the answer and save it to the solution's output.txt."""
    print(answer)
    (_folder() / "output.txt").write_text(f"{answer}\n")
