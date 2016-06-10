// RUN: %clang_cc1 -fsyntax-only -verify -x renderscript -D__RENDERSCRIPT__ %s
// RUN: %clang_cc1 -fsyntax-only -verify -x c %s

#ifndef __RENDERSCRIPT__
// expected-warning@+2 {{kernel attribute ignored}}
#endif
void __attribute__((kernel)) kernel();

// expected-warning@+1 {{'kernel' attribute only applies to functions}}
int __attribute__((kernel)) global;

#ifndef __RENDERSCRIPT__
// expected-error@+2 {{function return value cannot have __fp16 type; did you forget * ?}}
#endif
__fp16 fp16_return();

#ifndef __RENDERSCRIPT__
// expected-error@+2 {{parameters cannot have __fp16 type; did you forget * ?}}
#endif
void fp16_arg(__fp16 p);
