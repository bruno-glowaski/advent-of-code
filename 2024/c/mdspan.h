#include <stdbool.h>
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

#define MD_VIN(_vec, _dimensions) MD_IN((_vec)[0], (_vec)[1], _dimensions)

#define MD_CM_MAPPING(_stride, _x, _y) ((_y) * (_stride) + (_x))

#define MD_CM_GET_RAW(_arr, _stride, _x, _y)                                   \
  ((_arr)[MD_CM_MAPPING((_stride), (_x), (_y))])

#define MD_CM_GET(_span, _x, _y)                                               \
  MD_CM_GET_RAW((_span).buffer, (_span).stride, _x, _y)

#define MD_CM_VGET(_span, _vec) MD_CM_GET(_span, (_vec)[0], (_vec)[1])

#define MD_CM_BUFLEN(_span) ((_span).dimensions[1] * (_span).stride)

#define DECLARE_MDSPAN_SEARCH(_T, _func_name)                                  \
  static inline bool _func_name(_T _map, idx_t *x, idx_t *y, char c) {         \
    for_all_points_cm(*x, *y, (_map).dimensions) {                             \
      if (MD_CM_GET(_map, *x, *y) == c) {                                      \
        return true;                                                           \
      }                                                                        \
    }                                                                          \
    return false;                                                              \
  }

#define next_point_cm(_x, _y, _dimensions)                                     \
  (_y) += ((_x) + 1) / (_dimensions)[0], (_x) = ((_x) + 1) % (_dimensions)[0]

#define next_vpoint_cm(_vec, _dimensions)                                      \
  next_point_cm((_vec)[0], (_vec)[1], _dimensions)

#define for_all_points_cm(_x, _y, _dimensions)                                 \
  for (; (_y) < (_dimensions)[1]; next_point_cm(_x, _y, _dimensions))

#define for_all_vpoints_cm(_vec, _dimensions)                                  \
  for_all_points_cm((_vec)[0], (_vec)[1], _dimensions)
