* Impl `Display` for Units
* `PacketReader` and `PacketWriter` should return `Result<T, GameError` or any kind of error type.
    * Get rid of `catch_all` by returning a read error instead
* `UnitFlattiverseEvent`: "The connector clones the unit when the event is created, so this object does not track later
  live updates."
* impl `Readable` and `Writable` for `*Id` types
    * impl `Readable` and `Writable` for all types? like `let x = 0.0; x.read(reader); x.write(writer);` ??? maybe
    * Maybe `PacketReader::read<T: Readable>() -> T` ??
* `Atomic::store_or_default(bool, T)` ??