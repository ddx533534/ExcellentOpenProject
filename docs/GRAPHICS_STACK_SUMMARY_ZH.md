# 图形栈概念总结

本文整理前面讨论过的几个核心概念，包括：

- `Skia`
- `wgpu`
- `Vulkan`
- `OpenGL / OpenGL ES`
- `Metal`
- `WebGPU`
- 各平台常见图形栈

目标是用尽量简洁的中文，把这些概念放到同一套分层里理解。

## 1. 一句话总览

- `Skia` 解决的是“怎么把 2D 内容画好”。
- `wgpu` 解决的是“怎么更统一地控制 GPU”。
- `Vulkan / OpenGL / Metal / WebGPU` 解决的是“程序如何调用底层图形能力”。
- `GPU 驱动` 则是真正和硬件打交道的厂商实现。

## 2. 分层理解

可以把常见图形系统粗略分成四层：

1. UI / 图形引擎层
   例如：`Skia`、`Core Graphics`、`Vello`

2. GPU 抽象层
   例如：`wgpu`

3. 底层图形 API 层
   例如：`Vulkan`、`OpenGL`、`Metal`、`WebGPU`、`Direct3D`

4. 驱动 / GPU 硬件层
   例如：NVIDIA、AMD、Intel、Apple、Qualcomm 等厂商驱动及真实 GPU

## 3. 核心术语总结

### 3.1 Skia

- `Skia` 是一个成熟的 2D 图形引擎。
- 它负责文字、图片、路径、阴影、渐变、裁剪、透明混合等高层绘制能力。
- 它不是单纯桥接层，也不是只做预处理。
- 它本身具备很强的绘制能力，只是最终执行通常还要落到某个具体 backend 上。

一句话总结：

`Skia` 是“真正负责大部分 2D 绘制逻辑的主厨”。

### 3.2 wgpu

- `wgpu` 是 Rust 世界里的跨平台 GPU 抽象层。
- 它的目标是用统一接口抹平不同底层图形 API 的差异。
- 它通常对接的目标包括：`Vulkan`、`Metal`、`Direct3D 12`、`WebGPU`，有些场景也会兼容 `OpenGL`。
- `wgpu` 本身不是高层 2D 绘图引擎。
- 它不会自动替你做好文本排版、阴影、渐变这些高层绘制能力。

一句话总结：

`wgpu` 是“统一控制 GPU 的工程抽象层”，适合自己造渲染系统的人。

### 3.3 Vulkan

- `Vulkan` 是现代、显式、低开销的底层 GPU API。
- 它给开发者更强控制权，但复杂度也更高。
- 它常用于高性能原生渲染系统、游戏引擎、现代图形应用。

一句话总结：

`Vulkan` 是“现代底层 GPU API，控制力强，但更难用”。

### 3.4 OpenGL / OpenGL ES

- `OpenGL` 是老牌跨平台图形 API。
- `OpenGL ES` 是移动和嵌入式方向的变体。
- 它们的抽象更高，历史更久，兼容面很广，但设计也更老。

一句话总结：

`OpenGL / OpenGL ES` 是“历史悠久、兼容性广、但相对老一代的图形 API”。

### 3.5 Metal

- `Metal` 是 Apple 自家的现代底层 GPU API。
- 它服务于 `macOS` 和 `iOS`。
- 在 Apple 平台上，它是当前主流原生 GPU 路线。

一句话总结：

`Metal` 是“Apple 平台自己的现代 GPU API”。

### 3.6 WebGPU

- `WebGPU` 是 Web 平台的现代 GPU API 标准。
- 它处在和 `Vulkan / Metal / Direct3D 12` 相近的层级。
- 它不是 UI 框架，也不是图形引擎。
- 浏览器内部会把 `WebGPU` 映射到底层原生图形 API。

一句话总结：

`WebGPU` 是“浏览器世界的现代底层 GPU API 标准”。

### 3.7 GPU 驱动

- 驱动是显卡厂商提供的底层实现。
- 应用程序通常不直接面对驱动，而是通过 `Vulkan / Metal / OpenGL / D3D` 这类图形 API 间接访问驱动。

一句话总结：

`GPU 驱动` 是“真正和硬件打交道的那一层”。

## 4. 它们之间的关系

### 4.1 Skia 和 wgpu 不是同一层

- `Skia` 是高层 2D 图形引擎。
- `wgpu` 是低层 GPU 抽象层。
- 两者不在同一层竞争。

一句话总结：

`Skia` 负责“怎么画”，`wgpu` 负责“怎么更统一地用 GPU 去执行”。

### 4.2 Skia 和 Vulkan / OpenGL / Metal 的关系

- `Skia` 通常运行在这些 backend 之上。
- 上层用 `Skia` 画图。
- 下层由 `Vulkan / OpenGL / Metal / software` 等后端真正执行并呈现。

一句话总结：

`Skia + Vulkan/OpenGL/Metal` 是一条成熟的图形渲染路线。

### 4.3 wgpu 和 WebGPU 的关系

