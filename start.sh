#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

IMAGE_NAME="flare-df-local:1.0.0"
CONTAINERFILE="$ROOT_DIR/Containerfile"
PODMAN_STORE_ROOT="$ROOT_DIR/.podman/root"
PODMAN_STORE_RUNROOT="$ROOT_DIR/.podman/runroot"
PODMAN_OPTS=(--root "$PODMAN_STORE_ROOT" --runroot "$PODMAN_STORE_RUNROOT")
PKG_MANAGER=""
VENV_DIR="$ROOT_DIR/.tooling/.venv"
PYTHON_BIN="$VENV_DIR/bin/python"
PIP_BIN="$VENV_DIR/bin/pip"
LOCAL_INSTALL_LOG="$ROOT_DIR/.tooling/local-install.log"

prepare_workspace() {
  mkdir -p input output temp
  mkdir -p .tooling/cargo .tooling/home
  mkdir -p "$PODMAN_STORE_ROOT" "$PODMAN_STORE_RUNROOT"
  mkdir -p .tooling

  if [ ! -f "$ROOT_DIR/.env" ]; then
    if [ -f "$ROOT_DIR/.env.example" ]; then
      echo "📄 Creando archivo .env desde .env.example..."
      cp "$ROOT_DIR/.env.example" "$ROOT_DIR/.env"
    else
      echo "📄 Creando archivo .env vacío..."
      echo "SUDO_PASSWORD=" > "$ROOT_DIR/.env"
    fi
  fi
}

setup_bootstrap_venv() {
  if ! command -v python3 >/dev/null 2>&1; then
    echo "❌ Python 3 no está instalado. Se requiere para el loader de inicio."
    exit 1
  fi

  if [ ! -d "$VENV_DIR" ]; then
    echo "🐍 Creando entorno virtual de arranque..."
    python3 -m venv "$VENV_DIR"
  fi

  # shellcheck disable=SC1091
  source "$VENV_DIR/bin/activate"

  if ! "$PYTHON_BIN" -c "import alive_progress" >/dev/null 2>&1; then
    echo "📦 Instalando dependencias del entorno virtual..."
    "$PIP_BIN" install --quiet --upgrade pip >/dev/null 2>&1
    "$PIP_BIN" install --quiet alive-progress >/dev/null 2>&1
  fi
}

run_with_alive_loader() {
  local message="$1"
  local log_file="$2"
  shift 2

  : >"$log_file"
  ("$@" >"$log_file" 2>&1) &
  local task_pid=$!

  "$PYTHON_BIN" - "$task_pid" "$message" <<'PY' >/dev/null 2>&1 || true
import os
import sys
import time

pid = int(sys.argv[1])
text = sys.argv[2]

try:
    from alive_progress import alive_spinner
except Exception:
    while True:
        try:
            os.kill(pid, 0)
        except OSError:
            break
        time.sleep(0.2)
    raise SystemExit(0)

with alive_spinner(text=text, spinner="dots") as spinner:
    while True:
        try:
            os.kill(pid, 0)
        except OSError:
            break
        spinner()
        time.sleep(0.12)
PY

  wait "$task_pid"
}

ensure_sudo_session() {
  if [ "$(id -u)" -eq 0 ] || [ "$PKG_MANAGER" = "brew" ]; then
    return 0
  fi

  if ! command -v sudo >/dev/null 2>&1; then
    echo "❌ Se requiere sudo para instalar binarios en modo local."
    return 1
  fi

  echo "🔐 Validando permisos sudo para instalación local..."
  sudo -v
}

detect_package_manager() {
  if command -v apt-get >/dev/null 2>&1; then
    PKG_MANAGER="apt"
  elif command -v dnf >/dev/null 2>&1; then
    PKG_MANAGER="dnf"
  elif command -v pacman >/dev/null 2>&1; then
    PKG_MANAGER="pacman"
  elif command -v zypper >/dev/null 2>&1; then
    PKG_MANAGER="zypper"
  elif command -v brew >/dev/null 2>&1; then
    PKG_MANAGER="brew"
  else
    PKG_MANAGER=""
  fi
}

