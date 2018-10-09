uint8_t* encode(uint64_t value) {
  // calculate effective size
  int size = 0;
  {
    uint64_t buckets = value;
    while (buckets) {
      buckets >>= 7;
      size++;
    }
  }
  if (!size) {
    uint8_t[] result = new uint8_t[1];
    result[0] = 0;
    return result;
  } else if (10 == size)
    size = 9;
  // split
  uint8_t[] result = new uint8_t[size];
  int count = 0;
  for (; count < 8 && count < size - 1; count++) {
    result[count] = value >> (7 * count);
    result[count] |= 0x80;
  }
  result[count] = value >> (7 * count);
  return result;
}
