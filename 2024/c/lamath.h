#include <stdlib.h>
#include <string.h>

#define DECLARE_VEC(_T, _dims, _ident) typedef _T _ident[_dims];

#define DECLARE_VEC_COPY(_TVec)                                                \
  static inline void _TVec##_copy(_TVec dest, _TVec const src) {               \
    memcpy(dest, src, sizeof(_TVec));                                          \
  }

#define DECLARE_VEC_EQ(_TVec, _dims)                                           \
  static inline bool _TVec##_eq(_TVec const a, _TVec const b) {                \
    for (size_t i = 0; i < _dims; i++) {                                       \
      if (a[i] != b[i]) {                                                      \
        return false;                                                          \
      }                                                                        \
    }                                                                          \
    return true;                                                               \
  }

#define DECLARE_VEC_UN_OP(_TVec, _dims, _op, _ident)                           \
  static inline void _TVec##_##_ident(_TVec out, _TVec const in) {             \
    for (size_t i = 0; i < _dims; i++) {                                       \
      out[i] = _op(in[i]);                                                     \
    }                                                                          \
  }

#define DECLARE_VEC_SCALAR_OP(_TVec, _TScalar, _dims, _op, _ident)             \
  static inline void _TVec##_##_ident(_TVec out, _TVec const a, _TScalar b) {  \
    for (size_t i = 0; i < _dims; i++) {                                       \
      out[i] = a[i] _op b;                                                     \
    }                                                                          \
  }

#define DECLARE_VEC_BIN_OP(_TVec, _dims, _op, _ident)                          \
  static inline void _TVec##_##_ident(_TVec out, _TVec const a,                \
                                      _TVec const b) {                         \
    for (size_t i = 0; i < _dims; i++) {                                       \
      out[i] = a[i] _op b[i];                                                  \
    }                                                                          \
  }

#define DECLARE_VEC_NEG(_TVec, _dims) DECLARE_VEC_UN_OP(_TVec, _dims, -, neg)

#define DECLARE_VEC_ADD(_TVec, _dims) DECLARE_VEC_BIN_OP(_TVec, _dims, +, add)
#define DECLARE_VEC_SUB(_TVec, _dims) DECLARE_VEC_BIN_OP(_TVec, _dims, -, sub)

#define DECLARE_VEC_SMUL(_TVec, _TScalar, _dims)                               \
  DECLARE_VEC_SCALAR_OP(_TVec, _TScalar, _dims, *, smul)
#define DECLARE_VEC_SDIV(_TVec, _TScalar, _dims)                               \
  DECLARE_VEC_SCALAR_OP(_TVec, _TScalar, _dims, /, sdiv)

// static inline void float2_add(float2 const a, float2 const b, float2 dest) {
//   dest[0] = a[0] + b[0];
//   dest[1] = a[1] + b[1];
// }
// static inline void float2_sub(float2 const a, float2 const b, float2 dest) {
//   dest[0] = a[0] - b[0];
//   dest[1] = a[1] - b[1];
// }
// static inline void float2_neg(float2 const src, float2 dest) {
//   dest[0] = -src[0];
//   dest[1] = -src[1];
// }
// static inline void float2_to_long2(float2 const src, long2 dest) {
//   dest[0] = (int64_t)src[0];
//   dest[1] = (int64_t)src[1];
// }
