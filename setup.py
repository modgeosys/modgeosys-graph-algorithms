from setuptools import setup
from setuptools_rust import RustExtension

setup(name="astar_module",
      version="0.1",
      rust_extensions=[RustExtension("astar_module.astar_module", binding=pyo3)],
      packages=["astar_module"],
      zip_safe=False)