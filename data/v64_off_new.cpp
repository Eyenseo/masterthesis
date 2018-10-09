uint64_t size(uint64_t v) {
  return (v < 0x80U)
        ? 1
      // ...
      : ((v < 0x100000000000000U)
        ? 8
      : 9);
}