run_as_root() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  else
    sudo -n "$@"
  fi
}

install_system_packages() {
  local -a packages=("$@")
  if [ "${#packages[@]}" -eq 0 ]; then
    return 0
  fi

  case "$PKG_MANAGER" in
    apt)
      run_with_alive_loader "Actualizando índices APT" "$LOCAL_INSTALL_LOG" run_as_root apt-get update -y
      run_with_alive_loader "Instalando binarios externos (APT)" "$LOCAL_INSTALL_LOG" run_as_root apt-get install -y "${packages[@]}"
      ;;
    dnf)
      run_with_alive_loader "Instalando binarios externos (DNF)" "$LOCAL_INSTALL_LOG" run_as_root dnf install -y "${packages[@]}"
      ;;
    pacman)
      run_with_alive_loader "Instalando binarios externos (Pacman)" "$LOCAL_INSTALL_LOG" run_as_root pacman -Sy --noconfirm "${packages[@]}"
      ;;
    zypper)
      run_with_alive_loader "Instalando binarios externos (Zypper)" "$LOCAL_INSTALL_LOG" run_as_root zypper --non-interactive install "${packages[@]}"
      ;;
    brew)
      run_with_alive_loader "Instalando binarios externos (Brew)" "$LOCAL_INSTALL_LOG" brew install "${packages[@]}"
      ;;
    *)
      echo "❌ No se detectó gestor de paquetes compatible."
      return 1
      ;;
  esac
}

ensure_rust_toolchain_local() {
  if command -v cargo >/dev/null 2>&1 && command -v rustc >/dev/null 2>&1; then
    return 0
  fi

  if ! command -v curl >/dev/null 2>&1; then
    echo "❌ Falta curl para instalar Rust en modo usuario."
    echo "   Instálalo manualmente y vuelve a ejecutar ./start.sh"
    exit 1
  fi

  echo "🦀 Instalando Rust en entorno de usuario (rustup)..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal

  if [ -f "$HOME/.cargo/env" ]; then
    # shellcheck disable=SC1090
    source "$HOME/.cargo/env"
  fi

  if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ Rust se instaló, pero cargo no está en PATH."
    echo "   Ejecuta: source \$HOME/.cargo/env"
    exit 1
  fi
}

ensure_local_dependencies() {
  detect_package_manager
  if [ -z "$PKG_MANAGER" ]; then
    echo "❌ No se pudo detectar gestor de paquetes para modo local."
    exit 1
  fi

  local -a missing=()
  command -v qpdf >/dev/null 2>&1 || missing+=("qpdf")
  command -v gs >/dev/null 2>&1 || missing+=("ghostscript")

  if [ "${#missing[@]}" -gt 0 ]; then
    ensure_sudo_session
    echo "📦 Instalando binarios externos requeridos: ${missing[*]}"
    case "$PKG_MANAGER" in
      apt) install_system_packages qpdf ghostscript ca-certificates ;;
      dnf) install_system_packages qpdf ghostscript ca-certificates ;;
      pacman) install_system_packages qpdf ghostscript ca-certificates ;;
      zypper) install_system_packages qpdf ghostscript ca-certificates ;;
      brew) install_system_packages qpdf ghostscript ;;
    esac

    if [ $? -ne 0 ]; then
      echo "❌ Falló la instalación local de binarios."
      echo "   Revisa el log: $LOCAL_INSTALL_LOG"
      exit 1
    fi
    echo "✅ Binarios locales listos."
  fi

  ensure_rust_toolchain_local
}

verify_local_runtime() {
  local -a missing=()
  command -v qpdf >/dev/null 2>&1 || missing+=("qpdf")
  command -v gs >/dev/null 2>&1 || missing+=("ghostscript")
  command -v cargo >/dev/null 2>&1 || missing+=("cargo")

  if [ "${#missing[@]}" -gt 0 ]; then
    echo "❌ Faltan dependencias en modo local: ${missing[*]}"
    exit 1
  fi
}

