# git-hook-commit-msg

Rust app to check commits follows conventional commit standard.

That seems like a pretty silly thing to do, but portability between OSes has
become a big deal to me and having a single binary that performs the most
important check in my git workflow as quickly as possible is a part of that.

## Usage

This is intended to just be dropped in a global git hook config

```bash
mkdir ~/.githooks
cp git-hook-commit-msg ~/.githooks/commit-msg
git config --global core.hoohsPath=~/.githooks
```

You can do this on any OS including the dreaded windows using the built
artifact from project.

I am not publishing this to cargo or anything like that just yet since but
I may do so in future.
