# netcore-rs
A simple little wrapper for starting net core and calling into it.

# Requirements
* Rust of course.
* DotNetCore developer SDK.

# Status
At the moment the system is extremely simple and single purpose.  "build.rs" will build the original SDK test C# project and publish it.  The library test will then load this up and run the same tests as the C++ example.  Everything seems to be working.
