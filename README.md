# Ray Tracer Cube

Ray tracer académico en Rust que dibuja un cubo AABB rojo con iluminación
Lambertiana difusa y una cámara orbital. No contiene reflexión, refracción,
componente especular, texturas ni sombras.

## Ejecutar

```powershell
cargo run
```

Controles: flechas o `WASD` para orbitar; `Esc` para salir.

Para generar una imagen sin abrir una ventana:

```powershell
cargo run -- --render cube.bmp
```
