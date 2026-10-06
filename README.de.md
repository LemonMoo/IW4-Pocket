<p align="center">
  <a href="https://github.com/MarkusSela/IW4-Pocket/blob/main/media/demo.mp4"><img src="media/demo.gif" width="720" alt="IW4 Pocket demo"></a>
  <br><sub>▶️ <a href="https://github.com/MarkusSela/IW4-Pocket/blob/main/media/demo.mp4">Komplette Demo ansehen</a> · 18 s, iPhone 13 Pro Max</sub>
</p>

<p align="center">
  <img src="media/icon.png" width="110" alt="IW4 Pocket icon">
</p>

<h1 align="center">IW4 Pocket</h1>

<p align="center">Modern Warfare 2 (2009) auf dem iPhone, mit der Open-Source-Engine IW4L. Inoffiziell und experimentell.</p>

<p align="center"><a href="https://ko-fi.com/marukoshi"><img src="https://img.shields.io/badge/Ko--fi-Support-FF5E5B?logo=ko-fi&logoColor=white" alt="Ko-fi"></a></p>

<p align="center">🌍 <a href="README.md">English</a> · <a href="README.it.md">Italiano</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <b>Deutsch</b></p>

## 📊 Status

- ✅ Läuft nativ auf dem iPhone (Rust + Bevy + Metal), als IPA für SideStore verpackt
- ✅ Erreicht das Hauptmenü: Touch funktioniert als Klick, ein PS4-Controller funktioniert
- ⚠️ Das Laden einer Match-Karte (getestet: `mp_rust`) stürzt weiterhin ab: iOS beendet die App wegen zu viel Speicher. Noch nicht gelöst

## 📱 Kompatibilität

Sollte auf jedem iPhone oder iPad mit Metal laufen, wurde aber **nur auf einem iPhone 13 Pro Max (6 GB) getestet**. Geräte mit weniger RAM scheitern früher.

## 🧰 Das brauchst du

- Ein iPhone oder iPad (iOS 15+) und **SideStore** oder ein anderes Sideloading-Tool
- **Deine eigene Kopie der Modern-Warfare-2-(2009)-PC-Dateien**, der Ordner mit `zone/`. Hier ist nichts vom Spiel enthalten
- Einen Controller (PS4 verifiziert; Xbox-ähnliche hängen vom Modell ab)

## 🚀 Installation

1. Lade **IW4 Pocket.ipa** aus dem [neuesten Release](../../releases/latest) (oder dem Ordner [`ipa`](ipa)) herunter
2. Installiere sie mit SideStore **über** eine ältere Version: Deinstallieren löscht deine Spieldateien
3. Öffne die App einmal und kopiere deinen MW2-Ordner nach **Dateien > Auf meinem iPhone > IW4 Pocket > Games**
4. Starten. Das erste Laden ist langsam und der Bildschirm kann eine Weile rosa bleiben

## 📂 Protokolle und Einstellungen

- `iw4l-boot.log` (gleicher Ordner) protokolliert Start, Speicher und Abstürze. Bitte bei Fehlermeldungen anhängen
- Um das Texturlimit zu erzwingen, lege eine Textdatei `iw4l-texture-cap.txt` mit nur einer Zahl an, z. B. `256`, und starte die App komplett neu

## 🛠️ Selbst bauen

Starte den Workflow **ios-release** im Actions-Tab (macOS-Runner) und gib einen Tag an, um ein Release zu veröffentlichen.

## ⚖️ Credits und Rechtliches

Port von [IW4L](https://github.com/vladtrc/iw4L) von vladtrc und Mitwirkenden (Apache-2.0, siehe `LICENSE`, `NOTICE`; Original-README in `README.upstream.md`).

> Inoffizielles Fanprojekt, nicht verbunden mit Activision, Infinity Ward, Apple oder den IW4L-Autoren. Call of Duty und Modern Warfare sind Marken ihrer Inhaber. Du benötigst eine legitime Kopie des Spiels.

---

<p align="center">☕ Gefällt es dir? Unterstütze das Projekt auf <a href="https://ko-fi.com/marukoshi">Ko-fi</a></p>
