inline int64_t decode() {
  uint64_t v;

  if ((v = *(position++) >= 0x80U)) {
    v = (v & 0x80U - 1) | (*(position++) << 7U);
    if (v >= 0x4000U) {
      v = (v & 0x4000U - 1) | (*(position++) << 14U);
      if (v >= 0x200000U) {
        v = (v & 0x200000U - 1) | (*(position++) << 21U);
        if (v >= 0x10000000U) {
          v = (v & 0x10000000U - 1) | (*(position++) << 28U);
          if (v >= 0x800000000U) {
            v = (v & 0x800000000U - 1) | (*(position++) << 35U);
            if (v >= 0x40000000000U) {
              v = (v & 0x40000000000U - 1) | (*(position++) << 42U);
              if (v >= 0x2000000000000U) {
                v = (v & 0x2000000000000U - 1) | (*(position++) << 49U);
                if (v >= 0x100000000000000U) {
                  v = (v & 0x100000000000000U - 1) | (*(position++)<<56U);
                }
              }
            }
          }
        }
      }
    }
  }
  return v;
}
