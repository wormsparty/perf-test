project('cpp-test', ['cpp'],
          version: '0.1.0',
    meson_version: '>= 0.59.0',
  default_options: [ 
    'warning_level=2', 
    'werror=false', 
    'cpp_std=c++20',
    'default_library=shared',
    'buildtype=release',
    'optimization=3',
  ],
)

cmake = import('cmake')
opt_var = cmake.subproject_options()
opt_var.add_cmake_defines({'SKIP_BUILD_TEST': true})
sub_proj = cmake.subproject('libpqxx', options: opt_var)

cpp_sources = [
  'src/main.cpp',
  'src/api.cpp',
  'src/config.cpp',
  'src/filter.cpp',
  'src/utils.cpp'
]

cpp_deps = [
  dependency('libconfig++'),
  sub_proj.dependency('pqxx'),
]

executable('cpp-test', cpp_sources,
  dependencies: cpp_deps,
  install: true,
  win_subsystem: 'windows'
)
