from .linalg import *

__doc__ = linalg.__doc__
if hasattr(linalg, "__all__"):
    __all__: list[str] = linalg.__all__ # type: ignore

def run(): ...

# import xyz