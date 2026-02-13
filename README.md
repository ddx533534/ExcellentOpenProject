# ExcellentOpenProject

## 1.[resvg](https://github.com/linebender/resvg)
### 1.核心定位：一个快速、小巧、可移植的 SVG 渲染库，目标是支持完整的 SVG 规范。

### 2.主要特性
- 为边缘情况设计。<br/>
   测试套件独立开放，可供其他 SVG 库使用;比浏览器更正确地处理静态 SVG（浏览器侧重动态特性）。
- 安全性。完全用 Rust 编写，二进制文件中无非 Rust 代码，几乎无 unsafe 代码；安全处理多种格式：SVG/XML、CSS、TTF、PNG、JPEG、GIF、GZIP。
-  零膨胀 📦<br/>
CLI 应用小于 3MB，无外部依赖，仅包含 SVG 渲染所需代码。
- 可移植性 🌍<br/>
   支持所有 Rust 可编译的平台，包括 WASM；在不同架构上表现一致。
- SVG 预处理 🔄<br/>
   解析和渲染完全分离：usvg：SVG 解析库 ；resvg：渲染库。可基于 usvg 使用任何 2D 库自定义渲染器。
- 性能 ⚡<br/>
   用 Rust 编写 ，使用 tiny-skia 进行渲染 ；速度快，但仍有优化空间。
- 可重现性 🔁<br/>
   **跨平台像素级一致的渲染结果。Windows x86 和 macOS ARM 渲染结果完全相同，**

### 3.库内容
* resvg – 实际的 SVG 渲染器，负责渲染 SVG 图像。
* usvg – SVG 预处理器和简化器，优化 SVG 文件以便渲染。
* tiny-skia – 一个将 Skia（图形库）子集移植到 Rust 的库，用于渲染矢量图形。
* rustybuzz – 将 HarfBuzz（文本排版引擎）移植到 Rust 的子集，负责文本布局。
* ttf-parser – 用于解析 TrueType 和 OpenType 字体的库。
* fontdb – 一个简单的内存中字体数据库，支持类似 CSS 的查询来查找字体。
* roxmltree – 一个用于处理 SVG XML 数据的 XML 解析库。
* simplecss – 一个 CSS 2 解析器，处理 CSS 样式规则和选择器。
* pico-args – 一个极简的命令行参数解析库，广受欢迎。

**虽然 resvg 本身非常简洁，但依赖的这些支持库使它能够处理字体渲染、CSS 解析、SVG 优化等多种任务。即便整体代码量较大，它仍然被认为是目前最小且高效的 SVG 渲染库之一。**
[使用示例](./rust/tests/resvg_test.rs)


## 2.[thorvg](https://github.com/thorvg/thorvg)

### 1.📊 项目概览：ThorVG

ThorVG 是一个生产就绪的矢量图形渲染引擎，专为创建交互式应用和创意工具而设计。

### 2.🎯 核心功能

这是一个跨平台的矢量图形库，主要特点包括：

1. 轻量级设计 - 核心库仅约 150KB
2. 高性能 CPU 光栅化 - CPU渲染性能比常见矢量引擎平均快 1.8 倍
3. 广泛的平台支持 - 支持 Web、桌面(Windows/macOS/Linux)、移动(Android/iOS)、嵌入式系统、微控制器(ESP32)等


### 3.📦 支持的图形元素

- 基本图形：线条、矩形、圆形、路径
- 填充：纯色、线性/径向渐变、路径裁剪
- 描边：宽度、连接、端点、虚线模式
- 场景管理：场景图、对象变换
- 合成：混合和遮罩
- 文本：Unicode 字符、TTF 字体
- 图像：SVG、JPG、PNG、WebP、位图
- 特效：模糊、阴影、填充、色调替换
- 动画：Lottie 动画支持

### 4.🏗️ 项目结构

thorvg/
├── inc/# 公共头文件 (API)
│   └── thorvg.h     # 主 C++ API
├── src/              # 源代码
│   ├── renderer/    # 渲染引擎实现
│   ├── loaders/     # 各种格式加载器 (SVG, Lottie, PNG, JPG, etc.)
│   ├── savers/      # 保存器 (GIF等)
│   ├── bindings/    # API绑定 (C API)
│   └── common/      # 通用代码
├── test/            # 测试代码
├── tools/           # 命令行工具
└── meson.build      # 构建配置

### 5.🚀 项目入口点(Entrypoints)

1. API 入口 - inc/thorvg.h

这是主要的 C++ API 头文件，包含所有公共接口：
- tvg::Initializer::init() - 初始化引擎
- tvg::SwCanvas::gen() - 创建软件渲染画布
- tvg::Shape::gen() - 创建图形
- tvg::Picture::gen() - 加载图片/SVG/Lottie
- tvg::Animation::gen() - 创建动画

2. 渲染引擎入口

- src/renderer/tvgCanvas.cpp - 画布实现
- src/renderer/tvgRender.cpp - 渲染器基类
- src/renderer/sw_engine/ - 软件渲染引擎
- src/renderer/gl_engine/ - OpenGL 渲染引擎
- src/renderer/wg_engine/ - WebGPU 渲染引擎

3. 命令行工具入口 (在 tools/ 目录)

- tvg-svg2png - SVG 转 PNG 转换器
- tvg-lottie2gif - Lottie 转 GIF 转换器

4. 测试入口 - test/testMain.cpp

单元测试的主入口文件

### 6.🔧 构建系统

项目使用 Meson 构建系统：

```shell
meson setup builddir
```

编译安装
```shell
ninja -C builddir install
```
使用示例
```c++
// 1. 初始化引擎
tvg::Initializer::init(4);// 4个线程

// 2. 创建画布
auto canvas = tvg::SwCanvas::gen();
canvas->target(buffer, WIDTH, WIDTH, HEIGHT, tvg::ColorSpace::ARGB8888);

// 3. 创建图形并添加到画布
auto rect = tvg::Shape::gen();
rect->appendRect(50, 50, 200, 200, 20, 20);
rect->fill(100, 100, 100);
canvas->add(rect);

// 4. 渲染
canvas->draw();
canvas->sync();

// 5. 清理
tvg::Initializer::term();
```
### 7.🌟 实际应用

ThorVG 已被集成到多个知名项目中：
- Canva iOS - 渲染速度提升 80%，内存使用降低 70%
- Godot 游戏引擎 - UI矢量图形渲染
- LVGL - 嵌入式系统 GUI 框架
- Espressif - ESP32 微控制器官方组件
- Lottie Creator - Lottie 动画创作工具

这是一个非常专业和成熟的矢量图形渲染库项目！🎨