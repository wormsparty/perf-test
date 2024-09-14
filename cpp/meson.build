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

cpp_sources = [
  'src/main.cpp',
  'src/api.cpp',
  'src/config.cpp',
  'src/filter.cpp',
  'src/utils.cpp'
]

cpp_deps = [
  dependency('cpp-httplib'),
  dependency('RapidJSON'),
  dependency('libpqxx'),
  dependency('libconfig++'),
]

executable('cpp-test', cpp_sources,
  dependencies: cpp_deps,
  install: true,
  win_subsystem: 'windows'
)
