<p align="center">
  <img alt="InsurCraft" src="assets/app-icon/insurcraft-small.svg" width="112">
</p>

<h1 align="center">InsurCraft</h1>

<p align="center">
  <b>Banco de trabajo PDF open source para Insur S.A.</b><br>
  Lee, organiza, une, divide, comenta, firma y protege PDFs. Todo local, sin cuentas ni nube.
</p>

<p align="center">
  <img alt="Licencia: MIT OR Apache-2.0" src="https://img.shields.io/badge/licencia-MIT%20OR%20Apache--2.0-14658F">
  <img alt="Escrito en Rust" src="https://img.shields.io/badge/escrito%20en-Rust-0A3550">
  <img alt="Windows, macOS, Linux, web" src="https://img.shields.io/badge/corre%20en-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux%20%C2%B7%20web-14658F">
</p>

> **InsurCraft es un fork de [PdfCraft](https://github.com/storytold/pdfcraft), del equipo de ArtCraft.**
> Toma su código (licencia MIT OR Apache-2.0), le cambia el nombre, el ícono y los enlaces, y lo
> adapta para uso interno de Insur S.A. InsurCraft no es un producto de ArtCraft ni está respaldado
> por ellos. Los detalles están en [NOTICE](NOTICE).

## Qué hace

- **Leer**: visor rápido con búsqueda, marcadores, miniaturas, modo lectura y tema oscuro.
- **Organizar páginas**: reordenar, rotar, eliminar, insertar y extraer páginas arrastrándolas como tarjetas.
- **Unir y dividir**: combinar varios PDFs en uno (cada archivo queda como marcador), extraer páginas o
  partir un documento cada *n* páginas o antes de las páginas que elijas, sin perder enlaces, formularios
  ni adjuntos.
- **Comentar y llenar**: notas, resaltados, formas, formularios interactivos y firmas.
- **Proteger**: abrir PDFs con contraseña, cifrar, restringir permisos y censurar (redactar) contenido.
- **Automatizar**: cada función se puede usar desde la línea de comandos (`insurcraft-cli`), un canal
  JSON o un servidor MCP, para scripts y agentes de IA.
- **Privado**: todo corre en tu máquina, sobre tus archivos. Sin telemetría.

> **Estado: alfa.** El código base (PdfCraft) está en desarrollo activo. Revisa los resultados antes de
> usarlo con documentos importantes.

## Compilar y ejecutar

Necesitas [Rust](https://rustup.rs/) (en Windows, además, las "Build Tools" de Visual Studio con C++).

```sh
git clone https://github.com/Pourider/insurcraft
cd insurcraft
cargo run --release -p insurcraft -- documento.pdf     # app de escritorio
cargo run --release -p insurcraft-cli -- tools          # lista de herramientas de la CLI
```

Unir, extraer y dividir desde la línea de comandos:

```sh
insurcraft-cli combine reporte.pdf anexo.pdf --out combinado.pdf
insurcraft-cli extract reporte.pdf --pages 1,3,5 --out seleccion.pdf
insurcraft-cli split   reporte.pdf --every 10 --out-dir partes/
```

El ejecutable queda en `target/release/insurcraft` (`insurcraft.exe` en Windows).

El idioma se cambia en **Menú → Editar → Preferencias…** (Ctrl+coma). Español viene incluido.

Para compilar con las fuentes japonesas opcionales:

```sh
git clone https://github.com/storytold/craft-fonts ../craft-fonts
CRAFT_FONTS_DIR=../craft-fonts cargo run --release -p insurcraft -- documento.pdf
```

Más notas de desarrollo en [docs/development.md](docs/development.md). Las reglas para contribuir
(assets, licencias, calidad) están en [AGENTS.md](AGENTS.md) y [CLAUDE.md](CLAUDE.md).

### Instalador de Windows

`packaging/windows/` arma un `.msi` y un `.zip` portable. Para instalar en varias PCs de la empresa sin
acceso directo en el escritorio:

```powershell
msiexec /i "insurcraft-<ver>-windows-x64.msi" /qn /norestart INSTALLDESKTOPSHORTCUT=0
```

Los builds de InsurCraft **no están firmados** con certificado de código, así que Windows SmartScreen
puede mostrar una advertencia la primera vez.

## Web y versión en el navegador

El workflow `.github/workflows/pages.yml` publica en GitHub Pages la landing (`docs/index.html`) y la app
completa compilada a WebAssembly en `/app/`. Para activarlo: **Settings → Pages → Source: GitHub Actions**.
Después, cada push a `main` la actualiza.

## Estructura del proyecto

| Carpeta | Contenido |
|---|---|
| `apps/insurcraft` | la app de escritorio |
| `apps/insurcraft-cli` | la herramienta de línea de comandos y el servidor MCP |
| `apps/insurcraft-web` | la versión para navegador (WebAssembly) |
| `crates/` | el motor: lectura de PDF, render, organización, cifrado, firmas, OCR, etc. |
| `packaging/` | instaladores para Windows, macOS y Linux |
| `assets/` | ícono de la app, íconos Lucide y fuentes con licencia abierta |

## Traer mejoras de PdfCraft

InsurCraft se puede mantener al día con el proyecto original:

```sh
git remote add upstream https://github.com/storytold/pdfcraft
git fetch upstream
git merge upstream/main     # resuelve conflictos de nombres (PdfCraft → InsurCraft) a mano
```

## Licencia y créditos

InsurCraft se distribuye bajo [MIT](LICENSE-MIT) o [Apache 2.0](LICENSE-APACHE), a tu elección, igual que PdfCraft.

- Basado en **PdfCraft** del equipo de ArtCraft y sus colaboradores: https://github.com/storytold/pdfcraft
- Los avisos de terceros (íconos Lucide, fuentes Inter, JetBrains Mono, Dancing Script, hayro, etc.) están en
  [NOTICE](NOTICE) y [ATTRIBUTION.md](ATTRIBUTION.md).
- "ArtCraft" y sus logos son marcas del ArtCraft Team y no forman parte de este proyecto.
