<p align="center">
  <video src="https://github.com/MarkusSela/IW4-Pocket/raw/main/media/demo.mp4" controls muted loop playsinline width="100%"></video>
</p>

<p align="center"><sub>Démo (18 s, iPhone 13 Pro Max) · <a href="https://github.com/MarkusSela/IW4-Pocket/raw/main/media/demo.mp4">Si la vidéo ne se lance pas, ouvrez-la directement</a></sub></p>

<p align="center">
  <img src="media/icon.png" width="128" alt="IW4 Pocket icon">
</p>

<h1 align="center">IW4 Pocket</h1>

<p align="center">Modern Warfare 2 (2009) sur iPhone, avec le runtime open source IW4L. Non officiel et expérimental.</p>

<p align="center"><a href="https://ko-fi.com/marukoshi"><img src="https://img.shields.io/badge/Ko--fi-Support-FF5E5B?logo=ko-fi&logoColor=white" alt="Ko-fi"></a></p>

<p align="center">[English](README.md) · [Italiano](README.it.md) · [Español](README.es.md) · **Français** · [Deutsch](README.de.md)</p>

## État

- **Compile et fonctionne sur iPhone** (Rust + Bevy + wgpu sur Metal), empaqueté en IPA pour SideStore.
- **Atteint le menu principal.** Dans les menus, le toucher fonctionne comme un clic de souris. Une manette PS4 (DualShock 4) fonctionne via le framework GameController d'Apple.
- **Problème connu :** le chargement d'une carte de partie (testé : `mp_rust`) plante encore. iOS ferme l'app car elle utilise trop de mémoire. C'est un travail en cours, pas encore résolu.
- La gestion des textures est adaptée aux GPU Apple (pas de support BC/DXT : les textures sont décodées en RGBA8 et limitées en taille).

## Compatibilité

Techniquement, cela devrait fonctionner sur tout iPhone ou iPad avec Metal et assez de mémoire libre, mais il a **uniquement été testé sur un iPhone 13 Pro Max (6 Go, iOS 27)**. Les autres appareils ne sont pas vérifiés. La mémoire est le facteur limitant : sur ce téléphone, iOS accorde environ 3 Go à l'app, et les modèles avec moins de RAM échoueront plus tôt.

## Ce qu'il faut

- Un iPhone ou iPad avec Metal (iOS 15 ou plus récent).
- SideStore (ou un autre outil de sideloading) pour installer l'IPA.
- **Votre propre copie des données de Modern Warfare 2 (2009) pour PC**, le dossier qui contient `zone/`. Aucun fichier du jeu n'est inclus ni fourni ici.
- Une manette est fortement recommandée. La PS4 est vérifiée ; les manettes de type Xbox dépendent du modèle.

## Installation

1. Téléchargez **IW4 Pocket.ipa** depuis la [dernière release](../../releases/latest).
2. Installez-le avec SideStore. Installez par-dessus une ancienne version pour conserver vos fichiers de jeu ; désinstaller les supprime.
3. Ouvrez l'app une fois, puis copiez le dossier MW2 PC dans **Fichiers > Sur mon iPhone > IW4 Pocket > Games** (n'importe quel nom de sous-dossier convient s'il contient `zone/`).
4. Lancez l'app. Le premier chargement est lent et l'écran peut rester rose un moment.

## Fichiers et journaux

L'app écrit `Documents/iw4l-boot.log` (visible dans l'app Fichiers). Il consigne les étapes de démarrage, la limite de texture choisie, l'usage mémoire (`footprint` et ce qu'iOS autorise encore) et tout panic ou signal fatal. Joignez-le pour signaler un problème.

### Limite de taille des textures

La limite est choisie automatiquement selon la mémoire qu'iOS accorde à l'app. Pour la forcer, créez dans le même dossier un fichier texte brut `iw4l-texture-cap.txt` contenant uniquement un nombre, par exemple `256`, puis fermez complètement et rouvrez l'app.

## Ce qui a changé par rapport à IW4L d'origine

- Sur iOS, pas de textures BC : elles sont décodées en RGBA8 avec une limite de taille.
- Le rendu reste sur le thread principal (la surface Metal doit y être créée).
- Chemins dans la sandbox `Documents`, avec `Info.plist` iOS et icône de l'app.
- Pont GameController pour les manettes et toucher associé au bouton gauche de la souris.
- Journal de démarrage avec suivi de la mémoire et capture des signaux de plantage.

## Le compiler soi-même

Lancez le workflow **ios-release** depuis l'onglet Actions (runner macOS). Indiquez un tag pour publier une release. Rien ne se lance automatiquement à chaque push.

## Soutien

Si le projet vous plaît, vous pouvez le soutenir sur [Ko-fi](https://ko-fi.com/marukoshi).

## Crédits et licence

Ce projet est un port de [IW4L](https://github.com/vladtrc/iw4L) par vladtrc et contributeurs, sous licence Apache-2.0 (voir `LICENSE` et `NOTICE`). Le README d'origine est conservé dans `README.upstream.md`.

> IW4 Pocket est un projet de fan non officiel. Il n'est ni affilié ni approuvé par Activision, Infinity Ward, Apple ou les auteurs d'IW4L. Call of Duty et Modern Warfare sont des marques de leurs propriétaires. Vous devez posséder une copie légitime du jeu.
