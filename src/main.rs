#![feature(test)]
#[cfg(test)]
extern crate test;

extern crate cpython;
extern crate crypto2;
extern crate openssl;

use crypto2::hash::sha256;
use cpython::{Python, PyBytes, PyObject, NoArgs, ObjectProtocol};


mod sz_64_bytes {
    use super::*;

    #[bench]
    fn bench_crypto2_sha256(b: &mut test::Bencher) {
        let data  = test::black_box([1u8; 64]);

        b.bytes = data.len() as u64;
        b.iter(|| {
            sha256(&data)
        })
    }

    #[bench]
    fn bench_openssl_sha256(b: &mut test::Bencher) {
        let data  = test::black_box([1u8; 64]);
        let m = openssl::hash::MessageDigest::sha256();

        b.bytes = data.len() as u64;
        b.iter(|| {
            openssl::hash::hash(m, &data).unwrap()
        })
    }

    #[bench]
    fn bench_py_builtin_sha256(b: &mut test::Bencher) {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let data  = test::black_box([1u8; 64]);
        // d = _sha256.sha256(b"asdasd")
        // d.digest()
        let sha256_obj = py.import("_sha256").unwrap().get(py, "sha256").unwrap();
        let bytes = PyBytes::new(py, &data);

        b.bytes = data.len() as u64;
        b.iter(|| {
            let args = (&bytes, );
            let obj = sha256_obj.call(py, args, None).unwrap();
            let digest: PyObject = obj.call_method(py, "digest", NoArgs, None).unwrap();

            digest
        })
    }

    #[bench]
    fn bench_py_openssl_sha256(b: &mut test::Bencher) {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let data  = test::black_box([1u8; 64]);
        // d = _hashlib.openssl_sha256(b"asdasd")
        // d.digest()
        let sha256_obj = py.import("_hashlib").unwrap().get(py, "openssl_sha256").unwrap();
        let bytes = PyBytes::new(py, &data);

        b.bytes = data.len() as u64;
        b.iter(|| {
            let args = (&bytes, );
            let obj = sha256_obj.call(py, args, None).unwrap();
            let digest: PyObject = obj.call_method(py, "digest", NoArgs, None).unwrap();

            digest
        })
    }
}



mod sz_10_mb {
    use super::*;
    
    #[bench]
    fn bench_crypto2_sha256(b: &mut test::Bencher) {
        let data = vec![1u8; 1024*1024*10];

        b.bytes = data.len() as u64;
        b.iter(|| {
            sha256(&data)
        })
    }

    #[bench]
    fn bench_openssl_sha256(b: &mut test::Bencher) {
        let data = vec![1u8; 1024*1024*10];
        let m = openssl::hash::MessageDigest::sha256();

        b.bytes = data.len() as u64;
        b.iter(|| {
            openssl::hash::hash(m, &data).unwrap()
        })
    }

    #[bench]
    fn bench_py_builtin_sha256(b: &mut test::Bencher) {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let data = vec![1u8; 1024*1024*10];
        // d = _sha256.sha256(b"asdasd")
        // d.digest()
        let sha256_obj = py.import("_sha256").unwrap().get(py, "sha256").unwrap();
        let bytes = PyBytes::new(py, &data);

        b.bytes = data.len() as u64;
        b.iter(|| {
            let args = (&bytes, );
            let obj = sha256_obj.call(py, args, None).unwrap();
            let digest: PyObject = obj.call_method(py, "digest", NoArgs, None).unwrap();

            digest
        })
    }

    #[bench]
    fn bench_py_openssl_sha256(b: &mut test::Bencher) {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let data = vec![1u8; 1024*1024*10];
        // d = _hashlib.openssl_sha256(b"asdasd")
        // d.digest()
        let sha256_obj = py.import("_hashlib").unwrap().get(py, "openssl_sha256").unwrap();
        let bytes = PyBytes::new(py, &data);

        b.bytes = data.len() as u64;
        b.iter(|| {
            let args = (&bytes, );
            let obj = sha256_obj.call(py, args, None).unwrap();
            let digest: PyObject = obj.call_method(py, "digest", NoArgs, None).unwrap();

            digest
        })
    }
}

fn main() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let data  = [1u8; 64];
    let bytes = PyBytes::new(py, &data);
    let args  = (bytes, );

    let sha256_obj = py.import("_hashlib").unwrap().get(py, "openssl_sha256").unwrap();
    let obj = sha256_obj.call(py, args, None).unwrap();
    let digest: PyObject = obj.call_method(py, "digest", NoArgs, None).unwrap();

    println!("{:?}", digest);
}