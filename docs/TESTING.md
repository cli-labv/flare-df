# 🧪 Testing - FLARE-DF

Estrategia de testing y guía para ejecutar tests.

## 📋 Tipos de Tests

### 1. CLI Alignment Tests
Verifica el renderizado perfecto de la interfaz terminal.

**Ubicación:** `tests/cli_alignment/`

**Componentes testeados:**
- Alineación de boxes
- Colores y estilos
- Emojis y caracteres especiales
- Múltiples anchos de terminal

**Ejecutar:**
```bash
cargo test --test cli_alignment
```

### 2. Unit Tests
Tests de lógica individual.

**Ubicación:** `src/*/mod.rs`

**Componentes:**
- Parseo de configuración
- Cálculos de compresión
- Validación de rutas

**Ejecutar:**
```bash
cargo test --lib
```

### 3. Integration Tests
Tests de flujo completo.

**Ubicación:** `tests/integration/`

**Verifican:**
- Escaneo de PDFs
- Compresión end-to-end
- Manejo de errores

**Ejecutar:**
```bash
cargo test --test '*'
```

---

## 🚀 Ejecutar Tests

### Todos los Tests
```bash
cargo test
```

### Tests Específicos
```bash
# Solo unit tests
cargo test --lib

# Solo CLI alignment
cargo test --test cli_alignment

# Un test específico
cargo test test_compression_engine
```

### Con Output Detallado
```bash
cargo test -- --nocapture
```

### Mostrar Todos (incluyendo ignorados)
```bash
cargo test -- --include-ignored
```

---

## ✅ Checklist de Testing

Antes de hacer push:

- [ ] `cargo test` - Todos los tests pasan
- [ ] `cargo check` - Sin errores de compilación
- [ ] `cargo fmt --check` - Código formateado
- [ ] `cargo clippy` - Sin warnings de clippy
- [ ] `cargo doc --no-deps` - Documentación genera sin errores

**Script completo:**
```bash
cargo check && \
cargo test && \
cargo fmt --check && \
cargo clippy -- -D warnings && \
cargo doc --no-deps
```

---

## 📝 Escribir Tests

### Estructura Básica
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        // Arrange
        let input = "test";
        
        // Act
        let result = my_function(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

### Ejemplo Real
```rust
#[test]
fn test_compression_level_parse() {
    // Arrange
    let level_str = "Balanceado";
    
    // Act
    let level = CompressionLevel::from_str(level_str);
    
    // Assert
    assert!(level.is_ok());
    assert_eq!(level.unwrap(), CompressionLevel::Balanced);
}
```

---

## 🎯 Cobertura de Tests

### Cubierto (✅)
- CLI Alignment y renderizado
- Parsing de configuración
- Validación de rutas

### Pendiente (🔄)
- Tests de compresión end-to-end
- Tests de error handling
- Tests de permisos

### No Testeable Fácilmente (⚠️)
- Interacción de usuario (requiere mock)
- Sistema de archivos (requiere temp files)
- Procesos externos (qpdf, ghostscript)

---

## 📊 Resultados de Tests Actuales

```
test result: ok. X passed; 0 failed; 0 ignored

Módulos testeados:
├─ cli_alignment ✅
├─ models ✅
└─ utils ✅
```

---

## 🔍 Debugging de Tests

### Imprimir durante tests
```rust
#[test]
fn test_with_debug() {
    let value = some_computation();
    println!("Debug: {:?}", value);  // Usa --nocapture
    assert!(value > 0);
}
```

### Ejecutar con output
```bash
cargo test test_name -- --nocapture
```

### Detener en primer fallo
```bash
cargo test -- --test-threads=1
```

---

## 🏗️ CI/CD Testing

En producción (GitHub Actions):

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo test --verbose
      - run: cargo clippy -- -D warnings
```

---

## 📈 Mejor Prácticas

1. **Test Primero (TDD):**
   - Escribe test
   - Escribe código
   - Verifica test pase

2. **Nombres Descriptivos:**
   ```rust
   #[test]
   fn test_should_handle_empty_input_gracefully() { }
   ```

3. **Un Aserto por Test:**
   - Más específico y claro

4. **Usa Fixtures:**
   ```rust
   fn create_test_pdf() -> PathBuf {
       // Helper para tests
   }
   ```

5. **Mock When Needed:**
   ```rust
   // Mockea funciones externas
   mock_filesystem();
   assert!(result.is_ok());
   ```

---

## 🐛 Troubleshooting de Tests

### Tests fallan después de cambios
```bash
cargo clean
cargo test
```

### Error: "test timed out"
```bash
# Aumentar timeout
cargo test -- --test-threads=1 --timeout 60
```

### Conflicto de archivos temporales
```bash
# Usar paths únicos en tests
let temp_dir = temp_dir::create_unique();
```

---

## 📚 Recursos

- [The Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust by Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [Cargo Test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

---

**🔥 FLARE-DF** - Código bien testeado es código confiable
