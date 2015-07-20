Working with the web in Rust
------------------------------------

The main thrust of this is to show the basics of working with Iron. Iron uses Hyper as it's http library, so it's good to know a bit about how that works as well. The hyper and hyper-client projects show what that's all about.

## Strengths of Iron

- Modular and extensible - It's got a lot of exposed wiring, so it's pretty easy to modify or extend it in whatever way you please.
- Light weight - Features are all opt-in by adding the dependency, so Iron itself if really pretty tiny.
- Fast - The examples here all run in a matter of microseconds on my macbook.
- Simple - There's no automagical features here. Unlike some of the big full-stack frameworks, its pretty easy to locate problems in your code.

#### This is still very new

There are many things that you'd expect a framework to provide. Iron currently provides very few of them. Many of the features that we've come to expect out of web 'frameworks' either don't exist or are in very early stages of development. Additionally, many of the things that we expect to work out of the box with other frameworks must be configured manually with Iron. Don't let the word 'framework' fool you. To compare it to the Java world, Iron is much closer to a pure Servelet than it is to something like SpringMVC.

### Things that you have to configure manually

- Sessions - Actually managing the sessions is still pretty much up to you, even once sessions have been configured.
- Authentication - The crypto crate has rust implementations of most of the hashing algorithms you might need, but actually using them to authenticate a user is up to you.
- Compression - Pretty simple to setup a gzip on all outgoing pages.
- ETags - there's a crate for that
- Logging - It's there, but it's definitely not as fancy as what you've probably come to expect from a web framework
- Parsing request bodies - There is a crate for parsing JSON bodies, but nothing Iron-specific for XML.
- Content negotiation - Limited support, but the crate appears to be in active development.

### Things that are still missing or incomplete

- Database access - Crates.io doesn't offer much in terms of database drivers. Iron does nothing to help you with that. Your database options will be limited.
- Authorization - You'll have to roll your own.
- Helpers for common configurations don't really exist. The Rustless project is taking a stab at it for REST APIs, but that's about it.
- Multipart requests - There's a crate for this, but it looks really immature.
