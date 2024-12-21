# Build

```
nasm -f win32 first.asm -o build/first.obj
```

```
link build/first.obj /SUBSYSTEM:CONSOLE /ENTRY:main /OUT:build/first.exe /MACHINE:X86 "C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22621.0\um\x86\kernel32.lib"
```

# Run 
```
./build/first test.cm
```

