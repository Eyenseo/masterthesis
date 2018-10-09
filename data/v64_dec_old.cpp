uint64_t decode(uint8_t* v64) {
  int count = 0;
  uint64_t result = 0;
  register uint64_t bucket;
  for (; count < 8 && (*v64) & 0x80; count++, v64++) {
    bucket = v64[0];
    result |= (bucket & 0x7f) << (7 * count);
  }
  bucket = v64[0];
  result |= (8 == count ? bucket : (bucket & 0x7f)) << (7 * count);
  return result;
}
