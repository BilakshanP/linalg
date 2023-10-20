from .linalg import *

__doc__ = linalg.__doc__
if hasattr(linalg, "__all__"):
    __all__ = linalg.__all__ # type: ignore

# import xyz