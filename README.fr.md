<p align="center">
  <a href="https://github.com/MarkusSela/IW4-Pocket/blob/main/media/demo.mp4"><img src="media/demo.gif" width="720" alt="IW4 Pocket demo"></a>
  <br><sub>▶️ <a href="https://github.com/MarkusSela/IW4-Pocket/blob/main/media/demo.mp4">Voir la démo complète</a> · 18 s, iPhone 13 Pro Max</sub>
</p>

<p align="center">
  <img src="media/icon.png" width="110" alt="IW4 Pocket icon">
</p>

<h1 align="center">IW4 Pocket</h1>

<p align="center">Modern Warfare 2 (2009) sur iPhone, avec le moteur open source IW4L. Non officiel et expérimental.</p>

<p align="center"><a href="https://ko-fi.com/marukoshi"><img src="https://img.shields.io/badge/Ko--fi-Support-FF5E5B?logo=ko-fi&logoColor=white" alt="Ko-fi"></a></p>

<p align="center">🌍 <a href="README.md">English</a> · <a href="README.it.md">Italiano</a> · <a href="README.es.md">Español</a> · <b>Français</b> · <a href="README.de.md">Deutsch</a></p>

## 📊 État

- ✅ Fonctionne nativement sur iPhone (Rust + Bevy + Metal), empaqueté en IPA pour SideStore
- ✅ Atteint le menu principal : le toucher fonctionne comme un clic et la manette PS4 fonctionne
- ⚠️ Le chargement d'une carte de partie (testé : `mp_rust`) plante encore : iOS ferme l'app car elle utilise trop de mémoire. Pas encore résolu

## 📱 Compatibilité

Devrait fonctionner sur tout iPhone ou iPad avec Metal, mais **testé uniquement sur un iPhone 13 Pro Max (6 Go)**. Les appareils avec moins de RAM échoueront plus tôt.

## 🧰 Ce qu'il faut

- Un iPhone ou iPad (iOS 15+) et **SideStore** ou un autre outil de sideloading
- **Votre propre copie des fichiers de Modern Warfare 2 (2009) pour PC**, le dossier contenant `zone/`. Rien du jeu n'est inclus ici
- Une manette (PS4 vérifiée ; les manettes de type Xbox dépendent du modèle)

## 🚀 Installation

1. Téléchargez **IW4 Pocket.ipa** depuis la [dernière release](../../releases/latest) (ou le dossier [`ipa`](ipa))
2. Installez-le avec SideStore **par-dessus** une ancienne version : désinstaller supprime vos fichiers de jeu
3. Ouvrez l'app une fois, puis copiez votre dossier MW2 dans **Fichiers > Sur mon iPhone > IW4 Pocket > Games**
4. Lancez. Le premier chargement est lent et l'écran peut rester rose un moment

## 📂 Journaux et réglages

- `iw4l-boot.log` (même dossier) consigne le démarrage, la mémoire et les plantages. Joignez-le aux rapports de bug
- Pour forcer la limite des textures, ajoutez un fichier texte `iw4l-texture-cap.txt` contenant uniquement un nombre, par exemple `256`, puis redémarrez complètement l'app

## 🛠️ Le compiler soi-même

Lancez le workflow **ios-release** depuis l'onglet Actions (runner macOS) et indiquez un tag pour publier une release.

## ⚖️ Crédits et mentions légales

Port d'[IW4L](https://github.com/vladtrc/iw4L) par vladtrc et contributeurs (Apache-2.0, voir `LICENSE`, `NOTICE` ; README d'origine dans `README.upstream.md`).

> Projet de fan non officiel, sans lien avec Activision, Infinity Ward, Apple ni les auteurs d'IW4L. Call of Duty et Modern Warfare sont des marques de leurs propriétaires. Vous devez posséder une copie légitime du jeu.

---

<p align="center">☕ Ça vous plaît ? Soutenez le projet sur <a href="https://ko-fi.com/marukoshi">Ko-fi</a></p>
