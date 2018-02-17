from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(name="pyrustbio",
      version='0.1',
      rust_extensions=[RustExtension('pyrustbio._pyrustbio',
                                     'Cargo.toml',
                                     binding=Binding.PyO3)],
      packages=['pyrustbio'],
      zip_safe=False
     )
