
Bench
--------

.. code:: bash
	
	git clone https://github.com/LuoZijun/test_py_hashlib.git
	cd test_py_hashlib
	
	# NOTE：如果你的硬件没有 SHA-NI 指令，那么移除 `+sha` 即可。
	env RUSTFLAGS="-C target-feature=+aes,+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+avx,+avx2,+sha" cargo bench