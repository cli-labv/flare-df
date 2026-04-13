# ⚡ Guía de Inicio Rápido - FLARE-DF

¡Comprime tu primer PDF en 5 minutos!

## 🎯 Inicio en 5 Pasos

### 1. Ejecuta la Aplicación
```bash
cd /home/victor/code/flare-df
./start.sh
```

### 2. Selecciona Modo
```
📂 Modo input (./input)      ← Comprime PDFs en carpeta local
📁 Modo external (ruta)      ← Comprime PDFs en carpeta externa
```

### 3. Elige Nivel de Compresión
```
💎 Lossless (~10%)           ← Sin pérdida visual
✨ Alta Calidad (~30%)       ← Muy buena calidad
⚖️  Balanceado (~50%)        ← Recomendado
🔥 Agresivo (~70%)          ← Máxima compresión
⚙️  Personalizado (custom)   ← Tu propio porcentaje
```

### 4. Revisa Diagnóstico
```
📊 Resumen
├─ PDFs encontrados: X
├─ Tamaño total: XXX MB
├─ Reducción estimada: XX%
└─ Ahorro proyectado: XXX MB
```

### 5. Confirma y Espera
```
Elige: ⚡ Paralelo (rápido)
¿Iniciar? yes

[Spinners animados durante compresión]

✅ Resultado en ./output/
```

---

## 📂 Estructura de Carpetas

```
flare-df/
├── input/       ← Coloca PDFs aquí (modo input)
├── output/      ← PDFs comprimidos (generados)
└── temp/        ← Archivos temporales (auto-limpieza)
```

---

## 💡 Ejemplos Rápidos

### Ejemplo 1: Modo Input (Más Fácil)

```bash
# 1. Coloca PDF en ./input
cp mi_documento.pdf input/

# 2. Ejecuta
./start.sh

# 3. Selecciona: input → Balanceado → yes

# 4. Resultado en ./output/
ls output/
```

### Ejemplo 2: Modo External (Carpeta Externa)

```bash
# 1. Ejecuta
./start.sh

# 2. Selecciona: external
# 3. Ingresa ruta: /home/usuario/mis_pdfs
# 4. Selecciona: Balanceado
# 5. Confirma: yes

# Resultado se guarda en ./output/
```

### Ejemplo 3: Máxima Compresión

```bash
./start.sh
# input → Agresivo → yes

# Resultado: ~70% de reducción
# Ejemplo: 100 MB → 30 MB
```

---

## 🎯 Recomendaciones por Caso de Uso

### Para Archivos que no Necesitas Editar
```
Nivel: 🔥 Agresivo (~70%)
Reducción: Máxima
Calidad: Suficiente para lectura
```

### Para Documentos de Trabajo
```
Nivel: ⚖️ Balanceado (~50%)
Reducción: Buena
Calidad: Excelente
```

### Para PDFs Críticos (Escaneados, Fotos)
```
Nivel: ✨ Alta Calidad (~30%)
Reducción: Conservadora
Calidad: Prácticamente idéntica
```

### Para Máxima Compatibilidad
```
Nivel: 💎 Lossless (~10%)
Reducción: Mínima
Calidad: 100% idéntica
```

---

## ⚙️ Operaciones Comunes

### Comprimir Un Solo PDF
```bash
./start.sh
→ Selecciona modo input/external
→ Pon el PDF en la carpeta
→ Ejecuta
```

### Comprimir Múltiples PDFs
```bash
./start.sh
→ Selecciona modo input/external
→ Pon todos los PDFs
→ Selecciona ⚡ Paralelo (rápido)
→ Ejecuta
```

### Comprimir Carpeta Completa
```bash
./start.sh
→ Selecciona modo: external
→ Ingresa path de carpeta
→ Ejecuta
```

---

## 📊 Qué Esperar

### Pantalla de Progreso
```
🔥 [1/5] Mi Documento.pdf
→ Paso 1: QPDF...
⠋ Comprimiendo...

→ Paso 2: Ghostscript...
⠙ Comprimiendo...

✔ Comprimido: 100 MB → 50 MB (-50%)
```

### Resultado Final
```
╔════════════════════════════════╗
║ 🔥 FLARE-DF Compresión OK     ║
║ 📄 5 PDFs comprimidos         ║
║ 💾 500 MB → 250 MB (-50%)     ║
║ 📂 Ubicación: ./output        ║
╚════════════════════════════════╝
```

---

## 🎨 Animaciones Bonitas

En cada ejecución ves spinners diferentes:

```
⠋ Comprimiendo...    (dots)
⠙ Comprimiendo...    (dots2)
◐ Comprimiendo...    (arc)
▉ Comprimiendo...    (grow)
🌑 Comprimiendo...   (moon)
🕐 Comprimiendo...   (clock)
```

¡26 estilos diferentes! 🎉

---

## ⚡ Consejos Pro

1. **Modo Paralelo es Más Rápido**
   - Para múltiples PDFs: ⚡ Paralelo
   - Para detalle paso a paso: 🐢 Secuencial

2. **Spinners Aleatorios**
   - Cada ejecución es diferente
   - Mantiene las cosas interesantes 😄

3. **Compresión Inteligente**
   - QPDF optimiza estructura
   - Ghostscript comprime contenido
   - Resultado real y significativo

4. **Auto-limpieza**
   - Carpeta temp se limpia sola
   - Sin necesidad de manual cleanup

---

## 📚 Siguiente Paso

- Más ejemplos: [EXAMPLES.md](./EXAMPLES.md)
- Problemas: [TROUBLESHOOTING.md](./TROUBLESHOOTING.md)
- Ayuda: [FAQ.md](./FAQ.md)

---

**🔥 FLARE-DF** - Compresión rápida y simple
