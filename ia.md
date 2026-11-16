16/11 · Claude · chapitre 5 (CPU Exceptions) en français

Demande : une explication en français des parties du chapitre 5 utiles à mon projet
Gardé : l’IDT dans un lazy_static, le handler breakpoint en extern “x86-interrupt”, le test avec int3
Corrigé : #![feature(abi_x86_interrupt)] placé dans interrupts.rs, il doit être à la racine (main.rs)
Rejeté : le passage par lib.rs et init() comme dans l’article, inutile sans framework de test

16/11 · Claude · chapitre 6 (Double Faults) en français

Demande : une explication en français de la GDT, du TSS et de l’IST, adaptée à mon projet
Gardé : TSS avec une stack de secours de 20 Kio à l’index IST 0, GDT chargée avant l’IDT
Corrigé : le déclencheur proposé avant (let _ = * ptr;) ne lisait pas la mémoire, remplacé par une écriture
Rejeté : le test d’intégration stack_overflow.rs de l’article, je n’ai pas de framework de test

16/11 · Claude · test du double fault

Demande : un moyen de vérifier que le double fault s’exécute bien sur la stack de secours
Gardé : une fonction récursive infinie avec black_box, utilisée uniquement pour le test
Corrigé : un panic! de test resté après int3, remplacé par println! pour que l’exécution reprenne
Rejeté : le screen de la contre-épreuve sans set_stack_index, illisible en image, montrée en vidéo

