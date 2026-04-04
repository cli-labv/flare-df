#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Verificar si Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo (Rust) no está instalado."
    echo "   Instala Rust desde: https://rustup.rs/"
    exit 1
fi

cd "$ROOT_DIR"

# Crear .env desde .env.example si no existe
if [ ! -f "$ROOT_DIR/.env" ]; then
  if [ -f "$ROOT_DIR/.env.example" ]; then
    echo "📄 Creando archivo .env desde .env.example..."
    cp "$ROOT_DIR/.env.example" "$ROOT_DIR/.env"
  else
    echo "📄 Creando archivo .env vacío..."
    echo "SUDO_PASSWORD=" > "$ROOT_DIR/.env"
  fi
fi

# Crear directorios si no existen
mkdir -p input output temp

# Mostrar banner mientras se compila
show_banner() {
  cat << 'BANNER'

🔥
                        🔥🔥🔥
                      🔥🔥🔥🔥🔥

    ███████╗██╗      █████╗ ██████╗ ███████╗      ██████╗ ███████╗
    ██╔════╝██║     ██╔══██╗██╔══██╗██╔════╝      ██╔══██╗██╔════╝
    █████╗  ██║     ███████║██████╔╝█████╗  █████╗██║  ██║█████╗  
    ██╔══╝  ██║     ██╔══██║██╔══██╗██╔══╝  ╚════╝██║  ██║██╔══╝  
    ██║     ███████╗██║  ██║██║  ██║███████╗      ██████╔╝██║     
    ╚═╝     ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝      ╚═════╝ ╚═╝     

                       v1.0.0 - github.com/flare-df

════════════════════════════════════════════════════════════════════

BANNER
}

# Compilar si no existe el binario o si hay cambios en src
if [ ! -f "target/release/flare-df" ]; then
    show_banner
    echo "🔧 Compilando FLARE-DF por primera vez..."
    echo "   (esto puede tomar 1-2 minutos)"
    echo ""
    cargo build --release --quiet 2>&1 | grep -E "(Compiling|Finished|error)" || true
    echo "✅ Compilación completada!"
    echo ""
elif [ -n "$(find src -newer target/release/flare-df 2>/dev/null)" ]; then
    show_banner
    echo "🔧 Compilando cambios en FLARE-DF..."
    echo ""
    cargo build --release --quiet 2>&1 | grep -E "(Compiling|Finished|error)" || true
    echo "✅ Compilación completada!"
    echo ""
fi

# Ejecutar la aplicación
./target/release/flare-df
