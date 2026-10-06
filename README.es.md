<p align="center">
  <video src="https://github.com/MarkusSela/IW4-Pocket/raw/main/media/demo.mp4" controls muted loop playsinline width="100%"></video>
</p>

<p align="center"><sub>Demo (18 s, iPhone 13 Pro Max) · <a href="https://github.com/MarkusSela/IW4-Pocket/raw/main/media/demo.mp4">Si el vídeo no se reproduce, ábrelo directamente</a></sub></p>

<p align="center">
  <img src="media/icon.png" width="128" alt="IW4 Pocket icon">
</p>

<h1 align="center">IW4 Pocket</h1>

<p align="center">Modern Warfare 2 (2009) en iPhone, con el runtime de código abierto IW4L. No oficial y experimental.</p>

<p align="center"><a href="https://ko-fi.com/marukoshi"><img src="https://img.shields.io/badge/Ko--fi-Support-FF5E5B?logo=ko-fi&logoColor=white" alt="Ko-fi"></a></p>

<p align="center">[English](README.md) · [Italiano](README.it.md) · **Español** · [Français](README.fr.md) · [Deutsch](README.de.md)</p>

## Estado

- **Compila y se ejecuta en iPhone** (Rust + Bevy + wgpu sobre Metal), empaquetado como IPA para SideStore.
- **Llega al menú principal.** En los menús el toque funciona como clic del ratón. Un mando de PS4 (DualShock 4) funciona mediante el framework GameController de Apple.
- **Problema conocido:** cargar un mapa de partida (probado: `mp_rust`) sigue fallando. iOS cierra la app por usar demasiada memoria. Es un trabajo en curso y aún no está resuelto.
- El manejo de texturas está adaptado a las GPU de Apple (sin soporte BC/DXT: las texturas se decodifican a RGBA8 y se limita su tamaño).

## Compatibilidad

Técnicamente debería funcionar en cualquier iPhone o iPad con Metal y suficiente memoria libre, pero **solo se ha probado en un iPhone 13 Pro Max (6 GB, iOS 27)**. Los demás dispositivos no están verificados. El límite es la memoria: en ese teléfono iOS concede a la app unos 3 GB, y los modelos con menos RAM fallarán antes.

## Qué necesitas

- Un iPhone o iPad con Metal (iOS 15 o posterior).
- SideStore (u otra herramienta de sideloading) para instalar el IPA.
- **Tu propia copia de los datos de Modern Warfare 2 (2009) para PC**, la carpeta que contiene `zone/`. Aquí no se incluyen ni se facilitan archivos del juego.
- Se recomienda un mando. El de PS4 está verificado; los de estilo Xbox dependen del modelo.

## Instalación

1. Descarga **IW4 Pocket.ipa** de la [última release](../../releases/latest).
2. Instálalo con SideStore. Instala sobre una versión anterior para conservar los archivos del juego; desinstalar los borra.
3. Abre la app una vez y copia la carpeta de MW2 para PC en **Archivos > En mi iPhone > IW4 Pocket > Games** (vale cualquier nombre de subcarpeta si contiene `zone/`).
4. Inicia la app. La primera carga es lenta y la pantalla puede quedarse rosa un rato.

## Archivos y registros

La app escribe `Documents/iw4l-boot.log` (visible en la app Archivos). Registra los pasos de inicio, el límite de texturas elegido, el uso de memoria (`footprint` y cuánta permite aún iOS) y cualquier panic o señal fatal. Adjúntalo al informar de un problema.

### Límite de tamaño de texturas

El límite se elige automáticamente según la memoria que iOS concede a la app. Para forzarlo, crea en la misma carpeta un archivo de texto plano `iw4l-texture-cap.txt` con solo un número, por ejemplo `256`, y cierra y reabre la app por completo.

## Qué cambió respecto a IW4L original

- En iOS no hay texturas BC: se decodifican a RGBA8 con un límite de tamaño.
- El renderizado se mantiene en el hilo principal (la superficie Metal debe crearse ahí).
- Rutas en la sandbox `Documents`, con `Info.plist` de iOS e icono de la app.
- Puente GameController para mandos y entrada táctil asignada al botón izquierdo del ratón.
- Registro de arranque con seguimiento de memoria y captura de señales de fallo.

## Compilarlo tú mismo

Ejecuta el workflow **ios-release** desde la pestaña Actions (runner macOS). Indica una etiqueta para publicar una release. Nada se ejecuta automáticamente en cada push.

## Apoyo

Si te gusta el proyecto, puedes apoyarlo en [Ko-fi](https://ko-fi.com/marukoshi).

## Créditos y licencia

Este proyecto es un port de [IW4L](https://github.com/vladtrc/iw4L) de vladtrc y colaboradores, con licencia Apache-2.0 (ver `LICENSE` y `NOTICE`). El README original se conserva en `README.upstream.md`.

> IW4 Pocket es un proyecto de aficionados no oficial. No está afiliado ni respaldado por Activision, Infinity Ward, Apple ni los autores de IW4L. Call of Duty y Modern Warfare son marcas de sus propietarios. Necesitas una copia legítima del juego.
