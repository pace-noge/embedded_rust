## Debugging notes
- start openocd
`$ opencd`
- start arm-none-eabi gdb
`$ arm-none-eabi-gdb -x openocd.gdb`

### GDB command
```shell
$ layout src # open the layout
$ s # next step
$ n # next line
```