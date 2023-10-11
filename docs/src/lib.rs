// Dummy file for including the output of the build script.
// The purpose of this is to integrate building of the documentation
// into the build process of the backend.
// That way you can just build the backend and the documentation
// will seemlessly be built and embedded.

include!(concat!(env!("OUT_DIR"), "/lib.rs"));