compile_and_run_host() {
  if [ ! -f "target/release/flare-df" ] || [ -n "$(find src -newer target/release/flare-df 2>/dev/null)" ]; then
    echo "🔧 Compilando FLARE-DF en host..."
    cargo build --release --quiet
    echo "✅ Compilación completada"
  fi
  ./target/release/flare-df
}

ensure_podman() {
  if ! command -v podman >/dev/null 2>&1; then
    echo "❌ Podman no está instalado."
    echo "   Instálalo o elige modo local."
    exit 1
  fi
}

build_container_if_needed() {
  local image_exists=0
  if podman "${PODMAN_OPTS[@]}" image exists "$IMAGE_NAME"; then
    image_exists=1
  fi

  if [ "$image_exists" -eq 0 ] || [ "$CONTAINERFILE" -nt "$ROOT_DIR/.tooling/.container_built_stamp" ]; then
    echo "🐳 Construyendo entorno local FLARE-DF (Podman)..."
    podman "${PODMAN_OPTS[@]}" build -t "$IMAGE_NAME" -f "$CONTAINERFILE" "$ROOT_DIR"
    date +%s > "$ROOT_DIR/.tooling/.container_built_stamp"
  fi
}

run_app_container() {
  local -a tty_flags=()
  if [ -t 0 ] && [ -t 1 ]; then
    tty_flags=(-it)
  else
    tty_flags=(-i)
  fi

  podman "${PODMAN_OPTS[@]}" run --rm "${tty_flags[@]}" \
    --userns=keep-id \
    -e CARGO_HOME=/workspace/.tooling/cargo \
    -e CARGO_TARGET_DIR=/workspace/target \
    -e HOME=/workspace/.tooling/home \
    -v "$ROOT_DIR:/workspace:Z" \
    -w /workspace \
    "$IMAGE_NAME" \
    bash -lc '
      set -euo pipefail
      if [ ! -f target/release/flare-df ] || [ -n "$(find src -newer target/release/flare-df 2>/dev/null)" ]; then
        echo "🔧 Compilando FLARE-DF en contenedor..."
        cargo build --release --quiet
        echo "✅ Compilación completada"
      fi
      ./target/release/flare-df
    '
}

run_with_podman() {
  ensure_podman
  build_container_if_needed
  run_app_container
}

run_with_local() {
  ensure_local_dependencies
  verify_local_runtime
  compile_and_run_host
}

select_start_mode() {
  if [ ! -t 0 ]; then
    echo "podman"
    return
  fi

  local current=0
  local render_lines=0
  local -a labels=(
    "🖥️  Local (solo binarios externos)"
    "🐳 Podman (aislado, recomendado)"
  )
  local -a values=("local" "podman")

  while true; do
    if [ "$render_lines" -gt 0 ]; then
      printf "\033[%dA" "$render_lines" >&2
      printf "\033[J" >&2
    fi

    echo "🔥 FLARE-DF - Selecciona tipo de arranque" >&2
    echo "↑/↓ para mover, Enter para confirmar" >&2

    for i in "${!labels[@]}"; do
      if [ "$i" -eq "$current" ]; then
        printf "  \033[1;35m❯ %s\033[0m\n" "${labels[$i]}" >&2
      else
        printf "    %s\n" "${labels[$i]}" >&2
      fi
    done
    render_lines=$((2 + ${#labels[@]}))

    IFS= read -rsn1 key
    case "$key" in
      "")
        echo "${values[$current]}"
        return
        ;;
      $'\x1b')
        IFS= read -rsn2 key2
        case "$key2" in
          "[A") current=$(( (current - 1 + ${#labels[@]}) % ${#labels[@]} )) ;;
          "[B") current=$(( (current + 1) % ${#labels[@]} )) ;;
        esac
        ;;
    esac
  done
}

prepare_workspace
setup_bootstrap_venv
MODE="$(select_start_mode)"

case "$MODE" in
  podman) run_with_podman ;;
  local) run_with_local ;;
  *)
    echo "❌ Modo de inicio no soportado: $MODE"
    exit 1
    ;;
esac
