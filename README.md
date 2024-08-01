# Conway's Game of Life implementation

## Quick Start

```console
$ rustc ./conway.rs
$ ./conway
```

## Also, do not forget to check the code itself

```console
$ cat ./conway.rs

                  const R:&str="  ";
                  const Y:&str="██";
                  use std::thread;use
                  std::time::Duration
                  ;const S:usize=10;
                  const L:u64=150;#[
                  derive(Copy,Clone,
                  PartialEq)]enum C{
                  D,A,}struct B{e:[[
                                    C;S];S],}fn main()
                                    {let mut e=[[C::D;
                                    S];S];e[0][1]=C::A;
                                    e[1][2]=C::A;e[2][
                                    0]=C::A;e[2][1]=C::
                                    A;e[2][2]=C::A;let
                                    mut o=B{e};loop{for
                                    r in 0..S{for c in
                                    0..S{match o.e[r][
c]{C::D=>{print!("{R}");}C::A=>{print!("{Y}");}}}print!
("\n");}let mut x=[[C::D;S];S];for r in 0..S{for c in 0
..S{let mut n:usize=0;for dr in-1..=1{for dc in-1..=1{
if dr !=0||dc !=0{let nc=(((c as i32+dc)+S as i32)%S as
i32)as usize;let nr=(((r as i32+dr)+S as i32)%S as i32)
as usize;if o.e[nr][nc]==C::A{n+=1}}}}match o.e[r][c]{C
::D=>{if n==3{x[r][c]=C::A;}}C::A=>{if n==2||n==3{x[r][
c]=C::A;}}}}}o.e=x;thread::sleep(Duration::from_millis(
L));print!("\x1b[{S}A\x1b[{S}D");}} // Created by jus1d
```

wotafak? ( .-.)

# Generating some similar staff

For now, you need to specify pattern and size in `format.rs` in a bit crappy way. I hope I'll change it later

```console
$ rustc format.rs
$ ./format > output.rs
```

Than you can view formatted code, and maybe fix it manually (cause for now formatter isn't supacool)

```console
$ cat ./output.rs
rustc ./output.rs && ./output
```
