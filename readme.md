rustc1.82(stable-x86_64-unknown-linux-gnu)

## 项目结构
- crates/physics_engine_core 核心库, 支持导出wasm  
- crates/physics_engine_bevy 包装为bevy插件 
- examples/basic 基础示例(即distance joint demo)
- examples/editor 一个简单的编辑器 使用教程在对应文件夹的readme里
- examples/string cuboid组成一条绳子的demo
- examples/three_js physics_engine_core导出wasm后由js端调用  

## 参考资料
主要是参考了游戏开发物理学一书

## 理解
基础demo部分和大学物理讲授的内容还是很接近的，只是课堂上没有引入四元数只使用欧拉角，然后也不会考虑精度问题  
碰撞检测部分只针对长方体的话也有比较多的资料可以查到，但是碰撞响应的部分我根据查到的资料目前还调不出来一个表现特别好的  

## 并行结算
bevy 的query提供了parallel iterator的接口可以自动在线程池内完成计算

## 持续集成
github提供github action服务，可以在提交和收到pr时自动执行cargo clippy, cargo test来检查代码