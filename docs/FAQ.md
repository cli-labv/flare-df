# ❓ Preguntas Frecuentes - FLARE-DF

Respuestas a las preguntas más comunes sobre FLARE-DF.

## 🚀 Instalación y Configuración

### ¿Necesito privilegios de administrador?
**R:** Solo si tu carpeta proyecto requiere permisos. Puedes configurar `SUDO_PASSWORD` en `.env` para automatizar esto. Si no, la app te pedirá la contraseña cuando sea necesario.

### ¿Funciona en Windows?
**R:** Sí, usando **WSL2 (Windows Subsystem for Linux 2)**. Sigue la guía de instalación para Ubuntu dentro de WSL2.

### ¿Puedo instalar sin Rust?
**R:** No. Rust 1.70+ es requerido. Instálalo desde https://rustup.rs/

### ¿Qué versiones de QPDF/Ghostscript necesito?
**R:** 
- QPDF: 11.0+
- Ghostscript: 10.0+

Cualquier versión igual o superior funciona.

---

## 💾 Uso y Compresión

### ¿Cuánto puedo comprimir?
**R:** Depende del nivel:
- **Lossless**: 5-15% (sin pérdida visual)
- **Alta Calidad**: 20-40% (mínima pérdida)
- **Balanceado**: 40-60% (buen balance)
- **Agresivo**: 60-80% (máxima compresión)

### ¿Cuál nivel debería usar?
**R:** Recomendamos **Balanceado (~50%)** para la mayoría de casos:
- Buena reducción de tamaño
- Calidad visual excelente
- Equilibrio óptimo

### ¿Pierdo calidad con la compresión?
**R:** Depende del modo:
- **Lossless**: NO, sin pérdida
- **Alta Calidad**: Mínima, imperceptible
- **Balanceado**: Ligera, generalmente no noticeable
- **Agresivo**: Visible pero aceptable para lectura

### ¿Puedo especificar mi propio porcentaje?
**R:** Sí, usa el modo **⚙️ Personalizado** e ingresa 0-99%.

### ¿Cuánto tarda la compresión?
**R:** Depende de:
- Cantidad de PDFs
- Tamaño de los PDFs
- Poder del procesador
- Modo (paralelo es más rápido)

Tipicamente: 1-5 minutos para 100 MB en modo paralelo.

### ¿Procesamiento paralelo o secuencial?
**R:**
- **⚡ Paralelo**: Para múltiples PDFs (mucho más rápido)
- **🐢 Secuencial**: Para detalles paso a paso

---

## 📁 Archivos y Carpetas

### ¿Dónde van mis PDFs originales?
**R:** En la carpeta `input/` del proyecto o especifica una ruta externa.

### ¿Dónde están los PDFs comprimidos?
**R:** En la carpeta `output/` del proyecto.

### ¿Se modifican mis PDFs originales?
**R:** NO. Los originales nunca se tocan. Se crean copias en `output/`.

### ¿Qué es la carpeta `temp/`?
**R:** Archivos temporales durante la compresión. Se limpian automáticamente.

### ¿Puedo usar carpetas externas?
**R:** Sí, usa el modo **external** e ingresa la ruta absoluta.

### ¿Y si la carpeta no existe?
**R:** La app te lo dirá. Crea la carpeta y reintenta.

---

## 🔐 Seguridad y Permisos

### ¿Es seguro ingresar mi contraseña?
**R:** Sí, se guarda en `.env` que está en `.gitignore` (no se commitea).

### ¿Puedo usar una carpeta externa sin permisos?
**R:** No. Necesitas permisos de lectura/escritura. La app puede intentar elevar permisos con sudo.

### ¿Se registra mi contraseña en algún log?
**R:** No. Se usa para sudo y luego se descarta. No se almacena en logs.

### ¿Qué permisos necesito?
**R:**
- Lectura: Carpeta origen y PDFs
- Escritura: Carpeta output
- Ejecución: Script start.sh

---

## 🎨 Interfaz y Visualización

### ¿Por qué diferentes spinners cada vez?
**R:** Para mantener visualmente interesante. Hay 26 estilos diferentes que se seleccionan aleatoriamente.

### ¿Puedo desactivar los spinners?
**R:** No directamente, pero puedes redireccionar output a archivo si los necesitas ocultos.

### ¿Por qué aparece "Paso 1" y "Paso 2"?
**R:** 
- **Paso 1**: QPDF optimiza la estructura
- **Paso 2**: Ghostscript comprime contenido

Solo aplica en modos no-lossless.

---

## ⚙️ Configuración Avanzada

### ¿Puedo cambiar el número de workers?
**R:** Sí, en `.env`:
```env
MAX_WORKERS=4
```
Por defecto usa: `cpu_count() - 1`

### ¿Puedo cambiar las rutas?
**R:** Sí, en `.env`:
```env
INPUT_DIR=./mi_input
OUTPUT_DIR=./mi_output
TEMP_DIR=./mi_temp
```

### ¿Qué es DEBUG mode?
**R:** Muestra información adicional durante ejecución. Útil para desarrollo.
```env
DEBUG=true
```

---

## 🐛 Problemas y Solución

### Mi PDF no se comprimi significativamente
**R:** 
- ¿Usaste modo agresivo?
- ¿El PDF ya está optimizado?
- ¿Contiene muchas imágenes?

Algunos PDFs ya vienen muy comprimidos.

### Error: "Motores faltantes"
**R:** Instala QPDF y Ghostscript. Ver [INSTALLATION.md](./INSTALLATION.md).

### Error: "Permission denied"
**R:** 
- Configura `SUDO_PASSWORD` en `.env`, o
- Ejecuta con `sudo ./start.sh`

### La compresión es muy lenta
**R:**
- Usa **⚡ Paralelo** en lugar de secuencial
- Redondea workers a número más bajo en `.env`
- Aumenta RAM del sistema

### Se quedó colgado
**R:**
- Presiona Ctrl+C
- Revisa permisos
- Verifica espacio en disco

---

## 🔄 Actualización y Mantenimiento

### ¿Cómo actualizo a una versión nueva?
**R:**
```bash
git pull origin main
cargo build --release
```

### ¿Cómo deinstalamos FLARE-DF?
**R:**
```bash
# Simplemente elimina la carpeta
rm -rf ~/code/flare-df
```

Nada se instala globalmente (excepto si usaste `cargo install`).

### ¿Se pueden purgar archivos old?
**R:**
```bash
# Limpiar output viejo
rm output/*

# Limpiar build
cargo clean
```

---

## 🤝 Contribución

### ¿Puedo contribuir?
**R:** ¡Sí! Lee [CONTRIBUTING.md](./CONTRIBUTING.md).

### ¿Cómo reporte un bug?
**R:** Abre un issue en GitHub con:
- Descripción clara
- Pasos para reproducir
- Salida de error
- Tu sistema operativo

### ¿Puedo sugerir una mejora?
**R:** Sí, abre un issue con etiqueta "enhancement".

---

## 📞 Ayuda Adicional

- Documentación: Lee archivos .md en `docs/`
- Problemas: Consulta [TROUBLESHOOTING.md](./TROUBLESHOOTING.md)
- Ejemplos: Ver [EXAMPLES.md](./EXAMPLES.md)

---

**🔥 FLARE-DF** - ¿Preguntas? ¡Consulta aquí!
