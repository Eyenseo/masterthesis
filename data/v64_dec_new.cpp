int64_t decode() {
  uint64_t v;

  if ((v = *(position++) >= 0x80U)) {
    v = (v & 0x80U - 1) | (*(position++) << 7U);
    // ...
                if (v >= 0x100000000000000U) {
                  v = (v & 0x100000000000000U - 1) | (*(position++)<<56U);
                }
    // ...
  }
  return v;
}
