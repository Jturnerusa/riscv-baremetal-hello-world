sources = [
  'lib.rs',
  'uart.rs',
  'kprint.rs'
]

if get_option('qemu').enabled()
  flags += ['--cfg', 'qemu']
endif

libkernel = static_library('kernel', sources, rust_crate_type: 'staticlib', rust_args: flags)
