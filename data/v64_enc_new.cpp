void encode(uint64_t v) {
  if (v < 0x80U) {
    *(position++) = (uint8_t)(v);
  } else {
    *(position++) = (uint8_t)(0x80U | v);
    // ...
                if (v < 0x100000000000000U) {
                  *(position++) = (uint8_t)(v >> 49U);
                } else {
                  *(position++) = (uint8_t)(0x80U | v >> 49U);
                  *(position++) = (uint8_t)(v >> 56U);
                }
    // ...
  }
}
