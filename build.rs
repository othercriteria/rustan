extern crate gcc;

fn main() {
    gcc::Config::new()
                .file("stan/math/stan/math.hpp")
                .define("EIGEN_NO_DEBUG", None)
                .include("stan/stan/src")
                .include("stan/math")
                .include("stan/math/lib/eigen_3.2.4")
                .include("stan/math/lib/boost_1.58.0")
                .compile("libmath.a");
}
