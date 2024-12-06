#include <stdint.h>
#include <stdalign.h>
#include <features.h>
#include <sys/types.h>

typedef struct re_pattern_buffer {
	size_t re_nsub;
	void *__opaque, *__padding[4];
	size_t __nsub2;
	char __padding2;
} regex_t;

typedef int regoff_t;

typedef struct {
	regoff_t rm_so;
	regoff_t rm_eo;
} regmatch_t;

int regcomp(regex_t *__restrict, const char *__restrict, int);
int regexec(const regex_t *__restrict, const char *__restrict, size_t, regmatch_t *__restrict, int);
void regfree(regex_t *);