- `WebGPU` 是标准，是浏览器世界里的 GPU API。
- `wgpu` 是 Rust 世界里的实现和抽象。
- 在浏览器里，`wgpu` 通常通过 `WebGPU` 工作。
- 在原生平台，`wgpu` 再落到 `Vulkan / Metal / D3D12` 等后端。

一句话总结：

不是 `WebGPU` 是 `wgpu` 的实现，而是 `wgpu` 在浏览器里通常通过 `WebGPU` 工作。

## 5. 为什么有了 Skia 还会有 wgpu

- `Skia` 已经把文本排版、阴影、渐变、路径、图片等高层能力做好了。
- 如果目标只是成熟的 2D/UI 渲染，`Skia` 往往更省事。
- `wgpu` 适合的是另一类需求：
  - 自己控制 GPU 管线
  - 写 shader
  - 做 compute
  - 做 2D/3D 混合渲染
  - 自己实现 renderer

一句话总结：

`Skia` 适合“把 UI 和 2D 内容画出来”，`wgpu` 适合“自己掌控 GPU 渲染系统”。

## 6. 为什么用 wgpu 往往更麻烦

- 因为 `wgpu` 不直接提供高层 2D 绘图能力。
- 如果你选 `wgpu`，往往还要在它上面再补一层：
  - 文本渲染
  - 路径绘制
  - 抗锯齿
  - 阴影
  - 渐变
  - scene graph

这就是为什么会出现类似 `Vello` 这样的项目：

- 底下用 `wgpu`
- 上面补一层更高层的 2D renderer

一句话总结：

`wgpu` 更像地基，不像现成的成品楼。

## 7. 为什么浏览器不用直接暴露 Vulkan / Metal / OpenGL

- 浏览器运行的是不可信网页代码，必须满足强沙箱和安全隔离要求。
- 浏览器要跨平台运行，不能要求网页分别适配 `Metal`、`Vulkan`、`D3D12`。
- 浏览器需要一套更适合 Web 平台的 API 设计，包括生命周期、权限、异步模型和安全模型。
- `WebGL` 已经是上一代方案，`WebGPU` 是更现代的新方案。

一句话总结：

浏览器不是“闲着没事再造轮子”，而是必须有一套适合 Web 安全模型和跨平台需求的 GPU API，这就是 `WebGPU`。

## 8. 为什么底层会有 Vulkan / OpenGL / Metal 等多套 API

- 历史演进不同
- 平台厂商各自维护自己的图形路线
- 新旧图形模型差异很大
- 平台控制权和生态利益不同

现实中的解法不是“底层只保留一个 API”，而是：

- 底层继续多套 API 并存
- 上层用 `Skia`、`wgpu`、`ANGLE`、`WebGPU` 等方式去做抽象和兼容

一句话总结：

底层很难真正统一，所以行业的常见方案是“底层不统一，上层尽量统一”。

## 9. 常见平台图形栈

### 9.1 Android

常见图形栈可以概括为：

- 底层 GPU API：`Vulkan / OpenGL ES`
- 系统 UI 2D 渲染：`Skia / HWUI`
- 系统显示合成：`SurfaceFlinger`

一句话总结：

Android 常见图形栈是：`Vulkan/OpenGL ES + Skia/HWUI + SurfaceFlinger`

### 9.2 iOS

常见图形栈可以概括为：

- 底层 GPU API：`Metal`
- 2D 绘制：`Core Graphics / Quartz 2D`
- UI 合成与动画：`Core Animation`

一句话总结：

iOS 常见图形栈是：`Metal + Core Graphics + Core Animation`

### 9.3 macOS

常见图形栈可以概括为：

- 底层 GPU API：`Metal`
- 2D 绘制：`Core Graphics / Quartz 2D`
- UI 合成与动画：`Core Animation`

一句话总结：

macOS 常见图形栈是：`Metal + Core Graphics + Core Animation`

### 9.4 Windows

常见图形栈可以概括为：

- 底层 GPU API：`Direct3D`
- 2D 绘制：`Direct2D`
- 桌面合成：`DWM / DirectComposition`

一句话总结：

Windows 常见图形栈是：`Direct3D + Direct2D + DWM/DirectComposition`

## 10. 最终总括

如果把这些概念压缩成最短版本，可以记成下面几句：

- `Skia` 是成熟的 2D 图形引擎，负责高层绘制能力。
- `wgpu` 是跨平台 GPU 抽象层，负责统一控制不同底层 GPU API。
- `Vulkan / OpenGL / Metal / WebGPU` 是底层 GPU API，不是驱动，也不是 UI 框架。
- `WebGPU` 是 Web 世界里的现代 GPU API 标准。
- `wgpu` 在浏览器里通常通过 `WebGPU` 工作，在原生平台则落到 `Vulkan / Metal / D3D12` 等。
- `Skia` 路线更适合成熟 2D/UI 绘制，`wgpu` 路线更适合自己掌控渲染系统。

一句话版最终总结：

`Skia` 负责“把东西画好”，`wgpu` 负责“更统一地用 GPU”，`Vulkan/OpenGL/Metal/WebGPU` 负责“提供底层图形能力接口”。
