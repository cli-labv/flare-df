# 📚 Ejemplos de Uso - FLARE-DF

Ejemplos prácticos y casos de uso reales.

## 🎯 Ejemplo 1: Compresión Básica (Modo Input)

```bash
# 1. Coloca PDF en carpeta input
cp ~/Downloads/documento.pdf ./input/

# 2. Ejecuta FLARE-DF
./start.sh

# 3. Selecciona opciones
📂 Modo input
⚖️  Balanceado
⚡ Paralelo
yes

# 4. Resultado
✔ Comprimido: 100 MB → 50 MB (-50%)

# 5. Descarga resultado
ls output/
```

---

## 🎯 Ejemplo 2: Múltiples PDFs (Carpeta Externa)

```bash
# 1. Tengo PDFs en otra carpeta
ls ~/mis_documentos/
documento1.pdf
documento2.pdf
documento3.pdf

# 2. Ejecuta FLARE-DF
./start.sh

# 3. Selecciona opciones
📁 Modo external
/home/usuario/mis_documentos
🔥 Agresivo
⚡ Paralelo
yes

# 4. Resultado
✔ 3 PDFs comprimidos
💾 300 MB → 90 MB (-70%)

# 5. Descarga desde output/
cp output/* ~/mis_documentos_comprimidos/
```

---

## 🎯 Ejemplo 3: Máxima Compresión (Personalizado)

```bash
# 1. Quiero 80% de compresión
./start.sh

# 2. Selecciona opciones
📂 Modo input
⚙️  Personalizado
(ingresa: 80)
yes

# 3. Resultado
✔ Comprimido: 50 MB → 10 MB (-80%)
```

---

## 🎯 Ejemplo 4: Sin Pérdida (Lossless)

```bash
# 1. Documento importante, sin perder calidad
./start.sh

# 2. Selecciona opciones
📂 Modo input
💎 Lossless
⚡ Paralelo
yes

# 3. Resultado (pequeña reducción)
✔ Comprimido: 50 MB → 47.5 MB (-5%)
✅ 100% sin pérdida visual
```

---

## 🎯 Ejemplo 5: Paso a Paso (Secuencial)

```bash
# 1. Quiero ver cada paso en detalle
./start.sh

# 2. Selecciona opciones
📂 Modo input
⚖️  Balanceado
🐢 Secuencial
yes

# 3. Ves cada paso
[1/1] mi_documento.pdf
→ Paso 1: QPDF (optimización estructural)...
⠋ Comprimiendo...
→ Paso 2: Ghostscript (compresión inteligente)...
⠙ Comprimiendo...
✔ Comprimido: 100 MB → 50 MB (-50%)
```

---

## 💼 Caso de Uso 1: Compartir PDFs por Email

**Problema:** PDF de 50 MB, email permite máximo 25 MB

**Solución:**
```bash
./start.sh
📂 input
🔥 Agresivo  # 60-80% reducción
yes

# Resultado: ~15 MB
# Ahora puedes enviarlo por email ✅
```

---

## 💼 Caso de Uso 2: Archivar Documentos

**Problema:** Guardar años de documentos, ahorrar espacio

**Solución:**
```bash
./start.sh
📁 external → /ruta/a/documentos_2024
⚖️  Balanceado  # Balance óptimo
⚡ Paralelo  # Más rápido
yes

# Reducción: 40-60%
# Ejemplo: 500 GB → 200-300 GB ✅
```

---

## 💼 Caso de Uso 3: Documentos Escaneados

**Problema:** Scans ocupan mucho espacio

**Solución:**
```bash
./start.sh
📁 external → /ruta/a/scans
✨ Alta Calidad  # Mantiene legibilidad
⚡ Paralelo
yes

# Reducción: 20-40%
# Calidad: Excelente, imperceptible ✅
```

---

## 💼 Caso de Uso 4: Reducción Máxima

**Problema:** Espacio en disco crítico

