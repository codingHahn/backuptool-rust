# backuptool-rust

A simple tool to backup directory structures in pure Rust.
It uses Content-Defined-Chunking for cutting up files and support block-based deduplication.
Files and directories can be excluded using regex.
See `backuptool-rust -?` for help.

Files can either be backed up to a repository or restored from one.

Backup usage:
```
backuptool-rust backup files-to-backup/ backup-destination/
```

Restore usage:
```
backuptool-rust restore repository/ restore-destination/
```
