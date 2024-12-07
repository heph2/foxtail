# FoxTail

An agnostic Concurrent Job Queue which implements different "persistance" backend

## In Memory Queue

Currently Implementing it...

It Use Arc and Mutex around VedDeque for locking and safe concurrent
queue operations.

Obviously it doesn't assure any kind of persistance. It useful for
testing purposes.

## SQLite Backend

This use an embeded Sqlite for assure persistance.