**Solución:**
```bash
./start.sh
📂 input
🔥 Agresivo  # Máxima compresión
⚡ Paralelo
yes

# Reducción: 60-80%
# Calidad: Suficiente para lectura ✅
```

---

## 💼 Caso de Uso 5: Reporte Automatizado

**Problema:** Comprimir reportes mensualmente

**Solución (Script Bash):**
```bash
#!/bin/bash
# Comprimir reportes cada mes

cd ~/flare-df

# Copia reportes del mes anterior
cp ~/reportes/2024-03/*.pdf input/

# Comprime
./start.sh  # (Automatizado con inputs)

# Archiva
mv output/* ~/archive/reportes_comprimidos/

# Limpia
rm input/*
```

---

## 📊 Comparativa de Resultados

### PDF de Catálogo (Muchas Imágenes)
```
Original: 150 MB
─────────────────────
💎 Lossless:     140 MB (-7%)
✨ Alta Calidad: 120 MB (-20%)
⚖️  Balanceado:  75 MB (-50%)
🔥 Agresivo:     30 MB (-80%)
```

### PDF de Documento Texto
```
Original: 20 MB
─────────────────────
💎 Lossless:     19 MB (-5%)
✨ Alta Calidad: 18 MB (-10%)
⚖️  Balanceado:  15 MB (-25%)
🔥 Agresivo:     8 MB (-60%)
```

### PDF Escaneo (Mixto)
```
Original: 80 MB
─────────────────────
💎 Lossless:     76 MB (-5%)
✨ Alta Calidad: 60 MB (-25%)
⚖️  Balanceado:  48 MB (-40%)
🔥 Agresivo:     20 MB (-75%)
```

---

## ⚡ Benchmark: Tiempo de Compresión

**Sistema:** i7-8700K, 32 GB RAM

```
5 PDFs de 20 MB cada uno (100 MB total)

Paralelo (⚡):        30 segundos
Secuencial (🐢):    120 segundos

Diferencia: 4x más rápido con paralelo
```

---

## 🎓 Tips Pro

### Tip 1: Pre-visualiza Antes
```bash
# Comprime uno de prueba primero
# Antes de hacer lotes enteros
```

### Tip 2: Mezcla Niveles Diferentes
```bash
# Para carpeta con PDFs variados:
# - Documentos texto → Balanceado
# - Fotos/Catálogos → Agresivo  
# - Archivos → Lossless
```

### Tip 3: Usa Personalizado para Control
```bash
# Cuando necesitas exacto:
⚙️  Personalizado: 55%
# En lugar de seleccionar predefinido
```

### Tip 4: Modo Paralelo Siempre que Puedas
```bash
# ⚡ Paralelo es siempre más rápido
# Usa 🐢 Secuencial solo para debug
```

### Tip 5: Monitorea Primer Lote
```bash
# Verifica resultado antes de automatizar
# Asegúrate que nivel es correcto
```

---

## 🔄 Flujo de Trabajo Recomendado

```
1. Agrupa PDFs por tipo
   ├─ Documentos texto
   ├─ Escaneados
   └─ Imágenes/Catálogos

2. Para cada grupo:
   ├─ Prueba con 1 PDF
   ├─ Verifica resultado
   └─ Si OK, comprime todo el grupo

3. Usa nivel apropiado:
   ├─ Texto: Balanceado
   ├─ Escaneados: Alta Calidad
   └─ Imágenes: Agresivo

4. Archivo resultados
   ├─ Mantén copia original
   ├─ Guarda comprimidos separado
   └─ Documenta niveles usados
```

---

## 📈 Resultados Esperados

### Por Tipo de PDF

| Tipo | Balanceado | Agresivo |
|------|-----------|----------|
| Texto puro | 20-30% | 40-60% |
| Con imágenes | 40-60% | 60-80% |
| Escaneo B&N | 30-50% | 50-70% |
| Fotos color | 50-70% | 70-85% |
| Ya comprimido | 5-10% | 5-15% |

---

**🔥 FLARE-DF** - Ejemplos prácticos listos para usar
