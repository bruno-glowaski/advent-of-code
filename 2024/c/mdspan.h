#include <stdint.h>

#define MDSPAN(_T)                                                             \
  struct {                                                                     \
    _T *buffer;                                                                \
    dim_t dimensions;                                                          \
    idx_t stride;                                                              \
  }

typedef intptr_t idx_t;
typedef idx_t dim_t[2];

#define MD_IN(_x, _y, _dimensions)                                             \
  ((_x) >= 0 && (_y) >= 0 && (_x) < (_dimensions)[0] && (_y) < (_dimensions)[1])
#define MD_CM_MAPPING(_stride, _x, _y) ((_y) * (_stride) + (_x))
#define MD_CM_BUFLEN(_span) ((_span).dimensions[1] * (_span).stride)
#define MD_CM_GET(_span, _x, _y)                                               \
  ((_span).buffer[MD_CM_MAPPING((_span).stride, (_x), (_y))])

#define DECLARE_MDSPAN_SEARCH(_T, _func_name)                                  \
  static inline void _func_name(_T map, idx_t *x, idx_t *y, char c) {          \
    for_all_points_cm(*x, *y, map.dimensions) {                                \
      if (MD_CM_GET(map, *x, *y) == c) {                                       \
        break;                                                                 \
      }                                                                        \
    }                                                                          \
  }

#define for_all_points_cm(_x, _y, _dimensions)                                 \
  for (; (_y) < (_dimensions)[1]; (_y) += ((_x) + 1) / (_dimensions)[0],       \
                                  (_x) = ((_x) + 1) % (_dimensions)[0])
