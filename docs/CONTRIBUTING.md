# 🤝 Guía de Contribución - FLARE-DF

¡Gracias por tu interés en contribuir a FLARE-DF! Este documento te guiará a través del proceso.

## 📋 Tabla de Contenidos

- [Código de Conducta](#código-de-conducta)
- [¿Cómo puedo contribuir?](#cómo-puedo-contribuir)
- [Configuración del entorno](#configuración-del-entorno)
- [Proceso de desarrollo](#proceso-de-desarrollo)
- [Estándares de código](#estándares-de-código)
- [Proceso de Pull Request](#proceso-de-pull-request)

## 📜 Código de Conducta

Este proyecto se rige por un código de conducta de respeto mutuo. Al participar, se espera que mantengas un ambiente profesional y acogedor.

## 🎯 ¿Cómo puedo contribuir?

### Reportar Bugs

Los bugs se rastrean como issues de GitHub. Cuando crees un issue, incluye:

- **Descripción clara** del problema
- **Pasos para reproducir** el comportamiento
- **Comportamiento esperado** vs. **comportamiento actual**
- **Capturas de pantalla** si aplica
- **Información del sistema**: OS, versión de Rust, versión de QPDF/Ghostscript

### Sugerir Mejoras

Las sugerencias de mejoras también se rastrean como issues. Incluye:

- **Descripción clara** de la mejora propuesta
- **Justificación**: ¿Por qué es útil?
- **Ejemplos** de cómo funcionaría

### Contribuir con Código

1. **Fork** el repositorio
2. **Crea una rama** para tu feature (`git checkout -b feature/amazing-feature`)
3. **Realiza tus cambios** siguiendo los estándares de código
4. **Escribe tests** si aplica
5. **Commit** tus cambios con mensajes descriptivos
6. **Push** a tu fork (`git push origin feature/amazing-feature`)
7. **Abre un Pull Request**

## 🛠️ Configuración del Entorno

### Requisitos

- Rust 1.70+ (edición 2021)
- QPDF (`apt install qpdf` o `brew install qpdf`)
- Ghostscript (`apt install ghostscript` o `brew install ghostscript`)
- Git

### Instalación para Desarrollo

```bash
# Clonar tu fork
git clone https://github.com/TU_USUARIO/flare-df.git
cd flare-df

# Configurar upstream
git remote add upstream https://github.com/ORIGINAL/flare-df.git

# Configurar entorno
cp .env.example .env
nano .env  # Configura tus valores

# Instalar herramientas de desarrollo
rustup component add rustfmt clippy

# Verificar instalación
cargo check
```

## 🔄 Proceso de Desarrollo

### 1. Mantén tu fork actualizado

```bash
git fetch upstream
git checkout main
git merge upstream/main
```

### 2. Crea una rama para tu feature

```bash
git checkout -b feature/mi-nueva-feature
```

### 3. Desarrolla y prueba

```bash
# Desarrollo iterativo
cargo check  # Verificación rápida
cargo build  # Compilación

# Tests
cargo test

# Linters y formato
cargo fmt     # Formatear código
cargo clippy  # Análisis estático
```

### 4. Commit tus cambios

```bash
git add .
git commit -m "feat: Descripción clara del cambio"
```

#### Convención de mensajes de commit

Usamos [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` Nueva funcionalidad
- `fix:` Corrección de bug
- `docs:` Cambios en documentación
- `style:` Cambios de formato (no afectan el código)
- `refactor:` Refactorización de código
- `perf:` Mejoras de rendimiento
- `test:` Añadir o modificar tests
- `chore:` Tareas de mantenimiento

Ejemplos:
```
feat: Add custom compression percentage mode
fix: Resolve permission error on external folders
docs: Update README with Ghostscript installation
refactor: Extract compression pipeline to separate module
```

## 📏 Estándares de Código

### Estilo de Código

- Sigue el estilo de Rust estándar (`rustfmt`)
- Máximo 100 caracteres por línea
- Usa nombres descriptivos para variables y funciones
- Documenta funciones públicas con `///`

### Estructura de Archivos

```rust
//! Descripción del módulo
//! 
//! Detalles adicionales sobre qué hace este módulo

use std::path::Path;
use anyhow::Result;

/// Documentación de la función
/// 
/// # Argumentos
/// * `input` - Descripción del parámetro
/// 
/// # Retorna
/// Descripción del valor de retorno
pub fn mi_funcion(input: &Path) -> Result<String> {
    // Implementación
}
```

### Tests

Escribe tests para nuevas funcionalidades:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mi_funcion() {
        // Arrange
        let input = Path::new("test.pdf");
        
        // Act
        let result = mi_funcion(input);
        
        // Assert
        assert!(result.is_ok());
    }
}
```

### Documentación

- Documenta todas las funciones públicas
- Incluye ejemplos en la documentación cuando sea útil
- Actualiza el README si añades nuevas features

## 🔍 Proceso de Pull Request

### Antes de Enviar

1. ✅ El código compila sin errores (`cargo check`)
2. ✅ Todos los tests pasan (`cargo test`)
3. ✅ El código está formateado (`cargo fmt`)
4. ✅ No hay warnings de clippy (`cargo clippy`)
5. ✅ La documentación está actualizada
6. ✅ Has probado manualmente tus cambios

### Plantilla de PR

Cuando abras un PR, incluye:

```markdown
## Descripción
Breve descripción de los cambios

## Tipo de cambio
- [ ] Bug fix (cambio que corrige un issue)
- [ ] Nueva funcionalidad (cambio que añade funcionalidad)
- [ ] Breaking change (cambio que puede romper compatibilidad)
- [ ] Documentación

## ¿Cómo se ha probado?
Describe cómo probaste tus cambios

## Checklist
- [ ] Mi código sigue el estilo del proyecto
- [ ] He realizado una auto-revisión de mi código
- [ ] He comentado el código en áreas difíciles de entender
- [ ] He actualizado la documentación
- [ ] Mis cambios no generan nuevos warnings
- [ ] He añadido tests que prueban mi fix o feature
- [ ] Tests nuevos y existentes pasan localmente
```

### Revisión

- Un mantenedor revisará tu PR
- Puede haber comentarios o solicitudes de cambios
- Una vez aprobado, tu PR será merged

## 🏗️ Áreas para Contribuir

### Fácil (Good First Issue)

- Mejorar mensajes de error
- Añadir más spinners animados
- Traducir documentación
- Corregir typos

### Intermedio

- Optimizar rendimiento
- Mejorar tests
- Añadir nuevas opciones de CLI

### Avanzado

- Implementar nuevos motores de compresión
- Soporte para formatos adicionales
- Mejoras en el pipeline de compresión

## 📞 ¿Necesitas Ayuda?

- Abre un issue con la etiqueta `question`
- Revisa issues existentes
- Lee la documentación en `/docs`

## 🎉 Reconocimientos

Todos los contribuidores serán reconocidos en el README.

---

¡Gracias por contribuir a FLARE-DF! 🔥
