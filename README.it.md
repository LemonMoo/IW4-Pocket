<p align="center">
  <a href="https://github.com/MarkusSela/IW4-Pocket/blob/main/media/demo.mp4"><img src="media/demo.gif" width="720" alt="IW4 Pocket demo"></a>
  <br><sub>▶️ <a href="https://github.com/MarkusSela/IW4-Pocket/blob/main/media/demo.mp4">Guarda la demo completa</a> · 18 s, iPhone 13 Pro Max</sub>
</p>

<p align="center">
  <img src="media/icon.png" width="110" alt="IW4 Pocket icon">
</p>

<h1 align="center">IW4 Pocket</h1>

<p align="center">Modern Warfare 2 (2009) su iPhone, con il motore open source IW4L. Non ufficiale e sperimentale.</p>

<p align="center"><a href="https://ko-fi.com/marukoshi"><img src="https://img.shields.io/badge/Ko--fi-Support-FF5E5B?logo=ko-fi&logoColor=white" alt="Ko-fi"></a></p>

<p align="center">🌍 [English](README.md) · **Italiano** · [Español](README.es.md) · [Français](README.fr.md) · [Deutsch](README.de.md)</p>

## 📊 Stato

- ✅ Gira nativamente su iPhone (Rust + Bevy + Metal), impacchettato come IPA per SideStore
- ✅ Arriva al menu principale: il tocco funziona come clic e il controller PS4 funziona
- ⚠️ Il caricamento di una mappa di partita (provata: `mp_rust`) va ancora in crash: iOS chiude l'app per troppa memoria. Non ancora risolto

## 📱 Compatibilità

Dovrebbe girare su qualsiasi iPhone o iPad con Metal, ma è stato **testato solo su iPhone 13 Pro Max (6 GB)**. I dispositivi con meno RAM falliranno prima.

## 🧰 Cosa serve

- Un iPhone o iPad (iOS 15+) e **SideStore** o un altro strumento di sideload
- **La tua copia dei file di Modern Warfare 2 (2009) per PC**, la cartella che contiene `zone/`. Qui non c'è nulla del gioco
- Un controller (PS4 verificato; quelli stile Xbox dipendono dal modello)

## 🚀 Installazione

1. Scarica **IW4 Pocket.ipa** dall'[ultima release](../../releases/latest) (o dalla cartella [`ipa`](ipa))
2. Installalo con SideStore **sopra** una versione precedente: disinstallare cancella i file di gioco
3. Apri l'app una volta, poi copia la cartella di MW2 in **File > Su iPhone > IW4 Pocket > Games**
4. Avvia. Il primo caricamento è lento e lo schermo può restare rosa per un po'

## 📂 Log e impostazioni

- `iw4l-boot.log` (stessa cartella) registra avvio, memoria e crash. Allegalo alle segnalazioni
- Per forzare il limite delle texture, aggiungi un file di testo `iw4l-texture-cap.txt` con solo un numero, ad esempio `256`, poi riavvia del tutto l'app

## 🛠️ Compilarlo da solo

Avvia il workflow **ios-release** dalla scheda Actions (runner macOS) e indica un tag per pubblicare una release.

## ⚖️ Crediti e note legali

Port di [IW4L](https://github.com/vladtrc/iw4L) di vladtrc e collaboratori (Apache-2.0, vedi `LICENSE`, `NOTICE`; README originale in `README.upstream.md`).

> Progetto non ufficiale, non affiliato ad Activision, Infinity Ward, Apple né agli autori di IW4L. Call of Duty e Modern Warfare sono marchi dei rispettivi proprietari. Serve una copia legittima del gioco.

---

<p align="center">☕ Ti piace? Sostieni il progetto su [Ko-fi](https://ko-fi.com/marukoshi)</p>
