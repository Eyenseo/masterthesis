inline void encode(uint64_t v) {
  if (v < 0x80U) {
    *(position++) = (uint8_t)(v);
  } else {
    *(position++) = (uint8_t)(0x80U | v);
    if (v < 0x4000U) {
      *(position++) = (uint8_t)(v >> 7U);
    } else {
      *(position++) = (uint8_t)(0x80U | v >> 7U);
      if (v < 0x200000U) {
        *(position++) = (uint8_t)(v >> 14U);
      } else {
        *(position++) = (uint8_t)(0x80U | v >> 14U);
        if (v < 0x10000000U) {
          *(position++) = (uint8_t)(v >> 21U);
        } else {
          *(position++) = (uint8_t)(0x80U | v >> 21U);
          if (v < 0x800000000U) {
            *(position++) = (uint8_t)(v >> 28U);
          } else {
            *(position++) = (uint8_t)(0x80U | v >> 28U);
            if (v < 0x40000000000U) {
              *(position++) = (uint8_t)(v >> 35U);
            } else {
              *(position++) = (uint8_t)(0x80U | v >> 35U);
              if (v < 0x2000000000000U) {
                *(position++) = (uint8_t)(v >> 42U);
              } else {
                *(position++) = (uint8_t)(0x80U | v >> 42U);
                if (v < 0x100000000000000U) {
                  *(position++) = (uint8_t)(v >> 49U);
                } else {
                  *(position++) = (uint8_t)(0x80U | v >> 49U);
                  *(position++) = (uint8_t)(v >> 56U);
                }
              }
            }
          }
        }
      }
    }
  }
}
