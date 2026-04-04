# 🎨 Sistema de Spinners Aleatorios

## Descripción
FLARE-DF ahora incluye 26 estilos diferentes de spinners que se seleccionan aleatoriamente en cada ejecución, proporcionando una experiencia visual más dinámica y atractiva durante la compresión de PDFs.

## Características

### ✨ Spinners Disponibles
- **Dots**: Variantes clásicas con puntos en movimiento (dots, dots2, dots3, etc.)
- **Lines**: Líneas animadas (line, line2)
- **Arcs**: Círculos y arcos rotativos (arc, arc2)
- **Squares**: Cuadrados y bloques (square, square2, box)
- **Growth**: Barras que crecen y decrecen (grow, vertical)
- **Arrows**: Flechas direccionales
- **Shapes**: Triángulos y formas geométricas
- **Emojis**: Luna 🌑 y Reloj 🕐 (si el terminal soporta emojis)

### 🔄 Comportamiento

1. **Selección Aleatoria**: En cada ejecución de `./start.sh`, se selecciona un spinner aleatorio usando el timestamp actual como semilla.

2. **Consistencia**: El mismo spinner se usa durante toda la compresión (ambos pasos si es modo no-lossless).

3. **Visualización por Pasos**:
   ```
   → Paso 1: QPDF (optimización estructural)...
   ⠋ Comprimiendo...
   
   → Paso 2: Ghostscript (compresión inteligente)...
   ⠙ Comprimiendo...
   ```

4. **Velocidad**: 80ms por frame (12.5 fps) para animaciones suaves y no invasivas.

## Implementación Técnica

### Archivo Modificado
- `src/engines/pipeline.rs`: Agregado array de spinners y funciones helper

### Funciones Clave

```rust
const SPINNERS: &[&str] = &[
    "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",  // dots
    "⣾⣽⣻⢿⡿⣟⣯⣷",      // dots9
    // ... 24 más
];

fn get_random_spinner() -> &'static str {
    // Usa timestamp como semilla para selección aleatoria
}

fn create_spinner(message: &str) -> ProgressBar {
    // Crea spinner con estilo aleatorio
}
```

### Integración con Indicatif
Usa la librería `indicatif` (ya incluida en el proyecto) para renderizar los spinners:
- No requiere dependencias adicionales
- Compatible con cualquier terminal moderno
- Rendimiento óptimo

## Uso

Simplemente ejecuta la aplicación normalmente:

```bash
./start.sh
```

En cada ejecución verás un spinner diferente durante el proceso de compresión.

## Ejemplo de Salida

```
🔥 [1/1] Mi PDF.pdf
─────────────────────────────────────────────────
   📄 Tamaño original: 61.5 MiB
   → Paso 1: QPDF (optimización estructural)...
   ◐ Comprimiendo...
   → Paso 2: Ghostscript (compresión inteligente)...
   ◓ Comprimiendo...
   ✔ Comprimido: 61.5 MiB → 7.6 MiB (-87.6%)
```

## Notas

- Los spinners se limpian automáticamente al finalizar cada paso
- No interfieren con el output final ni las estadísticas
- Funcionan tanto en modo paralelo como secuencial
- Compatible con la funcionalidad de compresión existente (sin cambios en la lógica de negocio)

---

*Implementado con ❤️ usando Rust e Indicatif*
