# kernel-mahel

Noyau x86-64 minimal écrit en Rust (`no_std`), lancé dans QEMU.
Il démarre sans système d'exploitation, affiche du texte à l'écran (VGA),
écrit sur le port série (COM1) et gère les exceptions breakpoint et double fault
(avec une stack de secours via le TSS/IST).

## Prérequis

- Rust nightly (version fixée par le fichier `rust-toolchain` du dépôt)
- Composants : `rustup component add rust-src llvm-tools-preview`
- bootimage : `cargo install bootimage`
- QEMU : `sudo pacman -S qemu-system-x86` (Arch) ou `sudo apt install qemu-system-x86` (Debian/Ubuntu)

## Lancer

```bash
cargo run
```

QEMU s'ouvre : le noyau affiche `Test!`, déclenche un breakpoint (`int3`),
affiche la stack frame puis reprend son exécution (`Ca fonctionne`).

Pour voir la sortie du port série dans le terminal :

```bash
cargo bootimage
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-kernel-mahel/debug/bootimage-kernel-mahel.bin \
  -serial stdio
```

## Tester le double fault

Appeler une fonction récursive infinie dans `_start` (stack overflow), puis `cargo run`.
Le message `EXCEPTION: DOUBLE FAULT` s'affiche au lieu d'un redémarrage en boucle.

## Organisation

| Fichier | Rôle |
|---|---|
| `src/main.rs` | Entry point `_start`, panic handler, initialisation |
| `src/vga.rs` | Affichage écran (mode texte VGA, `print!` / `println!`) |
| `src/uart.rs` | Port série COM1 (`serial_print!` / `serial_println!`) |
| `src/interrupts.rs` | IDT et handlers (breakpoint, double fault) |
| `src/gdt.rs` | GDT, TSS et stack de secours (IST) |

Base du noyau : série « Writing an OS in Rust » (Philipp Oppermann).
Documentation détaillée : `Documentation_Sacha_Moiroud.pdf`. Journal IA : `IA.md`.
