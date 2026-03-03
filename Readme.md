## 卫星电话功能简介

### 实时采集音频格式
- 通道：单通道
- 采样率：8k/sec
- 采样精度: 16bit
- 采样间隔：未知，普遍间隔，1帧20ms，即160i16/frame
- 编码格式：原始pcm

### 功能分解 
1. 编码压缩：pcm->aac adts, 压缩率可调(0.2~0.1), 延迟120ms
2. 语音增强: gtcrn，格式onnx，延迟估计40ms以上
3. 人声分离：silero-vad，格式onnx，延迟估计40ms

### 功能流程
1. 重采样到16k, 320i16/frame（VAD模型只支持16k）
2. 人声检测，VAD识别speech的起始终止时间段
3. 语音增强，将时间段内人声降噪
4. 编码压缩后发送

## 风险
1. ARM性能支持不足，语音增强和人声分离模型运算量大，影响通话质量
2. 算法延迟累计到200ms，用户能否接受。业界实时语音通话延迟要求在40ms内，双方无差异感受
3. 人声检测根据产品适配，人声被识别到多个分段，分段大小的根据实际情况调节(见下面运行结果)
4. 工程参数调节，3种子功能都有配置参数，即使使用配置文件可变，调试通话质量花费时间估计困难
5. 工程产品适配，capi调用、交叉编译及链接问题，只能以我手中的RK3568+ubuntu20为验证
6. 开发代码只使用rust，使用此语言的开发人员少

### 代替方案
1. 使用古典算法实现语音增强和人声分离，不使用模型，性能和延迟可控在40ms，但通话质量提高不如模型好

### 特殊说明
1. 发来测试用的多个wav都有损坏(文件头中标识的长度小于文件实际长度)，修复wav使用fix_wav_length.py
2. 语音增强古典算法库只支持linux
3. demo中将全部时长编码压缩到aac文件，没有只取vad识别的人声段(需要根据业务调整)，原因是所有发音连在一起，中间没有间隙，听不懂。

## 运行结果
```
# 编码压缩，压缩了7倍
5/30/2025  14:45  4.33kB 1先人声后留白（数字）.wav
5/30/2025  15:12   644kB 1先人声后留白（数字）.wav.aac


# 识别出每一次发音的数字的起始时间和发音时常
.\audioalgo.exe .\先人声后留白（数字）.wav
先人声后留白（数字）.wav duration: 276s
start=1.254s duration=0.598s
start=4.614s duration=0.47s
start=5.766s duration=0.79s
start=6.822s duration=0.854s
start=8.006s duration=0.854s
start=10.214s duration=0.758s
start=11.366s duration=0.694s
start=12.454s duration=0.854s
start=14.726s duration=0.758s
start=15.814s duration=0.95s
start=17.094s duration=0.95s
start=18.214s duration=0.854s
start=19.206s duration=0.886s
start=21.382s duration=0.758s
start=22.63s duration=2.198s
start=25.67s duration=0.662s
start=26.854s duration=0.694s
start=27.942s duration=0.502s
start=28.838s duration=0.694s
start=29.798s duration=1.686s
start=31.718s duration=3.414s
start=36.934s duration=0.982s
start=38.022s duration=0.694s
start=40.326s duration=1.238s
start=42.342s duration=0.726s
start=43.27s duration=1.078s
start=44.422s duration=1.846s
start=46.63s duration=0.982s
start=52.87s duration=8.662s
start=61.83s duration=8.47s
start=70.374s duration=8.054s
start=78.502s duration=8.054s
start=86.63s duration=1.654s
start=90.534s duration=0.566s
start=93.606s duration=8.374s
start=102.182s duration=3.094s
start=107.974s duration=1.43s
start=110.566s duration=6.422s
start=117.83s duration=8.054s
start=128.966s duration=1.046s
start=132.39s duration=0.63s
start=135.686s duration=0.63s
start=140.038s duration=3.094s
start=143.302s duration=0.79s
start=145.126s duration=3.414s
start=148.71s duration=2.102s
```