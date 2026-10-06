<p align="center">
  <a href="https://github.com/MarkusSela/IW4-Pocket/blob/main/media/demo.mp4"><img src="media/demo.gif" width="720" alt="IW4 Pocket demo"></a>
  <br><sub>▶️ <a href="https://github.com/MarkusSela/IW4-Pocket/blob/main/media/demo.mp4">Ver la demo completa</a> · 18 s, iPhone 13 Pro Max</sub>
</p>

<p align="center">
  <img src="media/icon.png" width="110" alt="IW4 Pocket icon">
</p>

<h1 align="center">IW4 Pocket</h1>

<p align="center">Modern Warfare 2 (2009) en iPhone, con el motor de código abierto IW4L. No oficial y experimental.</p>

<p align="center"><a href="https://ko-fi.com/marukoshi"><img src="https://img.shields.io/badge/Ko--fi-Support-FF5E5B?logo=ko-fi&logoColor=white" alt="Ko-fi"></a></p>

<p align="center">🌍 [English](README.md) · [Italiano](README.it.md) · **Español** · [Français](README.fr.md) · [Deutsch](README.de.md)</p>

## 📊 Estado

- ✅ Se ejecuta de forma nativa en iPhone (Rust + Bevy + Metal), empaquetado como IPA para SideStore
- ✅ Llega al menú principal: el toque funciona como clic y el mando de PS4 funciona
- ⚠️ Cargar un mapa de partida (probado: `mp_rust`) sigue fallando: iOS cierra la app por usar demasiada memoria. Aún sin resolver

## 📱 Compatibilidad

Debería funcionar en cualquier iPhone o iPad con Metal, pero **solo se ha probado en un iPhone 13 Pro Max (6 GB)**. Los dispositivos con menos RAM fallarán antes.

## 🧰 Qué necesitas

- Un iPhone o iPad (iOS 15+) y **SideStore** u otra herramienta de sideloading
- **Tu propia copia de los archivos de Modern Warfare 2 (2009) para PC**, la carpeta que contiene `zone/`. Aquí no se incluye nada del juego
- Un mando (PS4 verificado; los de estilo Xbox dependen del modelo)

## 🚀 Instalación

1. Descarga **IW4 Pocket.ipa** de la [última release](../../releases/latest) (o de la carpeta [`ipa`](ipa))
2. Instálalo con SideStore **sobre** una versión anterior: desinstalar borra tus archivos del juego
3. Abre la app una vez y copia tu carpeta de MW2 en **Archivos > En mi iPhone > IW4 Pocket > Games**
4. Inicia. La primera carga es lenta y la pantalla puede quedarse rosa un rato

## 📂 Registros y ajustes

- `iw4l-boot.log` (misma carpeta) registra el inicio, la memoria y los fallos. Adjúntalo al informar de errores
- Para forzar el límite de texturas, añade un archivo de texto `iw4l-texture-cap.txt` con solo un número, por ejemplo `256`, y reinicia la app por completo

## 🛠️ Compilarlo tú mismo

Ejecuta el workflow **ios-release** desde la pestaña Actions (runner macOS) e indica una etiqueta para publicar una release.

## ⚖️ Créditos y aviso legal

Port de [IW4L](https://github.com/vladtrc/iw4L) de vladtrc y colaboradores (Apache-2.0, ver `LICENSE`, `NOTICE`; README original en `README.upstream.md`).

> Proyecto no oficial, sin afiliación con Activision, Infinity Ward, Apple ni los autores de IW4L. Call of Duty y Modern Warfare son marcas de sus propietarios. Necesitas una copia legítima del juego.

---

<p align="center">☕ ¿Te gusta? Apoya el proyecto en [Ko-fi](https://ko-fi.com/marukoshi)</p>
