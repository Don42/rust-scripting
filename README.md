# Rust Scripting
[![Build Status](https://travis-ci.org/Don42/rust-scripting.svg?branch=master)](https://travis-ci.org/Don42rust-scripting)

I had to do some bulk processing of files. Ususally I would have written a Bash
script or build something in Python. But this time I wanted to test how much
more overhead it would be to finish the task with Rust.

The first task was extracting downloaded archives from bandcamp as a folder
structure. That took some time because I had to get into the Rust file system API.
The result looks good even for a first try and improving the error handling
should be relatively easy.

The second task was taking flac files from my music library and converting them
to MP3 so I could listen to them in my car. The additional requirement was to
keep the file structure, not litter the converted files all over the library,
and keep the ID3 tags on the files.

For both cases I wanted to to use unix utils for the heavy lifting to test how
difficult their are to integrate in Rust.

For now the results look promising. Even without extra abstractions it's quite
easy and relatively quick to write small automation tools/scripts.
With some abstractions for common jobs it might get even easier.
One large disadvantage is that the tool needs to be compiled. This isn't such
an issue on a development system, but is a bit annoying when working on a system
remotely. Since most severs don't have the Rust toolchain installed. So this
makes only sense for automation tools that will be used/extended for some time.
One off scripts should probably still be implemented in Bash.

## License
Haven't decided on a license yet.
