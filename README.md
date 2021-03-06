Working with the web in Rust
------------------------------------

The main thrust of this is to show the basics of working with Iron. Iron uses Hyper as it's http library, so it's good to know a bit about how that works as well. The hyper and hyper-client projects show what that's all about.

## Building

Recently, OSX has deprecated openssl in favor of apple's own TLS library. This means that openssl libs and headers may not be discovered automatically. If you get any errors compiling on OSX, try setting the environment variables: `OPENSSL_INCLUDE_DIR`, and `OPENSSL_LIB_DIR`. If you've installed openssl using homebrew (`brew install openssl`), then these should be:

    export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
    export OPENSSL_LIB_DIR=/usr/local/opt/openssl/lib

## Strengths of Iron

- Modular and extensible - It's got a lot of exposed wiring, so it's pretty easy to modify or extend it in whatever way you please.
- Light weight - Features are all opt-in by adding the dependency, so Iron itself if really pretty tiny.
- Fast - The examples here all run in a matter of microseconds on my macbook.
- Simple - There's no automagical features here. Unlike some of the big full-stack frameworks, its pretty easy to locate problems in your code.

### This is still very new

There are many things that you'd expect a framework to provide. Iron currently provides very few of them. Many of the features that we've come to expect out of web 'frameworks' either don't exist or are in very early stages of development. Additionally, many of the things that we expect to work out of the box with other frameworks must be configured manually with Iron. Don't let the word 'framework' fool you. To compare it to the Java world, Iron is much closer to a pure Servlet than it is to something like SpringMVC.

### Things that you have to configure manually

- Sessions - Actually managing the sessions is still pretty much up to you, even once sessions have been configured.
- Authentication - The crypto crate has rust implementations of most of the hashing algorithms you might need, but actually using them to authenticate a user is up to you.
- Body/Parameter parsing - The bodyparser crate will even convert a JSON body to an instance of your struct! There's also a crate for making arbitrary parameters available in a map.
- Serialization/Deserialization - Going between JSON and structs is pretty easy and well supported.
- Compression - Pretty simple to setup a gzip on all outgoing pages.
- Serving static files - This crate can also handle some basic caching using Cache-Control and If-Modified-Since headers.
- Logging - It's there, but it's definitely not as fancy as what you've probably come to expect from a web framework
- Content negotiation - Limited support, but the crate appears to be in active development.

### Things that are still missing or incomplete

- Database access - There's more options here than there used to be. Postgres and Redis both have good drivers in rust, and MongoDB isn't far off. There's even an ORM, Diesel. Compared to other languages, options are still rather limited.
- Authorization - You'll have to roll your own. The crypto crate has most of the main hashing algorithms, but actually calling them is up to you.
- Helpers for common configurations don't really exist. The Rustless project is taking a stab at it for REST APIs, but that's about it.
