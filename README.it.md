<p align="center">
  <video src="https://github.com/MarkusSela/IW4-Pocket/raw/main/media/demo.mp4" controls muted loop playsinline width="100%"></video>
</p>

<p align="center"><sub>Demo (18 s, iPhone 13 Pro Max) · <a href="https://github.com/MarkusSela/IW4-Pocket/raw/main/media/demo.mp4">Se il video non parte, aprilo direttamente</a></sub></p>

<p align="center">
  <img src="media/icon.png" width="128" alt="IW4 Pocket icon">
</p>

<h1 align="center">IW4 Pocket</h1>

<p align="center">Modern Warfare 2 (2009) su iPhone, con il runtime open source IW4L. Non ufficiale e sperimentale.</p>

<p align="center"><a href="https://ko-fi.com/marukoshi"><img src="https://img.shields.io/badge/Ko--fi-Support-FF5E5B?logo=ko-fi&logoColor=white" alt="Ko-fi"></a></p>

<p align="center">[English](README.md) · **Italiano** · [Español](README.es.md) · [Français](README.fr.md) · [Deutsch](README.de.md)</p>

## Stato

- **Compila e gira su iPhone** (Rust + Bevy + wgpu su Metal), impacchettato come IPA per SideStore.
- **Arriva al menu principale.** Nei menu il tocco funziona come clic del mouse. Un controller PS4 (DualShock 4) funziona tramite il framework Apple GameController.
- **Problema noto:** il caricamento di una mappa di partita (provata: `mp_rust`) va ancora in crash. iOS chiude l'app perché usa troppa memoria. È un lavoro in corso e non è ancora risolto.
- La gestione delle texture è adattata alle GPU Apple (niente supporto BC/DXT: le texture vengono decodificate in RGBA8 e limitate in dimensione).

## Compatibilità

Tecnicamente dovrebbe funzionare su qualsiasi iPhone o iPad con Metal e abbastanza memoria libera, ma è stato **testato solo su iPhone 13 Pro Max (6 GB, iOS 27)**. Gli altri dispositivi non sono verificati. Il limite è la memoria: su quel telefono iOS concede all'app circa 3 GB, e i modelli con meno RAM falliranno prima.

## Cosa serve

- Un iPhone o iPad con Metal (iOS 15 o successivo).
- SideStore (o un altro strumento di sideload) per installare l'IPA.
- **La tua copia dei dati di Modern Warfare 2 (2009) per PC**, la cartella che contiene `zone/`. Qui non ci sono file del gioco e non vengono forniti.
- Si consiglia un controller. Il PS4 è verificato; i controller in stile Xbox dipendono dal modello.

## Installazione

1. Scarica **IW4 Pocket.ipa** dall'[ultima release](../../releases/latest).
2. Installalo con SideStore. Installa sopra una versione precedente per non perdere i file di gioco: disinstallare li cancella.
3. Apri l'app una volta, poi copia la cartella di MW2 PC in **File > Su iPhone > IW4 Pocket > Games** (va bene qualsiasi nome di sottocartella, purché contenga `zone/`).
4. Avvia l'app. Il primo caricamento è lento e lo schermo può restare rosa per un po'.

## File e log

L'app scrive `Documents/iw4l-boot.log` (visibile nell'app File). Registra i passi di avvio, il limite texture scelto, l'uso della memoria (`footprint` e quanta ne concede ancora iOS) e ogni panic o segnale fatale. Allegalo quando segnali un problema.

### Limite dimensione texture

Il limite viene scelto in automatico in base alla memoria che iOS concede all'app. Per forzarlo, crea nella stessa cartella un file di testo semplice `iw4l-texture-cap.txt` che contiene solo un numero, per esempio `256`, poi chiudi del tutto e riapri l'app.

## Cosa è cambiato rispetto a IW4L originale

- Su iOS niente texture BC: vengono decodificate in RGBA8 con un limite di dimensione.
- Il rendering resta sul thread principale (la superficie Metal va creata lì).
- Percorsi nella sandbox `Documents`, con `Info.plist` iOS e icona dell'app.
- Ponte GameController per i controller, e tocco mappato sul pulsante sinistro del mouse.
- Log di avvio con tracciamento della memoria e cattura dei segnali di crash.

## Compilarlo da solo

Avvia il workflow **ios-release** dalla scheda Actions (runner macOS). Indica un tag per pubblicare una release. Nulla parte in automatico a ogni push.

## Supporto

Se il progetto ti piace, puoi sostenerlo su [Ko-fi](https://ko-fi.com/marukoshi).

## Crediti e licenza

Questo progetto è un port di [IW4L](https://github.com/vladtrc/iw4L) di vladtrc e collaboratori, con licenza Apache-2.0 (vedi `LICENSE` e `NOTICE`). Il README originale è in `README.upstream.md`.

> IW4 Pocket è un progetto non ufficiale. Non è affiliato né approvato da Activision, Infinity Ward, Apple o dagli autori di IW4L. Call of Duty e Modern Warfare sono marchi dei rispettivi proprietari. Serve una copia legittima del gioco.
