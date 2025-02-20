Getting Started
===============

This section will walk you through the process of getting started with OpenDP. By the end, you should have a working OpenDP installation, and you'll be ready to explore OpenDP programming.

Installation
------------

OpenDP is built with a layered architecture. The core consists of a native library, implemented in Rust. On top of this core are bindings for using OpenDP from other languages (currently only Python, but we hope to add bindings for more languages in the future).

In order to use OpenDP in your applications, the first step is installing the library so that it's accessible to your local environment. Depending on how you intend to use OpenDP, these steps may vary.

Platforms
^^^^^^^^^

OpenDP is built for the following platforms:

* Python 3.7-3.10
* Linux, x86 64-bit, versions compatible with `manylinux <https://github.com/pypa/manylinux>`_ (others may work)
* macOS, x86 64-bit, version 10.13 or later
* Windows, x86 64-bit, version 7 or later

Other platforms may work, but will require `building from source <#building-opendp-from-source>`_.

Installing OpenDP for Python from PyPI
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

OpenDP is distributed as a `Python package on PyPI <https://pypi.org/project/opendp/>`_. You can install it using ``pip`` or another package management tool:

.. prompt:: bash

    pip install opendp

.. note::

    The OpenDP Python package contains binaries for all supported platforms, so you don't need to worry about specifying a specific platform.

At this point, you should be good to go! You can confirm your installation in Python by importing the top-level module ``opendp``:

.. doctest::

    >>> import opendp

Installing OpenDP for Rust from crates.io
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

OpenDP is also available as a `Rust crate <https://crates.io/crates/opendp>`_.
This is not as common, as most people will use OpenDP through one of the other language bindings.
But if you need to use the Rust interface of OpenDP directly, you can just reference the ``opendp`` crate in your ``Cargo.toml`` file:

.. code-block:: toml

    [dependencies]
    opendp = { version = "0.1.0", features = ["contrib"] }

.. note::

    The actual version may differ depending on the `releases available <https://github.com/opendp/opendp/releases>`_.

In the above snip, opting into the "contrib" feature includes code that has not yet completed the vetting process.
Continuous noise samplers require explicit opt-in and are behind the "floating-point" feature.

With that configured, the Rust dependency system will automatically download the crate as needed, and you can just ``use`` the ``opendp`` module:

.. code-block:: rust

    use opendp::core::*;
    // OpenDP code goes here!

Building OpenDP from Source
^^^^^^^^^^^^^^^^^^^^^^^^^^^

Under special circumstances, it may be necessary to install OpenDP directly from the source files.
This is only required if you want to build OpenDP from scratch, or if you're interested in :doc:`contributing to OpenDP <../developer/index>`.

There is a thorough guide to building from source in the :doc:`Development Environment <../developer/development-environment>` documentation.


.. _hello-opendp:

Hello, OpenDP!
--------------

Once you've installed OpenDP, you can write your first program.

Be aware that the vetting process is currently underway for the code in the OpenDP Library.
Any code that has not completed the vetting process is marked as "contrib" and will not run unless you opt-in.
Enable ``contrib`` globally with the following snippet:

.. doctest::

    >>> from opendp.mod import enable_features
    >>> enable_features('contrib')

In the example below, we'll construct a ``Transformation``, which is an OpenDP object that transforms data in some way.
In this case, the operation it performs is the identity transformation -- so no transformation at all!
Then we'll apply that transformation to a vector consisting of one string, and get back a copy of the vector.

.. doctest::

    >>> from opendp.transformations import make_identity
    >>> from opendp.typing import VectorDomain, AllDomain, SymmetricDistance
    ...
    >>> identity = make_identity(D=VectorDomain[AllDomain[str]], M=SymmetricDistance)
    >>> identity(["Hello, world!"])
    ['Hello, world!']

First, we import some types to have them in scope.
:func:`make_identity <opendp.transformations.make_identity>` is a :ref:`constructor function <constructors>`,
and the imports from :mod:`opendp.typing` are necessary for disambiguating the types the transformation will work with.

Next we call ``make_identity()`` to construct an identity ``Transformation``.
Because OpenDP is statically typed (even when called from dynamically typed languages like Python), we need to specify some type information.
This is done by supplying some key-value arguments.
``D=VectorDomain[AllDomain[str]]`` says that we want the ``Transformation`` to have an input and output :ref:`Domain <domains>` consisting of all string vectors,
and ``M=SymmetricDistance`` says that we want the resulting ``Transformation`` to use the OpenDP type ``SymmetricDistance`` for its input and output :ref:`Metric <metrics>`.

Finally, we invoke our ``identity`` transformation by calling it like a function on a string vector. As expected, it returns the same string vector back to us!

That's not particularly exciting, but it shows the rudiments of an OpenDP program.
Don't worry if some of the concepts don't make sense because they'll be explained later in this guide.

What's Next?
------------

Now that you've had a taste of OpenDP, you can start exploring the library in more depth.
The remainder of this guide will walk you through the concepts that underlie OpenDP,
starting with its conceptual underpinnings, known as the :doc:`OpenDP Programming Framework <programming-framework>`.

If you're eager to just jump in with programming, you can look at some of the :doc:`example uses of OpenDP <../examples>`.

For those who prefer to study reference material, you can consult the :doc:`API Docs <../api/index>`.
