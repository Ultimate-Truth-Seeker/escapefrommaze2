# 🧩 Maze — Juego 3D estilo Raycaster

Bienvenido a **Maze**, un videojuego desarrollado en **Rust** utilizando **raylib-rs**, que implementa un motor gráfico tipo *raycaster* similar a los clásicos como *Wolfenstein 3D*.  
El proyecto incluye un motor hecho desde cero, animaciones simples, efectos visuales, audio, pantallas de menú y un completo HUD en tiempo real.

---

## 🎮 Descripción General

El objetivo del juego es recorrer un laberinto, evitar a los enemigos y llegar a la meta antes de perder toda tu salud.  
El mundo se renderiza usando **raycasting**, lo que simula un entorno 3D mediante cálculos de proyección y distancia.

Maze incluye:

- Motor raycaster hecho desde cero  
- Enemigos con movimiento simple y animaciones  
- Interfaz de usuario completa:  
  - Pantalla de bienvenida  
  - Selección de nivel  
  - Pantalla de pausa  
  - Pantalla de victoria  
  - Pantalla de derrota  
- Efectos visuales (flash rojo al recibir daño)  
- Música de fondo  
- Efectos de sonido  
- Minimap que muestra jugador, enemigos, paredes, inicio y meta  
- Indicador de salud  
- Contador de FPS (normalmente estables cerca de 60)  

---

## 🎬 Video Demo

El video se encuentra en el archivo **demo.mov** que está en la raíz del repositorio. No está puesto en el readme porque tiene sonido y está muy pesado como para ponerlo aquí.

---

## 🧠 Características Técnicas

### 🔸 Motor Raycaster

- Cálculo de rayos por columna  
- Proyección de paredes con diferentes texturas  
- Z‑buffer simple para ordenar profundidad  
- Sistema de sprites para enemigos y meta  
- Función de colisión para evitar atravesar paredes  

### 🔸 HUD y Pantallas

Maze utiliza un sistema de pantallas totalmente dinámico:

- **Main Menu** – Inicio y selección de nivel  
- **Game Screen** – Juego principal  
- **Pause Screen** – Accesible con *ESC*  
- **Victory Screen** – Al llegar a la meta  
- **Defeat Screen** – Al perder toda la salud  

El HUD muestra:

- FPS  
- Salud del jugador  
- Minimapa  
- Efectos visuales de daño  

### 🔸 Audio

- Música de fondo en loop  
- Sonidos al recibir daño y otras interacciones  

---

## 📜 Reglas del Juego

1. El objetivo es **encontrar la salida del laberinto**.  
2. Hay enemigos patrullando el área.  
3. Al hacer contacto con un enemigo, el jugador recibe daño.  
4. Si la salud llega a 0 → **Derrota**.  
5. Si alcanzas la meta → **Victoria**.  
6. El minimapa ayuda a orientarse mostrando paredes, enemigos y tu posición.  

---

## ⌨️ Controles

### 🕹 Movimiento
- **W** – Avanzar  
- **A** – Izquierda  
- **S** – Retroceder  
- **D** – Derecha  

### 🎯 Cámara
- **Mouse** – Girar vista  
- Cursor bloqueado durante el juego  
- Al presionar **ESC**, el cursor se libera y aparece el menú de pausa  

### ⏸ Pausa
- **ESC** – Pausar / reanudar  

---

## 🗺️ Minimapa

El minimapa, ubicado en pantalla, representa:

- El layout completo del laberinto  
- El jugador  
- Los enemigos  
- El punto de inicio  
- La meta  
- Cada pared con un color distinto  

Ayuda a planear rutas y orientarse dentro del nivel.

---

## 🔧 Tecnologías

- **Rust**  
- **raylib-rs**  
- Sistema propio de:  
  - Renderizado por raycasting  
  - Sprite rendering  
  - Gestión de pantallas (UI)  
  - Efectos visuales  
  - Audio  

---

## 🏁 Estado del Proyecto

Actualmente el juego cuenta con:

✔ Motor raycaster funcional  
✔ Enemigos animados  
✔ Mapa con texturas  
✔ Música y efectos de sonido  
✔ HUD completo  
✔ Varias pantallas de UI  
✔ Minimap  
✔ Salud y efectos de daño  
✔ Selección de niveles  

---

## 🙌 Créditos

Desarrollado en Rust como un proyecto experimental inspirado en los clásicos motores 3D de los años 90.