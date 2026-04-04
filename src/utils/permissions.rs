//! FLARE-PDF - Gestión de permisos
//! ================================
//!
//! Manejo de permisos sudo y contraseñas.

use anyhow::Result;
use colored::Colorize;
use dialoguer::Password;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::config::{project_dir, env_path};

/// Máximo de intentos de contraseña
const MAX_ATTEMPTS: u32 = 3;

/// Gestor de permisos
pub struct PermissionManager;

impl PermissionManager {
    /// Crea una nueva instancia
    pub fn new() -> Self {
        Self
    }
    
    /// Obtiene la contraseña almacenada en .env
    pub fn get_stored_password(&self) -> Option<String> {
        dotenv::dotenv().ok();
        env::var("SUDO_PASSWORD")
            .ok()
            .filter(|p| !p.trim().is_empty())
    }
    
    /// Almacena la contraseña en .env
    pub fn store_password(&self, password: &str) -> Result<()> {
        let path = env_path();
        let content = if path.exists() {
            let existing = fs::read_to_string(&path)?;
            let mut new_content = String::new();
            let mut found = false;
            
            for line in existing.lines() {
                if line.starts_with("SUDO_PASSWORD=") {
                    new_content.push_str(&format!("SUDO_PASSWORD={}\n", password));
                    found = true;
                } else {
                    new_content.push_str(line);
                    new_content.push('\n');
                }
            }
            
            if !found {
                new_content.push_str(&format!("SUDO_PASSWORD={}\n", password));
            }
            
            new_content
        } else {
            format!("SUDO_PASSWORD={}\n", password)
        };
        
        fs::write(&path, content)?;
        Ok(())
    }
    
    /// Invalida la sesión sudo actual
    fn invalidate_sudo(&self) {
        let _ = Command::new("sudo")
            .arg("-k")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    
    /// Valida que la contraseña sudo sea correcta
    /// Usa 'sudo -v' que solo valida credenciales sin ejecutar nada
    fn validate_sudo_password(&self, password: &str) -> bool {
        // Primero invalidar cualquier sesión sudo previa
        self.invalidate_sudo();
        
        // Validar la contraseña con sudo -v
        let mut child = match Command::new("sudo")
            .arg("-S")
            .arg("-v")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(_) => return false,
        };
        
        if let Some(ref mut stdin) = child.stdin {
            let _ = writeln!(stdin, "{}", password);
        }
        
        match child.wait_timeout(Duration::from_secs(10)) {
            Ok(Some(status)) => status.success(),
            _ => false,
        }
    }
    
    /// Ejecuta chmod con sudo
    fn run_chmod(&self, password: &str) -> bool {
        let project = project_dir();
        
        let mut child = match Command::new("sudo")
            .arg("-S")
            .arg("chmod")
            .arg("-R")
            .arg("777")
            .arg(&project)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(_) => return false,
        };
        
        if let Some(ref mut stdin) = child.stdin {
            let _ = writeln!(stdin, "{}", password);
        }
        
        match child.wait_timeout(Duration::from_secs(30)) {
            Ok(Some(status)) => status.success(),
            _ => false,
        }
    }
    
    /// Solicita la contraseña al usuario
    fn prompt_password(&self) -> Result<String> {
        println!();
        let password = Password::new()
            .with_prompt("🔐 Ingresa tu contraseña sudo")
            .interact()?;
        Ok(password)
    }
    
    /// Verifica permisos silenciosamente
    /// Retorna (success, password_used)
    pub fn check_permissions_silent(&self) -> (bool, Option<String>) {
        if let Some(password) = self.get_stored_password() {
            // Primero validar que la contraseña sea correcta
            if self.validate_sudo_password(&password) {
                // Luego ejecutar chmod
                if self.run_chmod(&password) {
                    return (true, Some(password));
                }
            }
        }
        (false, None)
    }
    
    /// Solicita contraseña interactivamente y verifica permisos
    /// Guarda la contraseña en .env si es correcta
    pub fn ensure_permissions_interactive(&self) -> Result<()> {
        println!("{}", "⚠ Contraseña sudo no configurada o incorrecta.".yellow());
        
        let mut attempts = 0;
        
        while attempts < MAX_ATTEMPTS {
            let password = self.prompt_password()?;
            attempts += 1;
            
            print!("{}", "Verificando contraseña... ".dimmed());
            std::io::stdout().flush()?;
            
            // Validar contraseña primero
            if !self.validate_sudo_password(&password) {
                let remaining = MAX_ATTEMPTS - attempts;
                if remaining > 0 {
                    println!(
                        "{} {}",
                        "✖ Contraseña incorrecta.".red(),
                        format!("({} intento{} restante{})", 
                            remaining,
                            if remaining > 1 { "s" } else { "" },
                            if remaining > 1 { "s" } else { "" }
                        ).dimmed()
                    );
                } else {
                    println!("{}", "✖ Contraseña incorrecta.".red());
                }
                continue;
            }
            
            // Contraseña válida, ejecutar chmod
            if self.run_chmod(&password) {
                self.store_password(&password)?;
                println!("{}", "✔ Contraseña correcta y guardada en .env".green());
                return Ok(());
            } else {
                println!("{}", "✖ Error al aplicar permisos.".red());
            }
        }
        
        // Máximo de intentos alcanzado
        println!("\n{}", "❌ Máximo de intentos alcanzado. Saliendo...".red());
        std::process::exit(1);
    }
    
    /// Asegura que el proyecto tiene permisos de escritura
    pub fn ensure_permissions(&self, silent: bool) -> Result<()> {
        // Primero intentar con la contraseña almacenada
        let (success, _) = self.check_permissions_silent();
        
        if success {
            if !silent {
                println!("{}", "Permisos confirmados.".green());
            }
            return Ok(());
        }
        
        // Si falla, pedir interactivamente
        self.ensure_permissions_interactive()
    }
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Extensión para wait con timeout
trait WaitTimeout {
    fn wait_timeout(&mut self, timeout: Duration) -> std::io::Result<Option<std::process::ExitStatus>>;
}

impl WaitTimeout for std::process::Child {
    fn wait_timeout(&mut self, timeout: Duration) -> std::io::Result<Option<std::process::ExitStatus>> {
        use std::thread;
        use std::time::Instant;
        
        let start = Instant::now();
        loop {
            match self.try_wait()? {
                Some(status) => return Ok(Some(status)),
                None => {
                    if start.elapsed() >= timeout {
                        let _ = self.kill();
                        return Ok(None);
                    }
                    thread::sleep(Duration::from_millis(50));
                }
            }
        }
    }
}
